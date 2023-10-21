use constants::MEDIA_RELAY_TIMEOUT_SECS;
use db::deployment::Deployment;
use db::station::OwnerDeploymentInfo;
use db::Model;
use drop_tracer::{DropTracer, Token};
use futures_util::stream::TryStreamExt;
use hyper::{Body, StatusCode};
use log::*;
use mongodb::bson::doc;
use serde_util::DateTime;
use shutdown::Shutdown;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::task::JoinHandle;

use crate::channel::{SendError, Sender};

#[derive(Debug, thiserror::Error)]
pub enum GetInternalRelayError {
  #[error("db: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("deployment not found: {0}")]
  DeploymentNotFound(String),
  #[error("deployment no port")]
  DeploymentNoPort,
  #[error("craete request: {0}")]
  CreateRequest(hyper::http::Error),
  #[error("send request: {0}")]
  SendRequest(hyper::Error),
  #[error("relay timeout")]
  RelayTimeout,
  #[error("relay status: {0:?}")]
  RelayStatus(StatusCode),
}

pub async fn get_internal_relay_source(
  sender: Sender,
  deployment_id: String,
  _task_id: String,
  station_id: String,
  info: OwnerDeploymentInfo,
  drop_tracer: DropTracer,
  shutdown: Shutdown,
) -> Result<JoinHandle<()>, GetInternalRelayError> {
  let station_id2 = station_id.clone();

  let r = async move {
    let deployment = match Deployment::get_by_id(&info.deployment_id).await? {
      None => {
        return Err(GetInternalRelayError::DeploymentNotFound(
          info.deployment_id,
        ))
      }
      Some(doc) => doc,
    };

    use rand::seq::SliceRandom;
    let stream_port = deployment.stream_ports.choose(&mut rand::thread_rng());

    let port = match stream_port {
      None => return Err(GetInternalRelayError::DeploymentNoPort),
      Some(port) => *port,
    };

    let destination = SocketAddr::from((deployment.local_ip, port));

    let client = hyper::Client::default();

    let url = format!(
      "http://{}:{}/relay/{}",
      destination.ip(),
      destination.port(),
      station_id
    );

    let hyper_req = hyper::Request::builder()
      .uri(url)
      .header("connection", "close")
      .header(constants::HEADER_RELAY_SOURCE_DEPLOYMENT, &deployment_id);

    let hyper_req = match hyper_req.body(Body::empty()) {
      Ok(req) => req,
      Err(e) => return Err(GetInternalRelayError::CreateRequest(e)),
    };

    let hyper_res = tokio::select! {
      _ = tokio::time::sleep(tokio::time::Duration::from_secs(MEDIA_RELAY_TIMEOUT_SECS)) => {
        return Err(GetInternalRelayError::RelayTimeout)
      }

      res = client.request(hyper_req) => match res {
        Err(e) => return Err(GetInternalRelayError::SendRequest(e)),
        Ok(res) => res,
      }
    };

    if !hyper_res.status().is_success() {
      return Err(GetInternalRelayError::RelayStatus(hyper_res.status()));
    }

    let relay_session_doc = {
      use db::relay_session::*;
      let now = DateTime::now();
      let relay_session_doc = RelaySession {
        id: RelaySession::uid(),
        station_id: station_id.clone(),
        deployment_id,
        target_deployment_id: info.deployment_id.clone(),
        state: RelaySessionState::Open,
        transfer_bytes: 0,
        closed_at: None,
        duration_ms: None,
        created_at: now,
        updated_at: now,
      };

      RelaySession::insert(&relay_session_doc).await?;
      relay_session_doc
    };

    let dropper = RelaySessionDropper {
      id: relay_session_doc.id,
      station_id: station_id.clone(),
      start: Instant::now(),
      transfer: Arc::new(AtomicU64::new(0)),
      token: Some(drop_tracer.token()),
    };

    let mut no_listeners_since: Option<Instant> = None;

    let mut transfer = 0u64;

    let mut body = hyper_res.into_body();

    let spawn = tokio::spawn(async move {
      let signal = shutdown.signal();
      let task = async move {
        'root: loop {
          match body.try_next().await {
            Err(e) => {
              warn!(
                target: "media",
                "internal-relay media session for station {} request stream error: {} => {:?}",
                station_id,
                e,
                e
              );
              break 'root;
            }

            Ok(None) => {
              warn!(
                target: "media",
                "internal-relay media session for station {} request stream end",
                station_id
              );
              break 'root;
            }

            Ok(Some(bytes)) => {
              transfer += bytes.len() as u64;
              dropper.transfer.store(transfer, Ordering::SeqCst);
              match sender.send(bytes) {
                Ok(_) => {
                  no_listeners_since = None;
                  continue 'root;
                }

                Err(SendError::NoSubscribers(_)) => {
                  if let Some(since) = no_listeners_since {
                    if since.elapsed().as_secs() > constants::RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS
                    {
                      break 'root;
                    }
                  } else {
                    no_listeners_since = Some(Instant::now());
                    continue 'root;
                  }
                }

                // here the stream has been terminated (maybe replaced with a newer transmitter)
                Err(SendError::Terminated(_)) => break 'root,
              }
            }
          }
        }

        drop(dropper);
      };

      tokio::select! {
        _ = signal => {},
        _ = task => {}
      }
    });

    Ok(spawn)
  }
  .await;

  if let Err(e) = &r {
    warn!(
      target: "media",
      "internal-relay media session for station {} error: {} => {:?}",
      station_id2,
      e,
      e
    );
  };

  r
}

#[derive(Debug, thiserror::Error)]
pub enum RelaySessionError {
  #[error("db: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("hyper: {0}")]
  Hyper(#[from] hyper::Error),
  #[error("hyper http: {0}")]
  HyperHttp(#[from] hyper::http::Error),
}

#[derive(Debug)]
struct RelaySessionDropper {
  id: String,
  station_id: String,
  start: Instant,
  transfer: Arc<AtomicU64>,
  token: Option<Token>,
}

impl Drop for RelaySessionDropper {
  fn drop(&mut self) {
    use db::relay_session::*;

    let token = match self.token.take() {
      None => return,
      Some(token) => token,
    };

    let id = self.id.clone();
    let station_id = self.station_id.clone();

    let now = DateTime::now();

    let duration_ms = self.start.elapsed().as_millis();

    let transfer_bytes = self.transfer.load(Ordering::SeqCst);

    let update = doc! {
      "$set": {
        RelaySession::KEY_UPDATED_AT: Some(now),
        RelaySession::KEY_CLOSED_AT: Some(now),
        RelaySession::KEY_STATE: RelaySessionState::KEY_ENUM_VARIANT_CLOSED,
        RelaySession::KEY_DURATION_MS: Some(duration_ms as f64),
        RelaySession::KEY_TRANSFER_BYTES: transfer_bytes as f64,
      }
    };

    tokio::spawn(async move {
      info!("saving relay session {} station_id={}", id, station_id,);

      match RelaySession::update_by_id(&id, update).await {
        Err(e) => warn!(
          "error saving relay session {} for station {}: {} => {:?}",
          id, station_id, e, e
        ),
        Ok(r) => {
          if r.matched_count != 1 {
            warn!(
              "relay session save id={} station_id={} returned matched count != 1 ({})",
              id, station_id, r.matched_count
            )
          }
        }
      }

      drop(token)
    });
  }
}

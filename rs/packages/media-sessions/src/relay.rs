use db::Model;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use constants::RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS;
use drop_tracer::{DropTracer, Token};
use log::*;

use serde_util::DateTime;

use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;

use shutdown::Shutdown;
use std::sync::Arc;

use crate::{SendError, Transmitter};

#[derive(Debug, thiserror::Error)]
pub enum RelaySessionError {
  #[error("db: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("hyper: {0}")]
  Hyper(#[from] hyper::Error),
  #[error("hyper http: {0}")]
  HyperHttp(#[from] hyper::http::Error),
}

pub fn run_relay_session(
  tx: Transmitter,
  deployment_id: String,
  target_deployment_id: String,
  response: hyper::Response<hyper::Body>,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
) -> tokio::task::JoinHandle<Result<(), RelaySessionError>> {
  tokio::spawn(async move {
    let station_id = tx.info.station_id.clone();

    let result = async {
      info!("media session (relay) start for station {}", station_id,);

      let relay_session_doc = {
        use db::relay_session::*;
        let now = DateTime::now();
        let relay_session_doc = RelaySession {
          id: RelaySession::uid(),
          station_id: station_id.clone(),
          deployment_id,
          target_deployment_id,
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

      let mut body = response.into_body();
      // fill the burst without delay between chunk parts
      // tokio::pin!(stream);

      'root: loop {
        if shutdown.is_closed() || tx.is_terminated() {
          break 'root;
        }

        match body.try_next().await {
          Err(e) => {
            warn!("relay media session request stream error: {} => {:?}", e, e);
            break 'root;
          }

          Ok(None) => break 'root,
          Ok(Some(bytes)) => {
            transfer += bytes.len() as u64;
            dropper.transfer.store(transfer, Ordering::SeqCst);
            match tx.send(bytes) {
              Ok(_) => {
                no_listeners_since = None;
                continue 'root;
              }

              Err(SendError::NoListeners(_)) => {
                if let Some(since) = no_listeners_since {
                  if since.elapsed().as_secs() > RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS {
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

      Ok(())
    }
    .await;

    if let Err(ref e) = result {
      warn!("media session for station {station_id} error: {e} => {e:?}");
    }

    result
  })
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

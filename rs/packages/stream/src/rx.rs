use std::net::SocketAddr;

use db::{
  account::Account,
  audio_file::AudioFile,
  deployment::Deployment,
  station::{OwnerDeploymentInfo, Station},
  Model,
};
use drop_tracer::DropTracer;
use hyper::{Body, StatusCode};
use media_sessions::{
  external_relay::run_external_relay_session, playlist::run_playlist_session,
  relay::run_relay_session, Listener, MediaSessionMap,
};
use mongodb::bson::doc;
use serde_util::DateTime;
use shutdown::Shutdown;

use crate::X_OPENSTREAM_RELAY_CODE;

#[derive(Debug)]
pub enum GetRxMode {
  Stream {
    local_addr: SocketAddr,
    remote_addr: SocketAddr,
  },
  Relay,
}

#[derive(Debug, thiserror::Error)]
pub enum GetRxError {
  #[error("relay db: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("rx station not streaming: {0}")]
  StationNotStreaming(String),
  #[error("rx station streaming from other server: {0}")]
  StationStreamingFromOtherServer(String),
  #[error("rx station not found: {0}")]
  StationNotFound(String),
  #[error("rx account not found: {0}")]
  AccountNotFound(String),
  #[error("rx owner deployment not found: {0}")]
  DeploymentNotFound(String),
  #[error("rx deployment no port")]
  DeploymentNoPort,
  #[error("rx transfer limit")]
  TransferLimit,
  #[error("rx listeners limit")]
  ListenersLimit,
  #[error("rx relay create request: {0}")]
  RelayCreateRequest(hyper::http::Error),
  #[error("rx relay send request: {0}")]
  RelaySendRequest(hyper::Error),
  #[error("rx relay status: {:?}", 0)]
  RelayStatus(StatusCode),
}

pub async fn get_rx(
  mode: GetRxMode,
  deployment_id: &str,
  station_id: &str,
  media_sessions: &MediaSessionMap,
  drop_tracer: &DropTracer,
  shutdown: &Shutdown,
) -> Result<(Listener, Station), GetRxError> {
  let task_id = Station::random_owner_task_id();
  let info = OwnerDeploymentInfo {
    deployment_id: deployment_id.to_string(),
    task_id: task_id.clone(),
    content_type: String::from("audio/mpeg"),
    health_checked_at: Some(DateTime::now()),
  };

  let is_relay = matches!(mode, GetRxMode::Relay);
  let check_account_limits = !is_relay;

  let (rx, station) = 'rx: {
    let (station, dropper) = 'station: {
      match Station::try_set_owner_deployment_info(station_id, info, drop_tracer.token()).await? {
        Ok((station, dropper)) => {
          break 'station (station, Some(dropper));
        }

        Err(None) => {
          return Err(GetRxError::StationNotFound(station_id.to_string()));
        }

        Err(Some((station, owner_info))) => {
          let (relay_tx, local_addr, remote_addr) = 'relay_tx: {
            let lock = media_sessions.upgradable_read();
            if owner_info.deployment_id == deployment_id {
              break 'station (station, None);
            } else {
              match mode {
                GetRxMode::Relay => {
                  return Err(GetRxError::StationStreamingFromOtherServer(
                    station_id.to_string(),
                  ));
                }

                GetRxMode::Stream {
                  local_addr,
                  remote_addr,
                } => match lock.get(station_id) {
                  None => {
                    let relay_tx = lock.upgrade().transmit(
                      station_id,
                      &task_id,
                      media_sessions::MediaSessionKind::Relay {
                        content_type: owner_info.content_type.clone(),
                      },
                    );

                    break 'relay_tx (relay_tx, local_addr, remote_addr);
                  }

                  Some(session) => {
                    if session.info().kind().is_relay() {
                      break 'station (station, None);
                    } else {
                      let relay_tx = lock.upgrade().transmit(
                        station_id,
                        &task_id,
                        media_sessions::MediaSessionKind::Relay {
                          content_type: owner_info.content_type.clone(),
                        },
                      );

                      break 'relay_tx (relay_tx, local_addr, remote_addr);
                    }
                  }
                },
              }
            }
          };

          let account = match Account::get_by_id(&station.account_id).await? {
            Some(account) => account,
            None => return Err(GetRxError::AccountNotFound(station.account_id)),
          };

          if account.limits.transfer.avail() == 0 {
            return Err(GetRxError::TransferLimit);
          }

          if account.limits.listeners.avail() == 0 {
            return Err(GetRxError::ListenersLimit);
          }

          let deployment = match Deployment::get_by_id(&owner_info.deployment_id).await? {
            None => return Err(GetRxError::DeploymentNotFound(owner_info.deployment_id)),
            Some(doc) => doc,
          };

          use rand::seq::SliceRandom;
          let stream_port = deployment.stream_ports.choose(&mut rand::thread_rng());

          let port = match stream_port {
            None => return Err(GetRxError::DeploymentNoPort),
            Some(port) => *port,
          };

          let destination = SocketAddr::from((deployment.local_ip, port));

          let client = hyper::Client::default();

          let mut hyper_req = hyper::Request::builder().uri(format!(
            "http://{}:{}/relay/{}",
            destination.ip(),
            destination.port(),
            station_id
          ));

          hyper_req = hyper_req
            .header(X_OPENSTREAM_RELAY_CODE, &owner_info.deployment_id)
            .header("x-openstream-relay-remote-addr", format!("{}", remote_addr))
            .header("x-openstream-relay-local-addr", format!("{}", local_addr))
            .header("x-openstream-relay-deployment-id", deployment_id)
            .header("x-openstream-relay-target-deployment-id", deployment_id)
            .header("connection", "close");

          let hyper_req = match hyper_req.body(Body::empty()) {
            Ok(req) => req,
            Err(e) => return Err(GetRxError::RelayCreateRequest(e)),
          };

          match client.request(hyper_req).await {
            Err(e) => return Err(GetRxError::RelaySendRequest(e)),
            Ok(hyper_res) => {
              // if error return the same error to the client
              if !hyper_res.status().is_success() {
                return Err(GetRxError::RelayStatus(hyper_res.status()));
              }

              let rx = relay_tx.subscribe();
              run_relay_session(
                relay_tx,
                deployment_id.to_string(),
                owner_info.deployment_id,
                hyper_res,
                shutdown.clone(),
                drop_tracer.clone(),
              );

              break 'rx (rx, station);
            }
          }
        }
      }
    };

    if check_account_limits {
      let account = match Account::get_by_id(&station.account_id).await? {
        Some(account) => account,
        None => return Err(GetRxError::AccountNotFound(station.account_id)),
      };

      if account.limits.transfer.avail() == 0 {
        return Err(GetRxError::TransferLimit);
      }

      if account.limits.listeners.avail() == 0 {
        return Err(GetRxError::ListenersLimit);
      }
    }

    #[allow(clippy::collapsible_if)]
    if media_sessions.read().get(station_id).is_none() {
      match &station.external_relay_url {
        None => {
          if !AudioFile::exists(doc! { AudioFile::KEY_STATION_ID: &station.id }).await? {
            return Err(GetRxError::StationNotStreaming(station.id));
          }
        }

        Some(_) => {}
      }
    };

    let rx = {
      let lock = media_sessions.upgradable_read();

      match lock.get(station_id) {
        Some(session) => session.subscribe(),

        None => {
          let mut lock = lock.upgrade();
          match &station.external_relay_url {
            None => {
              let tx = lock.transmit(
                station_id,
                &task_id,
                media_sessions::MediaSessionKind::Playlist {},
              );
              let rx = tx.subscribe();
              let shutdown = shutdown.clone();
              let deployment_id = deployment_id.to_string();
              let drop_tracer = drop_tracer.clone();
              tokio::spawn(async move {
                let _ = run_playlist_session(tx, deployment_id, shutdown, drop_tracer, true)
                  .await
                  .unwrap();
                drop(dropper);
              });
              rx
            }

            Some(url) => {
              let tx = lock.transmit(
                station_id,
                &task_id,
                media_sessions::MediaSessionKind::ExternalRelay,
              );
              let rx = tx.subscribe();
              let shutdown = shutdown.clone();
              let deployment_id = deployment_id.to_string();
              let drop_tracer = drop_tracer.clone();
              let url = url.clone();

              tokio::spawn(async move {
                let _ = run_external_relay_session(tx, deployment_id, url, shutdown, drop_tracer)
                  .await
                  .unwrap();
                drop(dropper);
              });
              rx
            }
          }
        }
      }
    };

    (rx, station)
  };

  Ok((rx, station))
}

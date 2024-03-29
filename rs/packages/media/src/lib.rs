pub mod channel;
pub mod drop;
pub mod handle;
pub mod health;

use constants::MEDIA_LOCK_TIMEOUT_SECS;
use db::{
  audio_file::AudioFile,
  probe::{Probe, ProbeResult},
  run_transaction,
  station::{OwnerDeploymentInfo, Station},
  Model,
};
use drop::{MapEntryRelease, OwnerDeploymentDropper};
use drop_tracer::DropTracer;
use mongodb::bson::doc;
use mongodb::options::FindOneAndUpdateOptions;
use parking_lot::Mutex;
use serde_util::DateTime;
use shutdown::Shutdown;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::{Mutex as AsyncMutex, OwnedMutexGuard};

use channel::{Receiver, Sender};
use handle::internal_relay::GetInternalRelayError;
use handle::{get_internal_relay_source, run_external_relay_source, run_playlist_source};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProbeCodec {
  Mp3,
  Aac,
}

#[derive(Debug)]
pub struct Handle {
  sender: Sender,
}

impl Drop for Handle {
  fn drop(&mut self) {
    self.terminate();
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Kind {
  Live,
  Playlist,
  ExternalRelay,
  InternalRelay,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
  pub task_id: String,
  pub kind: Kind,
  pub content_type: String,
}

impl Info {
  pub fn new(kind: Kind, task_id: String, content_type: String) -> Self {
    Self {
      kind,
      task_id,
      content_type,
    }
  }
}

impl Handle {
  #[inline(always)]
  pub fn new(sender: Sender) -> Self {
    Self { sender }
  }

  #[allow(clippy::bool_comparison)]
  /// returns true if the handle was not terminated before.
  /// otherwise returns false.
  #[inline(always)]
  pub fn terminate(&self) -> bool {
    self.sender.terminated.swap(true, Ordering::SeqCst) == false
  }

  #[inline(always)]
  pub fn is_terminated(&self) -> bool {
    self.sender.terminated.load(Ordering::SeqCst)
  }

  #[inline(always)]
  pub fn info(&self) -> &Info {
    &self.sender.info
  }
}

type Map = HashMap<String, Arc<AsyncMutex<Option<Handle>>>>;

#[derive(Debug, Clone)]
pub struct MediaSessionMap {
  deployment_id: String,
  drop_tracer: DropTracer,
  shutdown: Shutdown,
  map: Arc<Mutex<Map>>,
}

#[derive(Debug, thiserror::Error)]
pub enum SubscribeError {
  #[error("db: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("station not found: {0}")]
  StationNotFound(String),
  #[error("playlist is empty")]
  PlaylistEmpty,
  #[error("internal relay error: {0}")]
  InternalRelay(#[from] GetInternalRelayError),
  #[error("external relay redirect: {0}")]
  ExternalRelayRedirect(String),
}

impl MediaSessionMap {
  pub fn new(deployment_id: String, drop_tracer: DropTracer, shutdown: Shutdown) -> Self {
    Self {
      deployment_id,
      drop_tracer,
      shutdown,
      map: Arc::new(Mutex::new(Map::new())),
    }
  }

  pub async fn lock(&self, station_id: &str) -> OwnedMutexGuard<Option<Handle>> {
    let item = {
      let mut lock = self.map.lock();
      let item = lock
        .entry(station_id.to_string())
        .or_insert_with(|| Arc::new(AsyncMutex::new(None)));
      item.clone()
    };

    let timeout = tokio::time::Duration::from_secs(MEDIA_LOCK_TIMEOUT_SECS);
    tokio::time::timeout(timeout, item.lock_owned())
      .await
      .unwrap_or_else(|_| {
        panic!("media lock timeout elapsed for station {station_id} on call to lock()")
      })
  }

  pub async fn terminate(&self, station_id: &str) -> Option<Handle> {
    let entry = { self.map.lock().remove(station_id)? };

    let timeout = tokio::time::Duration::from_secs(MEDIA_LOCK_TIMEOUT_SECS);
    let handle = tokio::time::timeout(timeout, entry.lock())
      .await
      .unwrap_or_else(|_| {
        panic!("media lock timeout elapsed for station {station_id} on call to terminate()")
      })
      .take()?;

    Some(handle)
  }

  pub async fn terminate_task(&self, station_id: &str, task_id: &str) -> Option<Handle> {
    let entry = {
      let lock = self.map.lock();
      lock.get(station_id)?.clone()
    };

    let timeout = tokio::time::Duration::from_secs(MEDIA_LOCK_TIMEOUT_SECS);
    let mut handle = tokio::time::timeout(timeout, entry.lock())
      .await
      .unwrap_or_else(|_| {
        panic!("media lock timeout elapsed for station {station_id}, task {task_id} on call to terminate_task()")
      });

    match &*handle {
      None => None,
      Some(item) => {
        if item.info().task_id == task_id {
          handle.take()
        } else {
          None
        }
      }
    }
  }

  pub async fn playlist_restart(&self, station_id: &str) -> Result<(), PlaylistRestartError> {
    let mut lock = self.lock(station_id).await;
    // TODO: allow to restart playlist even if it's live
    // so when live transmission end the playlist starts from the top
    match &*lock {
      None => {}
      Some(handle) => match handle.info().kind {
        Kind::Live => return Err(PlaylistRestartError::Live),
        Kind::ExternalRelay => return Err(PlaylistRestartError::ExternalRelay),
        Kind::InternalRelay => return Err(PlaylistRestartError::InternalRelay),
        Kind::Playlist => {}
      },
    };

    let task_id = Station::random_owner_task_id();

    run_transaction!(session => {
      let station = match tx_try!(Station::get_by_id_with_session(station_id, &mut session).await) {
        None => return Err(PlaylistRestartError::NotFound),
        Some(station) => station,
      };

      if let Some(info) = &station.owner_deployment_info {
        if info.deployment_id != self.deployment_id {
          return Err(PlaylistRestartError::DeploymentMismatch);
        }
      }

      let owner_info = OwnerDeploymentInfo {
        content_type: "audio/mpeg".to_string(),
        deployment_id: self.deployment_id.clone(),
        task_id: task_id.clone(),
        health_checked_at: Some(DateTime::now()),
      };

      let update = doc! {
        "$set": { Station::KEY_OWNER_DEPLOYMENT_INFO: owner_info }
      };

      tx_try!(Station::update_by_id_with_session(&station.id, update, &mut session).await);

      let new_sender = Sender::new(
        station_id.to_string(),
        Info::new(
          Kind::Playlist,
          task_id.clone(),
          "audio/mpeg".to_string(),
        ),
      );

      let new_handle = Handle::new(new_sender.clone());
      *lock = Some(new_handle);

      let map_entry_release = MapEntryRelease::new(
        station_id.to_string(),
        task_id.clone(),
        self.clone(),
      );

      let owner_deployment_dropper = OwnerDeploymentDropper::new(
        station_id.to_string(),
        task_id.clone(),
        self.drop_tracer.token(),
      );

      {
        let deployment_id = self.deployment_id.clone();
        let drop_tracer = self.drop_tracer.clone();
        let shutdown = self.shutdown.clone();
        tokio::spawn(async move {
          let _ = run_playlist_source(
            new_sender,
            deployment_id,
            task_id,
            station.id,
            false,
            drop_tracer,
            shutdown
          ).await;
          drop(owner_deployment_dropper);
          drop(map_entry_release);
        });
      };

      Ok(())
    })
  }

  pub async fn subscribe(&self, station_id: &str) -> Result<Receiver, SubscribeError> {
    let mut lock = self.lock(station_id).await;
    match &*lock {
      Some(handle) => Ok(handle.sender.subscribe()),
      None => {
        // external relay redirect
        let station: Station;
        match Station::get_by_id(station_id).await? {
          None => return Err(SubscribeError::StationNotFound(station_id.to_string())),
          Some(s) => {
            station = s;
            if let Some(url) = &station.external_relay_url {
              if station.external_relay_redirect {
                return Err(SubscribeError::ExternalRelayRedirect(url.clone()));
              }
            }
          }
        }

        let probe_result: Option<(ProbeCodec, usize)>;

        if station.external_relay_url.is_some() {
          let filter = doc! {
            ProbeResult::KEY_ENUM_TAG: ProbeResult::KEY_ENUM_VARIANT_OK,
            Probe::KEY_STATION_ID: station_id,
          };

          let sort = doc! {
            Probe::KEY_CREATED_AT: -1
          };

          let options = mongodb::options::FindOneOptions::builder()
            .sort(sort)
            .build();

          let last_probe = Probe::cl().find_one(filter, options).await?;

          match last_probe {
            None => {
              probe_result = None;
            }

            Some(doc) => {
              let mut streams = doc
                .streams()
                .into_iter()
                .filter_map(|(codec, bitrate)| match codec?.as_ref() {
                  "mp3" => Some((ProbeCodec::Mp3, bitrate)),
                  "aac" => Some((ProbeCodec::Aac, bitrate)),
                  _ => None,
                })
                .collect::<Vec<(ProbeCodec, Option<usize>)>>();

              streams.sort_by(|(_, br1), (_, br2)| {
                use std::cmp::Ordering;

                match (br1, br2) {
                  (Some(br1), Some(br2)) => br2.cmp(br1),
                  (Some(_), None) => Ordering::Less,
                  (None, Some(_)) => Ordering::Greater,
                  (None, None) => Ordering::Equal,
                }
              });

              let first = streams.first();

              match first {
                None => probe_result = None,
                Some((codec, br)) => {
                  let bitrate = br.unwrap_or(128_000).max(64_000).min(320_000);
                  probe_result = Some((*codec, bitrate));
                }
              }
            }
          }
        } else {
          probe_result = None;
        }

        let task_id = Station::random_owner_task_id();

        let map_entry_release =
          MapEntryRelease::new(station_id.to_string(), task_id.clone(), self.clone());

        let content_type = match probe_result {
          None => "audio/mpeg".to_string(),
          Some((codec, _)) => match codec {
            ProbeCodec::Mp3 => "audio/mpeg".to_string(),
            ProbeCodec::Aac => "audio/aac".to_string(),
          },
        };

        let owner_deployment_info = OwnerDeploymentInfo {
          content_type: content_type.clone(),
          deployment_id: self.deployment_id.clone(),
          task_id: task_id.clone(),
          health_checked_at: Some(DateTime::now()),
        };

        let options = FindOneAndUpdateOptions::builder()
          .return_document(mongodb::options::ReturnDocument::Before)
          .build();

        let filter = doc! {
          Station::KEY_ID: station_id,
        };

        let update = vec![doc! {
          "$set": {
            Station::KEY_OWNER_DEPLOYMENT_INFO: {
              "$ifNull": [
                const_str::concat!("$", Station::KEY_OWNER_DEPLOYMENT_INFO),
                owner_deployment_info.clone(),
              ]
            }
          }
        }];

        let station = match Station::cl()
          .find_one_and_update(filter, update, options)
          .await?
        {
          Some(station) => station,
          None => return Err(SubscribeError::StationNotFound(station_id.to_string())),
        };

        let sender: Sender;
        let info: Info;
        // this station is the document BEFORE the update
        // that means that if owner_deployment_info is null it was taken by this task
        // otherwise it was already taken by another task
        match &station.owner_deployment_info {
          None => {
            let owner_deployment_dropper = OwnerDeploymentDropper::new(
              station_id.to_string(),
              task_id.clone(),
              self.drop_tracer.token(),
            );

            match station.external_relay_url {
              // 1) external relay
              Some(url) => {
                info = Info::new(Kind::ExternalRelay, task_id.clone(), content_type);
                sender = Sender::new(station_id.to_string(), info);

                {
                  let sender = sender.clone();
                  let deployment_id = self.deployment_id.clone();
                  let task_id = task_id.clone();
                  let station_id = station_id.to_string();
                  let drop_tracer = self.drop_tracer.clone();
                  let shutdown = self.shutdown.clone();
                  let codec_info = probe_result;
                  tokio::spawn(async move {
                    let _ = run_external_relay_source(
                      sender,
                      deployment_id,
                      task_id,
                      station_id,
                      url,
                      codec_info,
                      drop_tracer,
                      shutdown,
                    )
                    .await
                    .unwrap();

                    drop(owner_deployment_dropper);
                    drop(map_entry_release);
                  });
                };
              }

              // 2) playlist
              None => {
                let file_filter = doc! { AudioFile::KEY_STATION_ID: station_id };
                let has_playlist_files = AudioFile::exists(file_filter).await?;
                if !has_playlist_files {
                  return Err(SubscribeError::PlaylistEmpty);
                }

                info = Info::new(Kind::Playlist, task_id.clone(), "audio/mpeg".to_string());
                sender = Sender::new(station_id.to_string(), info);

                {
                  let sender = sender.clone();
                  let deployment_id = self.deployment_id.clone();
                  let task_id = task_id.clone();
                  let station_id = station_id.to_string();
                  let drop_tracer = self.drop_tracer.clone();
                  let shutdown = self.shutdown.clone();

                  tokio::spawn(async move {
                    let _ = run_playlist_source(
                      sender,
                      deployment_id,
                      task_id,
                      station_id,
                      true,
                      drop_tracer,
                      shutdown,
                    )
                    .await;
                    drop(owner_deployment_dropper);
                    drop(map_entry_release);
                  });
                };
              }
            }
          }

          Some(owner_info) => {
            info = Info::new(
              Kind::InternalRelay,
              task_id.clone(),
              owner_info.content_type.clone(),
            );
            sender = Sender::new(station_id.to_string(), info);

            {
              let sender = sender.clone();
              let deployment_id = self.deployment_id.clone();
              let task_id = task_id.clone();
              let station_id = station_id.to_string();
              let drop_tracer = self.drop_tracer.clone();
              let shutdown = self.shutdown.clone();

              let spawn = get_internal_relay_source(
                sender,
                deployment_id,
                task_id,
                station_id,
                owner_info.clone(),
                drop_tracer,
                shutdown,
              )
              .await?;

              tokio::spawn(async move {
                let _ = spawn.await;
                drop(map_entry_release);
              });
            };
          }
        };

        let receiver = sender.subscribe();
        let handle = Handle::new(sender);
        *lock = Some(handle);

        Ok(receiver)
      }
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum PlaylistRestartError {
  #[error("db: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("station not found")]
  NotFound,
  #[error("live")]
  Live,
  #[error("external relay")]
  ExternalRelay,
  #[error("internal relay")]
  InternalRelay,
  #[error("deployment mismatch")]
  DeploymentMismatch,
}

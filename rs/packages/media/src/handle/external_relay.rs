use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use db::media_session::MediaSession;
use db::{media_session::MediaSessionState, Model};
use drop_tracer::{DropTracer, Token};
use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
use futures_util::StreamExt;
use log::*;
use mongodb::bson::doc;
use serde_util::DateTime;
use shutdown::Shutdown;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::task::JoinHandle;

use crate::channel::{SendError, Sender};
use crate::handle::util::PrettyDuration;

use constants::{
  EXTERNAL_RELAY_NO_DATA_SHUTDOWN_SECS, EXTERNAL_RELAY_NO_DATA_START_SHUTDOWN_SECS,
  EXTERNAL_RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS, /*STREAM_BURST_LENGTH,*/
  STREAM_BURST_LENGTH, STREAM_CHUNK_SIZE, STREAM_KBITRATE,
};

pub fn run_external_relay_source(
  sender: Sender,
  deployment_id: String,
  task_id: String,
  station_id: String,
  url: String,
  drop_tracer: DropTracer,
  shutdown: Shutdown,
) -> JoinHandle<Result<(), ExternalRelayError>> {
  tokio::spawn(async move {
    let signal = shutdown.signal();

    let media_session_id = MediaSession::uid();
    let document = {
      use db::media_session::*;
      let now = DateTime::now();
      let document = MediaSession {
        id: media_session_id.clone(),
        deployment_id,
        station_id: station_id.clone(),
        kind: MediaSessionKind::ExternalRelay { url: url.clone() },
        state: MediaSessionState::Open,
        closed_at: None,
        duration_ms: None,
        now_playing: None,
        transfer_bytes: 0,
        health_checked_at: Some(now),
        created_at: now,
        updated_at: now,
      };

      match MediaSession::insert(&document).await {
        Ok(_) => {}
        Err(e) => {
          error!(target: "media", "error inserting relay session document into db: {e} => {e:?}");
          return Err(ExternalRelayError::Db(e));
        }
      };
      document
    };

    info!(
      target: "media",
      "external relay media session start {}, station {}",
      document.id, document.station_id
    );

    let transfer = Arc::new(AtomicUsize::new(0));

    let dropper = MediaSessionDropper(Some((
      std::time::Instant::now(),
      document.id,
      transfer.clone(),
      drop_tracer.token(),
    )));

    let fut = async move {
      let ffmpeg_config = FfmpegConfig {
        input: Some(url),
        kbitrate: STREAM_KBITRATE,
        readrate: true,
        readrate_initial_burst: STREAM_BURST_LENGTH as f64,
        ..FfmpegConfig::default()
      };

      let ff_spawn = match Ffmpeg::new(ffmpeg_config).spawn() {
        Ok(spawn) => spawn,
        Err(e) => {
          error!(
            target: "media",
            "relay session ffmpeg spawn io: {e} => {e:?}"
          );
          return Err(ExternalRelayError::Spawn(e));
        }
      };

      let FfmpegSpawn {
        mut stderr,
        stdin: _stdin,
        stdout,
        mut child,
        config: _,
      } = ff_spawn;

      let stderr_handle = async move {
        let mut data = Vec::new();
        stderr.read_to_end(&mut data).await?;
        Result::<Vec<u8>, std::io::Error>::Ok(data)
      };

      let stdout_handle = {
        let station_id = station_id.clone();

        async move {
          use stream_util::*;

          let mut chunks = stdout.into_bytes_stream(STREAM_CHUNK_SIZE);

          let mut no_listeners_since: Option<Instant> = None;

          async fn chunk_timeout(first_chunk: bool) {
            if first_chunk {
              tokio::time::sleep(tokio::time::Duration::from_secs(
                EXTERNAL_RELAY_NO_DATA_START_SHUTDOWN_SECS,
              ))
              .await
            } else {
              tokio::time::sleep(tokio::time::Duration::from_secs(
                EXTERNAL_RELAY_NO_DATA_SHUTDOWN_SECS,
              ))
              .await
            }
          }

          let mut first_chunk = true;

          loop {
            let chunk = tokio::select! {
              _ = chunk_timeout(first_chunk) => {
                if first_chunk {
                  info!(
                    target: "media",
                    "shutting down external-relay for station {} (no data received in specified start window)",
                    station_id
                  );
                } else {
                  info!(
                    target: "media",
                    "shutting down external-relay for station {} (no data received in specified window)",
                    station_id
                  );
                }
                break;
              },

              chunk = chunks.next() => chunk
            };

            first_chunk = false;

            match chunk {
              None => break,
              Some(Err(_e)) => break,
              Some(Ok(bytes)) => {
                match sender.send(bytes) {
                  Ok(_) => {
                    no_listeners_since = None;
                  }

                  // check if shutdown delay is elapsed
                  Err(SendError::NoSubscribers(_)) => match no_listeners_since {
                    Some(instant) => {
                      if instant.elapsed().as_secs()
                        > EXTERNAL_RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS
                      {
                        info!(
                          "shutting down external-relay for station {} (no listeners shutdown delay elapsed)",
                          station_id
                        );
                        break;
                      } else {
                        continue;
                      }
                    }

                    None => {
                      no_listeners_since = Some(Instant::now());
                      continue;
                    }
                  },

                  Err(SendError::Terminated(_)) => break,
                };
              }
            }
          }
        }
      };

      let status_handle = async move { child.wait().await };

      let health_handle = crate::health::run_health_check_interval_for_station_and_media_session(
        &station_id,
        &media_session_id,
        &task_id,
      );

      let join_fut = async { tokio::join!(status_handle, stdout_handle, stderr_handle) };

      let (status, _stdout, stderr) = tokio::select! {
        tup = join_fut => tup,
        _ = health_handle => unreachable!()
      };

      let exit = match status {
        Ok(exit) => exit,

        Err(e) => {
          warn!(
            target: "media",
            "external-relay session for station {station_id}: ffmpeg child error: {} => {:?}",
            e, e
          );
          return Err(ExternalRelayError::ExitIo(e));
          // let mut headers = HeaderMap::with_capacity(1);
          // headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));

          // let body = Body::from("unexpected error allocating the stream converter (exit 1), please report this to the administrators");

          // let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
          // *res.headers_mut() = headers;
          // *res.body_mut() = body;

          // return res;
        }
      };

      drop(dropper);

      // 224 is stdout broken pipe (that happens normally when the session is cancelled)
      if exit.success() {
        Ok(())

        // let mut res = Response::new(StatusCode::OK);
        // *res.body_mut() = Body::from("data streamed successfully");

        // let mut headers = HeaderMap::with_capacity(1);
        // headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));

        // res
      } else {
        match stderr {
          Err(e) => {
            warn!(
              target: "media",
              "external-releay session {station_id}: ffmpeg exit non-zero: exit={exit} stderr_error={e}"
            );
            Err(ExternalRelayError::StderrError(e))
            // format!("internal error allocating stream converter (stderr 1)")
          }

          Ok(v) => {
            let stderr = String::from_utf8_lossy(v.as_ref()).to_string();
            // 224 happens when stdout is terminated (broken pipe) that happens normally when the session is cancelled
            if exit.code() == Some(224) && stderr.contains("Broken pipe") {
              Ok(())
            } else {
              warn!(
                target: "media",
                "external-releay session for station {station_id}: ffmpeg exit non-zero: exit={exit} stderr={stderr}"
              );
              Err(ExternalRelayError::ExitNotOk { stderr })
            }
          }
        }
      }
    };

    tokio::select! {
      r = fut => r,
      _ = signal => Ok(())
    }
  })
}

#[derive(Debug, thiserror::Error)]
pub enum ExternalRelayError {
  #[error("live error db: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("live error spawn: {0}")]
  Spawn(std::io::Error),
  #[error("live error stderr io: {0}")]
  StderrError(std::io::Error),
  #[error("live error exit io: {0}")]
  ExitIo(std::io::Error),
  #[error("live error exit not ok: stderr = {stderr}")]
  ExitNotOk { stderr: String },
}

#[derive(Debug)]
struct MediaSessionDropper(Option<(std::time::Instant, String, Arc<AtomicUsize>, Token)>);

impl Drop for MediaSessionDropper {
  fn drop(&mut self) {
    if let Some((start, id, transfer, token)) = self.0.take() {
      let duration_ms = start.elapsed().as_millis() as u64;
      info!(
        "closing external-relay media session {}: duration = {}",
        id,
        PrettyDuration(duration_ms)
      );

      tokio::spawn(async move {
        let transfer = transfer.load(Ordering::SeqCst);

        let now = DateTime::now();

        let update = doc! {
          "$set": {
            MediaSession::KEY_STATE: MediaSessionState::KEY_ENUM_VARIANT_CLOSED,
            MediaSession::KEY_TRANSFER_BYTES: transfer as f64,
            MediaSession::KEY_DURATION_MS: duration_ms as f64,
            MediaSession::KEY_CLOSED_AT: now
          }
        };

        if let Err(e) = db::media_session::MediaSession::update_by_id(&id, update).await {
          error!(
            "error closing external-relay session into db, session {}, {} {:?}",
            id, e, e
          )
        }

        drop(token)
      });
    }
  }
}

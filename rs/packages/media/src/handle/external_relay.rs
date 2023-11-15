use std::process::ExitStatus;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use db::media_session::MediaSession;
use db::{media_session::MediaSessionState, Model};
use drop_tracer::{DropTracer, Token};
use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn, Format};
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
        // audio/mpeg
        format: Format::MP3,
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

      let status_handle = {
        let station_id = station_id.clone();
        async move {
          use stream_util::*;
          use tokio::time::sleep;

          let mut chunks = stdout.into_bytes_stream(STREAM_CHUNK_SIZE);

          let mut no_listeners_since: Option<Instant> = None;

          let mut is_first_chunk = true;

          'chunks: loop {
            let timeout_secs: u64;
            let start_str: &str;

            if is_first_chunk {
              timeout_secs = EXTERNAL_RELAY_NO_DATA_START_SHUTDOWN_SECS;
              start_str = " start ";
            } else {
              timeout_secs = EXTERNAL_RELAY_NO_DATA_SHUTDOWN_SECS;
              start_str = " ";
            };

            let chunk = tokio::select! {

              chunk = chunks.next() => chunk,

              _ = sleep(Duration::from_secs(timeout_secs)) => {
                info!(
                  target: "media",
                  "shutting down external-relay for station {} (no data received in specified{start_str}window)",
                  station_id
                );
                break 'chunks;
              }
            };

            is_first_chunk = false;

            match chunk {
              None => break 'chunks,
              Some(Err(_e)) => break 'chunks,
              Some(Ok(bytes)) => {
                transfer.fetch_add(bytes.len(), Ordering::Relaxed);

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
                        break 'chunks;
                      }
                    }

                    None => {
                      no_listeners_since = Some(Instant::now());
                    }
                  },

                  Err(SendError::Terminated(_)) => break 'chunks,
                };
              }
            }
          }

          child.kill().await?;

          let exit = child.wait().await?;

          Ok::<ExitStatus, std::io::Error>(exit)
        }
      };

      let health_handle = crate::health::run_health_check_interval_for_station_and_media_session(
        &station_id,
        &media_session_id,
        &task_id,
      );

      let join_fut = async { tokio::join!(status_handle, stderr_handle) };

      let (status, stderr) = tokio::select! {
        (status, stderr) = join_fut => (status, stderr),
        r = health_handle => match r {
          Ok(never) => match never {},
          Err(e) => return Err(e.into()),
        }
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
        }
      };

      let r = if exit.success() {
        Ok(())
      } else {
        match stderr {
          Err(e) => {
            warn!(
              target: "media",
              "external-relay session {station_id}: ffmpeg exit non-zero: exit={exit} stderr_error={e}"
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
                "external-relay session for station {station_id}: ffmpeg exit non-zero: exit={exit} stderr={stderr}"
              );
              Err(ExternalRelayError::ExitNotOk { stderr })
            }
          }
        }
      };

      drop(dropper);

      r
    };

    let signal = shutdown.signal();

    let r = tokio::select! {
      r = fut => r,
      _ = signal => Ok(())
    };

    r
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

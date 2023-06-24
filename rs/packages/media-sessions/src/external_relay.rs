use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use crate::{SendError, Transmitter};
use constants::{
  EXTERNAL_RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS, STREAM_BURST_LENGTH, STREAM_CHUNK_SIZE,
  STREAM_KBITRATE,
};
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

#[derive(Debug, thiserror::Error)]
pub enum LiveError {
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

pub fn run_external_releay_session(
  tx: Transmitter,
  deployment_id: String,
  url: String,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
) -> tokio::task::JoinHandle<Result<(), LiveError>> {
  tokio::spawn(async move {
    let signal = shutdown.signal();

    let fut = async move {
      let station_id = tx.info.station_id().to_string();

      let document = {
        use db::media_session::*;
        let document = MediaSession {
          id: MediaSession::uid(),
          deployment_id,
          station_id: station_id.clone(),
          created_at: DateTime::now(),
          updated_at: DateTime::now(),
          kind: MediaSessionKind::ExternalRelay { url: url.clone() },
          state: MediaSessionState::Open,
          closed_at: None,
          duration_ms: None,
          now_playing: None,
          transfer_bytes: 0,
        };

        match MediaSession::insert(&document).await {
          Ok(_) => {}
          Err(e) => {
            error!("error inserting relay session document into db: {e} => {e:?}");
            return Err(LiveError::Db(e));
          }
        };
        document
      };

      info!(
        "relay media session start {}, station {}",
        document.id, document.station_id
      );

      let transfer = Arc::new(AtomicUsize::new(0));

      let dropper = MediaSessionDropper(Some((
        std::time::Instant::now(),
        document.id,
        transfer.clone(),
        drop_tracer.token(),
      )));

      let ffmpeg_config = FfmpegConfig {
        input: Some(url),
        kbitrate: STREAM_KBITRATE,
        ..FfmpegConfig::default()
      };

      // Err(_) => {
      //   // FORBIDEN (403) is used to communicate all sorts of errors
      //   let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
      //   *res.body_mut() = Body::from("error allocating internal stream converter, try again later or report it to the administrators");
      //   return res;
      // }

      let ff_spawn = match Ffmpeg::new(ffmpeg_config).spawn() {
        Ok(spawn) => spawn,
        Err(e) => {
          error!("relay session ffmpeg spawn io: {e} => {e:?}");
          return Err(LiveError::Spawn(e));
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
        let shutdown = shutdown.clone();
        let station_id = station_id.clone();

        async move {
          use stream_util::*;

          let mut chunks = stdout.into_bytes_stream(STREAM_CHUNK_SIZE);

          let mut no_listeners_since: Option<Instant> = None;

          // fill the burst
          let mut filled_burst_len: usize = 0;
          let go_on = loop {
            if filled_burst_len >= STREAM_BURST_LENGTH {
              break true;
            }

            match chunks.next().await {
              None => break false,
              Some(Err(_e)) => break false,
              Some(Ok(bytes)) => {
                if shutdown.is_closed() {
                  break false;
                }

                filled_burst_len += 1;
                match tx.send(bytes) {
                  Ok(_) => continue,

                  // check if shutdown delay is elapsed
                  Err(SendError::NoListeners(_)) => match no_listeners_since {
                    Some(instant) => {
                      if instant.elapsed().as_secs()
                        > EXTERNAL_RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS
                      {
                        info!(
                        "shutting down external-relay for station {} (no listeners shutdown delay elapsed)",
                        station_id
                      );
                        break false;
                      } else {
                        continue;
                      }
                    }

                    None => {
                      no_listeners_since = Some(Instant::now());
                      continue;
                    }
                  },
                  Err(SendError::Terminated(_)) => break false,
                };
              }
            }
          };

          if go_on {
            // we continue but now the stream is byte rated
            let chunks = chunks.rated(STREAM_KBITRATE * 1000);
            tokio::pin!(chunks);

            loop {
              match chunks.next().await {
                None => break,
                Some(Err(_e)) => break,
                Some(Ok(bytes)) => {
                  if shutdown.is_closed() {
                    break;
                  }

                  match tx.send(bytes) {
                    Ok(_) => continue,

                    // check if shutdown delay is elapsed
                    Err(SendError::NoListeners(_)) => match no_listeners_since {
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
        }
      };

      let status_handle = async move { child.wait().await };

      let (status, _stdout, stderr) = tokio::join!(status_handle, stdout_handle, stderr_handle);

      let exit = match status {
        Ok(exit) => exit,

        Err(e) => {
          warn!(
            "external-relay session for station {station_id}: ffmpeg child error: {} => {:?}",
            e, e
          );
          return Err(LiveError::ExitIo(e));
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
            warn!("external-releay session {station_id}: ffmpeg exit non-zero: exit={exit} stderr_error={e}");
            Err(LiveError::StderrError(e))
            // format!("internal error allocating stream converter (stderr 1)")
          }

          Ok(v) => {
            let stderr = String::from_utf8_lossy(v.as_ref()).to_string();
            // 224 happens when stdout is terminated (broken pipe) that happens normally when the session is cancelled
            if exit.code() == Some(224) && stderr.contains("Broken pipe") {
              Ok(())
            } else {
              warn!(
                "external-releay session for station {station_id}: ffmpeg exit non-zero: exit={exit} stderr={stderr}"
              );
              Err(LiveError::ExitNotOk { stderr })
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

#[derive(Debug)]
struct MediaSessionDropper(Option<(std::time::Instant, String, Arc<AtomicUsize>, Token)>);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct PrettyDuration(pub u64);

impl std::fmt::Display for PrettyDuration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const SEC: u64 = 1_000;
    const MIN: u64 = SEC * 60;
    const HOUR: u64 = MIN * 60;
    const DAY: u64 = HOUR * 24;

    let src = self.0;

    let ms = src % SEC;
    let s = (src % MIN) / SEC;
    let m = (src % HOUR) / MIN;
    let h = (src % DAY) / HOUR;
    let d = src / DAY;

    if d != 0 {
      write!(f, "{d}d {h}h {m}m {s}s {ms}ms")
    } else if h != 0 {
      write!(f, "{h}h {m}m {s}s {ms}ms")
    } else if m != 0 {
      write!(f, "{m}m {s}s {ms}ms")
    } else if s != 0 {
      write!(f, "{s}s {ms}ms")
    } else {
      write!(f, "{ms}ms")
    }
  }
}

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

        let update = doc! {
          "$set": {
            MediaSession::KEY_STATE: MediaSessionState::KEY_ENUM_VARIANT_CLOSED,
            MediaSession::KEY_TRANSFER_BYTES: transfer as f64,
            MediaSession::KEY_DURATION_MS: duration_ms as f64
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

use crate::{SendError, Transmitter};
use bytes::Bytes;
use constants::STREAM_CHUNK_SIZE;
use db::{media_session::MediaSessionState, Model};
use drop_tracer::{DropTracer, Token};
use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
use futures_util::{Stream, StreamExt};
use log::*;
use serde_util::DateTime;
use shutdown::Shutdown;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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

pub async fn run_live_session<E: std::error::Error>(
  tx: Transmitter,
  data: impl Stream<Item = Result<Bytes, E>>,
  request: db::http::Request,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
) -> Result<(), LiveError> {
  let signal = shutodown.signal();

  let fut = async move {
    tokio::pin!(data);

    let station_id = tx.info.station_id().to_string();

    let document = {
      use db::media_session::*;
      let document = MediaSession {
        id: MediaSession::uid(),
        station_id: station_id.clone(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        kind: MediaSessionKind::Live { request },
        state: MediaSessionState::Open,
      };

      match MediaSession::insert(&document).await {
        Ok(_) => {}
        Err(e) => {
          error!("error inserting live session document into db: {e} => {e:?}");
          return Err(LiveError::Db(e));
        }
      };
      document
    };

    info!(
      "live media session start {}, station {}",
      document.id, document.station_id
    );

    let dropper = MediaSessionDropper(Some((
      std::time::Instant::now(),
      document,
      drop_tracer.token(),
    )));

    let ffmpeg_config = FfmpegConfig {
      readrate: true,
      copycodec: true,
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
        error!("live session ffmpeg spawn io: {e} => {e:?}");
        return Err(LiveError::Spawn(e));
      }
    };

    let FfmpegSpawn {
      mut stderr,
      mut stdin,
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

      async move {
        use stream_util::*;

        let chunks = stdout.into_bytes_stream(STREAM_CHUNK_SIZE);

        tokio::pin!(chunks);

        loop {
          match chunks.next().await {
            None => {
              // trace!("channel {id}: ffmpeg stdout end");
              break;
            }
            Some(Err(_e)) => {
              // trace!("channel {id}: ffmpeg stdout error: {e}");
              break;
            }
            Some(Ok(bytes)) => {
              if shutdown.is_closed() {
                break;
              }
              // trace!("channel {id}: ffmpeg stdout data: {} bytes", bytes.len());
              // only fails if there are no receivers but we continue either way
              match tx.send(bytes) {
                Ok(_) => continue,
                Err(SendError::NoListeners(_)) => continue,
                Err(SendError::Terminated(_)) => break,
              };
            }
          }
        }
      }
    };

    let write_handle = {
      // move stdin to drop on close
      async move {
        loop {
          let data = data.next().await;

          if shutdown.is_closed() {
            break;
          }

          match data {
            None => {
              // trace!("channel {id}: recv body end");
              break;
            }

            Some(Err(_e)) => {
              // trace!("channel {id}: recv body error: {e}");
              break;
            }

            Some(Ok(data)) => {
              // trace!("channel {id}: recv body data: {} bytes", data.len());

              match stdin.write_all(data.as_ref()).await {
                Err(_e) => {
                  // trace!("channel {id} stdin error: {e}");
                  break;
                }

                Ok(()) => {
                  continue;
                  // trace!("channel {id} stdin write: {} bytes", data.len());
                }
              }
            }
          }
        }
      }
    };

    let status_handle = async move { child.wait().await };

    let (status, _write, _stdout, stderr) =
      tokio::join!(status_handle, write_handle, stdout_handle, stderr_handle);

    let exit = match status {
      Ok(exit) => exit,

      Err(e) => {
        warn!(
          "live session for station {station_id}: ffmpeg child error: {} => {:?}",
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

    // trace!("channel {id}: ffmpeg child end: {exit}");
    drop(dropper);

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
          warn!("channel {station_id}: ffmpeg exit non-zero: exit={exit} stderr_error={e}");
          Err(LiveError::StderrError(e))
          // format!("internal error allocating stream converter (stderr 1)")
        }

        Ok(v) => {
          let stderr = String::from_utf8_lossy(v.as_ref()).to_string();
          warn!(
            "live session for station {station_id}: ffmpeg exit non-zero: exit={exit} stderr={stderr}"
          );
          Err(LiveError::ExitNotOk { stderr })
          // format!("error converting the audio stream (exit), possibly the audio is corrupted or is using a not supported format: {out}")
        }
      }
    }
  };

  tokio::select! {
    r = fut => r,
    _ => signal => Ok(())
  }
}

#[derive(Debug)]
struct MediaSessionDropper(Option<(std::time::Instant, db::media_session::MediaSession, Token)>);

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
    if let Some((start, mut doc, token)) = self.0.take() {
      let duration_ms = start.elapsed().as_millis() as u64;
      info!(
        "closing live media session {} for station {}: duration = {}",
        doc.id,
        doc.station_id,
        PrettyDuration(duration_ms)
      );

      tokio::spawn(async move {
        doc.state = MediaSessionState::Closed {
          closed_at: DateTime::now(),
          duration_ms,
        };

        if let Err(e) = db::media_session::MediaSession::replace(&doc.id, &doc).await {
          error!(
            "error closing live session into db, session {}, station {}: = {} {:?}",
            doc.id, doc.station_id, e, e
          )
        }

        drop(token)
      });
    }
  }
}

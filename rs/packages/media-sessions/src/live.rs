use std::sync::{
  atomic::{AtomicU64, Ordering},
  Arc,
};

use crate::{SendError, Transmitter};
use bytes::Bytes;
use constants::STREAM_CHUNK_SIZE;
use db::{media_session::MediaSessionState, Model};
use drop_tracer::{DropTracer, Token};
use futures_util::{Stream, StreamExt};
use log::*;
use serde_util::DateTime;
use shutdown::Shutdown;
use stream_util::IntoTryBytesStreamChunked;

#[derive(Debug, thiserror::Error)]
pub enum LiveError<E> {
  #[error("mongodb: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("data: {0}")]
  Data(E),
  // #[error("probe: {0}")]
  // Probe(#[from] mp3::ProbeError),
  // #[error("play: {0}")]
  // Play(#[from] mp3::PlayError),
}

pub async fn run_live_session<E: std::error::Error + Send + Sync + 'static>(
  tx: Transmitter,
  data: impl Stream<Item = Result<Bytes, E>> + Send + Sync + 'static,
  deployment_id: String,
  request: db::http::Request,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
) -> Result<(), LiveError<E>> {
  let station_id = tx.info.station_id().to_string();

  let document = {
    use db::media_session::*;
    let now = DateTime::now();
    let document = MediaSession {
      id: MediaSession::uid(),
      deployment_id,
      station_id: station_id.clone(),
      created_at: now,
      updated_at: now,
      transfer_bytes: 0,
      kind: MediaSessionKind::Live { request },
      now_playing: None,
      state: MediaSessionState::Open,
      closed_at: None,
      duration_ms: None,
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

  let document_id = document.id.clone();

  info!("live media session start {document_id}, station {station_id}");

  let transfer_bytes = Arc::new(AtomicU64::new(0));

  let dropper = MediaSessionDropper(Some((
    document.id,
    document.station_id,
    std::time::Instant::now(),
    transfer_bytes.clone(),
    drop_tracer.token(),
  )));

  //let data = Box::pin(data.chunked(1000).rated(400_000 / 8));
  //let data = Box::pin(data);

  //let reader = mp3::TryStreamAsyncRead::new(data);

  //let output = mp3::readrate(reader).chunked(STREAM_CHUNK_SIZE);
  //tokio::pin!(output);

  use stream_util::IntoTryBytesStreamRated;
  let output = data.rated(400_000 / 8).chunked(STREAM_CHUNK_SIZE);
  tokio::pin!(output);

  let signal = shutdown.signal();
  let fut = async move {
    let mut transfer = 0u64;

    loop {
      if shutdown.is_closed() || tx.is_terminated() {
        break;
      }

      match output.next().await {
        None => break,
        Some(Err(e)) => {
          warn!("live session error: {e} => {e:?}");
          return Err(LiveError::Data(e));
        }
        Some(Ok(bytes)) => {
          transfer += bytes.len() as u64;
          transfer_bytes.store(transfer, Ordering::Release);
          match tx.send(bytes) {
            Ok(_) => continue,
            Err(SendError::NoListeners(_)) => continue,
            Err(SendError::Terminated(_)) => break,
          }
        }
      }
    }

    Ok(())
  };

  let result = tokio::select! {
    result = fut => result,
    _ = signal => Ok(()),
  };

  drop(dropper);

  result
}

#[derive(Debug)]
struct MediaSessionDropper(Option<(String, String, std::time::Instant, Arc<AtomicU64>, Token)>);

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct PrettyBytes(pub u64);

impl std::fmt::Display for PrettyBytes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const K: u64 = 1_000;
    const M: u64 = 1_000_000;
    const G: u64 = 1_000_000_000;

    let b = self.0;

    if b < K {
      write!(f, "{b} B")
    } else if b < M {
      let k = b as f64 / K as f64;
      write!(f, "{:.2} KB", k)
    } else if b < G {
      let m = b as f64 / M as f64;
      write!(f, "{:.2} MB", m)
    } else {
      write!(f, "{:.2} GB", b as f64 / G as f64)
    }
  }
}

impl Drop for MediaSessionDropper {
  fn drop(&mut self) {
    use mongodb::bson::doc;

    if let Some((id, station_id, start, transfer, token)) = self.0.take() {
      let duration_ms = start.elapsed().as_millis() as u64;
      let transfer_bytes = transfer.load(Ordering::Acquire);
      info!(
        "closing live media session {} for station {}: duration = {}, transfer = {}",
        id,
        station_id,
        PrettyDuration(duration_ms),
        PrettyBytes(transfer_bytes),
      );

      tokio::spawn(async move {
        use db::media_session::MediaSession;

        let now = DateTime::now();

        let update = doc! {
          "$set": {
            MediaSession::KEY_UPDATED_AT: now,
            MediaSession::KEY_CLOSED_AT: now,
            MediaSession::KEY_STATE: MediaSessionState::KEY_ENUM_VARIANT_CLOSED,
            MediaSession::KEY_DURATION_MS: duration_ms as f64,
            MediaSession::KEY_TRANSFER_BYTES: transfer_bytes as f64,
          }
        };

        if let Err(e) = MediaSession::update_by_id(&id, update).await {
          error!(
            "error closing live session into db, session {}, station {}: = {} {:?}",
            id, station_id, e, e
          )
        }

        drop(token)
      });
    }
  }
}

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
pub enum LiveError {
  #[error("mongodb: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("mp3: {0}")]
  Mp3(#[from] mp3::ReadRateError),
  // #[error("probe: {0}")]
  // Probe(#[from] mp3::ProbeError),
  // #[error("play: {0}")]
  // Play(#[from] mp3::PlayError),
}

pub async fn run_live_session(
  tx: Transmitter,
  data: impl Stream<Item = Result<Bytes, impl std::error::Error + Send + Sync + 'static>>
    + Send
    + Sync
    + 'static,
  request: db::http::Request,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
) -> Result<(), LiveError> {
  let station_id = tx.info.station_id().to_string();

  let document = {
    use db::media_session::*;
    let document = MediaSession {
      id: MediaSession::uid(),
      station_id: station_id.clone(),
      created_at: DateTime::now(),
      updated_at: DateTime::now(),
      transfer_bytes: 0,
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

  let document_id = document.id.clone();

  info!("live media session start {document_id}, station {station_id}");

  let transfer_bytes = Arc::new(AtomicU64::new(0));

  let dropper = MediaSessionDropper(Some((
    std::time::Instant::now(),
    transfer_bytes.clone(),
    document,
    drop_tracer.token(),
  )));

  //let data = Box::pin(data.chunked(1000).rated(400_000 / 8));
  let data = Box::pin(data);

  let reader = mp3::TryStreamAsyncRead::new(data);

  let output = mp3::readrate(reader).chunked(STREAM_CHUNK_SIZE);
  tokio::pin!(output);

  let mut transfer = 0u64;

  loop {
    if shutdown.is_closed() || tx.is_terminated() {
      break;
    }

    match output.next().await {
      None => break,
      Some(Err(e)) => {
        warn!("live session error: {e} => {e:?}");
        return Err(e.into());
      }
      Some(Ok(bytes)) => {
        transfer += bytes.len() as u64;
        transfer_bytes.store(transfer, Ordering::Relaxed);
        match tx.send(bytes) {
          Ok(_) => continue,
          Err(SendError::NoListeners(_)) => continue,
          Err(SendError::Terminated(_)) => break,
        }
      }
    }
  }

  // let mut output = rx.chunked(STREAM_CHUNK_SIZE);
  // // tokio::pin!(output);
  // loop {
  //   if shutdown.is_closed() || tx.is_terminated() {
  //     break;
  //   }

  //   match output.next().await {
  //     None => break,
  //     Some(Err(e)) => {
  //       warn!("live media session {document_id} for station {station_id} play error: {e} => {e:?}");
  //       return Err(e.into());
  //     }
  //     Some(Ok(bytes)) => match tx.send(bytes) {
  //       Ok(_) => continue,
  //       Err(SendError::NoListeners(_)) => continue,
  //       Err(SendError::Terminated(_)) => break,
  //     },
  //   }
  // }

  drop(dropper);

  Ok(())
}

#[derive(Debug)]
struct MediaSessionDropper(
  Option<(
    std::time::Instant,
    Arc<AtomicU64>,
    db::media_session::MediaSession,
    Token,
  )>,
);

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
    if let Some((start, transfer, mut doc, token)) = self.0.take() {
      let duration_ms = start.elapsed().as_millis() as u64;
      let transfer_bytes = transfer.load(Ordering::Acquire);
      info!(
        "closing live media session {} for station {}: duration = {}, transfer = {}",
        doc.id,
        doc.station_id,
        PrettyDuration(duration_ms),
        PrettyBytes(transfer_bytes),
      );

      tokio::spawn(async move {
        doc.transfer_bytes = transfer_bytes;
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

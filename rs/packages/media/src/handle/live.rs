use std::sync::{
  atomic::{AtomicU64, Ordering},
  Arc,
};

use crate::{
  channel::{SendError, Sender},
  handle::util::{PrettyBytes, PrettyDuration},
};
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

#[allow(clippy::too_many_arguments)]
pub async fn run_live_source<E: std::error::Error + Send + Sync + 'static>(
  sender: Sender,
  deployment_id: String,
  task_id: String,
  station_id: String,
  data: impl Stream<Item = Result<Bytes, E>> + Send + Sync + 'static,
  request: db::http::Request,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
) -> Result<(), LiveError<E>> {
  let media_session_id = db::media_session::MediaSession::uid();
  let document = {
    use db::media_session::*;
    let now = DateTime::now();
    let document = MediaSession {
      id: media_session_id.clone(),
      deployment_id,
      station_id: station_id.clone(),
      transfer_bytes: 0,
      kind: MediaSessionKind::Live { request },
      now_playing: None,
      state: MediaSessionState::Open,
      closed_at: None,
      duration_ms: None,
      health_checked_at: Some(now),
      created_at: now,
      updated_at: now,
    };

    match MediaSession::insert(&document).await {
      Ok(_) => {}
      Err(e) => {
        error!(
          target: "media",
          "error inserting live session document into db: {e} => {e:?}"
        );
        return Err(LiveError::Db(e));
      }
    };
    document
  };

  let document_id = document.id.clone();

  info!(
    target: "media",
    "live media session start {document_id}, station {station_id}"
  );

  let transfer_bytes = Arc::new(AtomicU64::new(0));

  let dropper = MediaSessionDropper(Some((
    document.id,
    document.station_id,
    std::time::Instant::now(),
    transfer_bytes.clone(),
    drop_tracer.token(),
  )));

  let handle = async move {
    use stream_util::IntoTryBytesStreamRated;
    let output = data.rated(400_000 / 8).chunked(STREAM_CHUNK_SIZE);
    tokio::pin!(output);

    let signal = shutdown.signal();

    let fut = async move {
      let mut transfer = 0u64;

      loop {
        match output.next().await {
          None => break,
          Some(Err(e)) => {
            warn!("live session error: {e} => {e:?}");
            return Err(LiveError::Data(e));
          }
          Some(Ok(bytes)) => {
            transfer += bytes.len() as u64;
            transfer_bytes.store(transfer, Ordering::Release);
            match sender.send(bytes) {
              Ok(_) => continue,
              Err(SendError::NoSubscribers(_)) => continue,
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

    result
  };

  let health_handle = crate::health::run_health_check_interval_for_station_and_media_session(
    &station_id,
    &media_session_id,
    &task_id,
  );

  let result = tokio::select! {
    result = handle => result,
    () = health_handle => unreachable!()
  };

  drop(dropper);

  result
}

#[derive(Debug)]
struct MediaSessionDropper(Option<(String, String, std::time::Instant, Arc<AtomicU64>, Token)>);

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

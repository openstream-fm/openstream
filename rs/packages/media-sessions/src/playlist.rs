use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;

use constants::{PLAYLIST_NO_LISTENERS_SHUTDOWN_DELAY, STREAM_BURST_LENGTH, STREAM_CHUNK_SIZE};
use db::{audio_chunk::AudioChunk, audio_file::AudioFile, Model};
use drop_tracer::{DropTracer, Token};
use log::*;

use parking_lot::Mutex;
use serde_util::DateTime;

use futures_util::stream::{StreamExt, TryStreamExt};
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;

use atomic_float::AtomicF64;

use shutdown::Shutdown;
use std::sync::Arc;
use stream_util::{IntoTryBytesStreamChunked, IntoTryBytesStreamRated};

use crate::{SendError, Transmitter};

pub fn run_playlist_session(
  tx: Transmitter,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
  resume: bool,
) -> tokio::task::JoinHandle<Result<(), mongodb::error::Error>> {
  tokio::spawn(async move {
    let station_id = tx.info.station_id.as_str();

    let result = async {
      let station_id = tx.info.station_id.as_str();

      let (resume_playlist_id, start_file_id, i, part) = if resume {
        resume_info_for_station(station_id).await?
      } else {
        let file = AudioFile::playlist_first(station_id).await?;
        (None, file, 0.0, 0)
      };

      let start_file = match start_file_id {
        None => {
          info!(
            "not starting playlist session for station {station_id} no files found for account"
          );
          return Ok(());
        }
        Some(id) => id,
      };

      info!(
        "media session (playlist) start for station {} file_id={} order={} chunk={} part={}",
        station_id, start_file.id, start_file.order, i, part
      );

      let out = PlaylistIndexInfoOut(Arc::new(Inner {
        file_id: Mutex::new(start_file.id.clone()),
        file_order: AtomicF64::new(start_file.order),
        i: AtomicF64::new(i),
        part: AtomicUsize::new(part),
        transfer: AtomicU64::new(0),
      }));

      let media_session_doc = {
        use db::media_session::*;
        let media_session_doc = MediaSession {
          id: MediaSession::uid(),
          station_id: station_id.to_string(),
          created_at: DateTime::now(),
          updated_at: DateTime::now(),
          transfer_bytes: 0,
          kind: MediaSessionKind::Playlist {
            resumed_from: resume_playlist_id,
            last_audio_chunk_date: DateTime::now(),
            last_audio_chunk_i: i,
            last_audio_chunk_skip_parts: part,
            last_audio_file_id: start_file.id.clone(),
            last_audio_file_order: start_file.order,
          },
          state: MediaSessionState::Open,
        };

        MediaSession::insert(&media_session_doc).await?;
        media_session_doc
      };

      let media_session_doc_id = media_session_doc.id.clone();

      let dropper = MediaSessionDropper {
        doc: media_session_doc,
        out: out.clone(),
        token: Some(drop_tracer.token()),
        start: Instant::now(),
      };

      let mut first = true;

      // we fill the burst on start
      let mut burst_len: usize = 0;

      let mut no_listeners_since: Option<Instant> = None;

      let mut current_file = start_file;

      'files: loop {
        let (i, part) = if first {
          first = false;
          (i, part)
        } else {
          let next_file =
            AudioFile::playlist_next(station_id, &current_file.id, current_file.order).await?;

          match next_file {
            None => {
              info!(
                "stopping playlist for station {} (no files found in account)",
                station_id
              );

              break 'files;
            }

            Some(next_file) => {
              current_file = next_file;
            }
          }

          (0.0, 0)
        };

        if shutdown.is_closed() || tx.is_terminated() {
          return Ok(());
        }

        info!(
          "start playback of audio file {}: '{}' for station {}",
          current_file.id,
          current_file
            .metadata
            .title
            .as_ref()
            .unwrap_or(&current_file.filename),
          station_id,
        );

        {
          use db::media_session::MediaSession;
          MediaSession::set_file_chunk_part(
            &media_session_doc_id,
            &current_file.id,
            i,
            part as f64,
          )
          .await?;
        }

        out.set_file_id(current_file.id.clone());
        out.set_file_order(current_file.order);
        out.set_i(i);
        out.set_part(part);

        let mut first_item = true;

        let stream = AudioChunk::stream_from(&current_file.id, i).inspect(|_| {
          if first_item {
            first_item = false;
          } else {
            out.increment_i();
            out.set_part(0);
          }
        });

        let stream = stream.chunked(STREAM_CHUNK_SIZE).skip(part).inspect(|_| {
          out.increment_part();
        });
        //.rated(file.bytes_sec)

        let mut transfer = 0u64;

        // fill the burst without delay between chunk parts
        tokio::pin!(stream);
        if burst_len < STREAM_BURST_LENGTH {
          'chunks: loop {
            if shutdown.is_closed() || tx.is_terminated() {
              break 'files;
            }

            match stream.try_next().await? {
              None => continue 'files,

              Some(bytes) => {
                burst_len += 1;

                transfer += bytes.len() as u64;
                out.set_transfer(transfer);

                if shutdown.is_closed() {
                  return Ok(());
                }

                let burst_filled = burst_len >= STREAM_BURST_LENGTH;

                match tx.send(bytes) {
                  Ok(_) | Err(SendError::NoListeners(_)) => {
                    if burst_filled {
                      break 'chunks;
                    } else {
                      continue 'chunks;
                    }
                  }

                  // here the stream has been terminated (maybe replaced with a newer transmitter)
                  Err(SendError::Terminated(_)) => break 'files,
                }
              }
            }
          }
        }

        // add byte rate to the stream
        let stream = stream.rated(current_file.bytes_sec);
        tokio::pin!(stream);

        'chunks: loop {
          if shutdown.is_closed() {
            break 'files;
          }

          match stream.try_next().await? {
            None => continue 'files,

            Some(bytes) => {
              transfer += bytes.len() as u64;
              out.set_transfer(transfer);
              if shutdown.is_closed() || tx.is_terminated() {
                return Ok(());
              }

              match tx.send(bytes) {
                // n is the number of listeners that received the chunk
                Ok(_) => {
                  no_listeners_since = None;
                  continue 'chunks;
                }
                
                // check if shutdown delay is elapsed
                Err(SendError::NoListeners(_)) => match no_listeners_since {
                  Some(instant) => {
                    if instant.elapsed() > PLAYLIST_NO_LISTENERS_SHUTDOWN_DELAY {
                      info!(
                        "shutting down playlist for station {} (no listeners shutdown delay elapsed)",
                          station_id
                      );
                      break 'files;
                    } else {
                      continue 'chunks;
                    }
                  }
                
                  None => {
                    no_listeners_since = Some(Instant::now());
                    continue 'chunks;
                  }
                } 
                // here the stream has been terminated (maybe replaced with a newer transmitter)
                Err(SendError::Terminated(_)) => break 'files,
              }
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

async fn resume_info_for_station(
  station_id: &str,
) -> Result<(Option<String>, Option<AudioFile>, f64, usize), mongodb::error::Error> {
  use db::media_session::{MediaSession, MediaSessionKind};
  let filter = doc! {
    MediaSession::KEY_STATION_ID: station_id,
    MediaSessionKind::KEY_ENUM_TAG: MediaSessionKind::TAG_PLAYLIST,
  };
  let sort = doc! {
    MediaSession::KEY_CREATED_AT: -1
  };

  let options = FindOneOptions::builder().sort(sort).build();

  let session = match MediaSession::cl().find_one(filter, options).await? {
    None => {
      let file = AudioFile::playlist_first(station_id).await?;
      return Ok((None, file, 0.0, 0));
    }
    Some(session) => session,
  };

  let (audio_file_id, audio_file_order, i, parts) = match session.kind {
    // this will never happen for security we provide an impl nevertheless
    MediaSessionKind::Live { .. } => {
      warn!(
        "unreachable MediaSessionKind::Live reached for station {} playlist",
        station_id
      );

      let file = AudioFile::playlist_first(station_id).await?;
      return Ok((None, file, 0.0, 0));
    }

    MediaSessionKind::Playlist {
      resumed_from: _,
      last_audio_file_id,
      last_audio_file_order,
      last_audio_chunk_i,
      last_audio_chunk_skip_parts,
      last_audio_chunk_date: _,
    } => (
      last_audio_file_id,
      last_audio_file_order,
      last_audio_chunk_i,
      last_audio_chunk_skip_parts,
    ),
  };

  let filter = doc! { AudioFile::KEY_ID: &audio_file_id, AudioFile::KEY_STATION_ID: station_id };
  let file = match AudioFile::cl().find_one(filter, None).await? {
    None => {
      match AudioFile::playlist_next(station_id, &audio_file_id, audio_file_order).await? {
        None => return Ok((Some(session.id), None, 0.0, 0)),
        Some(file) => return Ok((Some(session.id), Some(file), 0.0, 0)),
      };
    }
    Some(file) => file,
  };

  Ok((Some(session.id), Some(file), i, parts))
}

#[derive(Debug, Clone)]
pub struct PlaylistIndexInfoOut(Arc<Inner>);

#[derive(Debug)]
struct Inner {
  file_id: Mutex<String>,
  file_order: AtomicF64,
  i: AtomicF64,
  part: AtomicUsize,
  transfer: AtomicU64,
}

impl PlaylistIndexInfoOut {
  pub fn i(&self) -> f64 {
    self.0.i.load(Ordering::SeqCst)
  }

  pub fn part(&self) -> usize {
    self.0.part.load(Ordering::SeqCst)
  }

  pub fn file_id(&self) -> String {
    self.0.file_id.lock().clone()
  }

  pub fn file_order(&self) -> f64 {
    self.0.file_order.load(Ordering::SeqCst)
  }

  pub fn transfer(&self) -> u64 {
    self.0.transfer.load(Ordering::Relaxed)
  }

  pub fn set_i(&self, v: f64) {
    // info!("set i {v}");
    self.0.i.store(v, Ordering::SeqCst);
  }

  pub fn increment_i(&self) {
    let _v = self.0.i.fetch_add(1.0, Ordering::SeqCst);
    // info!("increment i {}", v + 1);
  }

  pub fn set_part(&self, v: usize) {
    // info!("set part {v}");
    self.0.part.store(v, Ordering::SeqCst);
  }

  pub fn set_file_order(&self, v: f64) {
    // info!("set part {v}");
    self.0.file_order.store(v, Ordering::SeqCst);
  }

  pub fn increment_part(&self) {
    // info!("increment part {}", v + 1);
    let _ = self.0.part.fetch_add(1, Ordering::SeqCst);
  }

  pub fn set_file_id(&self, id: String) {
    // info!("set file_id {id:?}");
    *self.0.file_id.lock() = id;
  }

  pub fn set_transfer(&self, n: u64) {
    self.0.transfer.store(n, Ordering::Relaxed)
  }
}

#[derive(Debug)]
struct MediaSessionDropper {
  start: Instant,
  doc: db::media_session::MediaSession,
  out: PlaylistIndexInfoOut,
  token: Option<Token>,
}

impl Drop for MediaSessionDropper {
  fn drop(&mut self) {
    use db::media_session::*;

    let token = match self.token.take() {
      None => return,
      Some(token) => token,
    };

    let now = DateTime::now();

    let file_id = self.out.file_id();
    let file_order = self.out.file_order();
    let i = self.out.i();
    let part = self.out.part();
    let transfer_bytes = self.out.transfer();

    let doc = MediaSession {
      id: self.doc.id.clone(),
      station_id: self.doc.station_id.clone(),
      created_at: self.doc.created_at,
      updated_at: self.doc.updated_at,
      transfer_bytes,
      kind: MediaSessionKind::Playlist {
        resumed_from: self.doc.resumed_from().map(ToString::to_string),
        last_audio_file_id: file_id.clone(),
        last_audio_file_order: file_order,
        last_audio_chunk_i: i,
        last_audio_chunk_skip_parts: part,
        last_audio_chunk_date: now,
      },
      state: MediaSessionState::Closed {
        closed_at: now,
        duration_ms: self.start.elapsed().as_millis() as u64,
      },
    };

    tokio::spawn(async move {
      info!(
        "saving media session {} station_id={} file_id={} i={} part={}",
        doc.id, doc.station_id, file_id, i, part,
      );
      match MediaSession::replace(&doc.id, &doc).await {
        Err(e) => warn!(
          "error saving media session {} for station {}: {}",
          doc.id, doc.station_id, e
        ),
        Ok(r) => {
          if r.matched_count != 1 {
            warn!(
              "media session save id={} station_id={} returned matched count != 1 ({})",
              doc.id, doc.station_id, r.matched_count
            )
          }
        }
      }
      drop(token)
    });
  }
}

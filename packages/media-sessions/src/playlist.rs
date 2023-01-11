use std::{
  sync::atomic::{AtomicU64, AtomicUsize, Ordering},
  time::Instant,
};

use constants::STREAM_CHUNK_SIZE;
use db::{audio_chunk::AudioChunk, audio_file::AudioFile, Model};
use drop_tracer::{DropTracer, Token};
use log::*;

use parking_lot::Mutex;
use serde_util::DateTime;

use crate::{SendError, Transmitter};
use futures_util::stream::{StreamExt, TryStreamExt};
use mongodb::{
  bson::doc,
  options::{FindOneOptions, FindOptions},
};
use shutdown::Shutdown;
use std::sync::Arc;
use stream_util::{IntoTryBytesStreamChunked, IntoTryBytesStreamRated};

pub fn run_playlist_session(
  tx: Transmitter,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
  resume: bool,
) -> tokio::task::JoinHandle<Result<(), mongodb::error::Error>> {
  tokio::spawn(async move {
    let account_id = tx.info.station_id.as_str();

    let result = async {
      let account_id = tx.info.station_id.as_str();
      let filter = doc! { AudioFile::KEY_ACCOUNT_ID: account_id };

      let (resume_playlist_id, start_file_id, skip, i, part) = if resume {
        resume_info_for_account(account_id).await?
      } else {
        (None, None, 0, 0.0, 0)
      };

      info!(
        "media session (playlist) start for account {account_id} file_id=={start_file_id:?} skip={skip} i={i} part={part}"
      );

      let out = PlaylistIndexInfoOut(Arc::new(Inner {
        file_id: Mutex::new(start_file_id.clone()),
        i: AtomicU64::new(i as u64),
        part: AtomicUsize::new(part),
      }));

      let media_session_doc = {
        use db::media_session::*;
        let media_session_doc = MediaSession {
          id: MediaSession::uid(),
          account_id: account_id.to_string(),
          created_at: DateTime::now(),
          updated_at: DateTime::now(),
          kind: MediaSessionKind::Playlist {
            resumed_from: resume_playlist_id,
            last_audio_chunk_date: DateTime::now(),
            last_audio_chunk_i: i,
            last_audio_chunk_skip_parts: part,
            last_audio_file_id: start_file_id,
          },
          state: MediaSessionState::Open,
        };

        MediaSession::insert(&media_session_doc).await?;
        media_session_doc
      };

      let dropper = MediaSessionDropper {
        doc: media_session_doc,
        out: out.clone(),
        token: Some(drop_tracer.token()),
        start: Instant::now(),
      };

      let mut skip = skip;
      let mut first = true;

      'files: loop {
        let (i, part) = if first {
          first = false;
          (i, part)
        } else {
          (0.0, 0)
        };

        if shutdown.is_closed() || tx.is_terminated() {
          return Ok(());
        }

        info!("seeking file for account {account_id}, skip = {skip}");

        let options = FindOneOptions::builder().skip(skip).build();
        let file = AudioFile::cl().find_one(filter.clone(), options).await?;
        let file = match file {
          Some(file) => file,
          None => {
            if skip == 0 {
              info!("no files found for account {account_id}");
              return Ok(());
            } else {
              info!("playlist restart for account {account_id}");
              skip = 0;
              continue 'files;
            }
          }
        };

        skip += 1;

        info!(
          "start playback of audio file {}: '{}' for account {}",
          file.id,
          file.metadata.title.as_ref().unwrap_or(&file.filename),
          account_id,
        );

        out.set_file_id(file.id.clone());
        out.set_i(i);
        out.set_part(part);

        let mut first_item = true;

        let stream = AudioChunk::stream_from(&file.id, i).inspect(|_| {
          if first_item {
            first_item = false;
          } else {
            out.increment_i();
            out.set_part(0);
          }
        });

        let stream = stream
          .chunked(STREAM_CHUNK_SIZE)
          .skip(part)
          .rated(file.bytes_sec)
          .inspect(|_| {
            out.increment_part();
          });

        tokio::pin!(stream);

        'chunks: loop {
          if shutdown.is_closed() || tx.is_terminated() {
            return Ok(());
          }

          match stream.try_next().await? {
            None => break 'chunks,

            Some(bytes) => {
              if shutdown.is_closed() || tx.is_terminated() {
                return Ok(());
              }

              match tx.send(bytes) {
                // n is the number of listeners that received the chunk
                Ok(_n) => continue 'chunks,
                // we ignore no listeners and continue streaming
                Err(SendError::NoListeners(_)) => continue 'chunks,
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
      warn!("media session for account {account_id} error: {e} => {e:?}");
    }

    result
  })
}

async fn resume_info_for_account(
  account_id: &str,
) -> Result<(Option<String>, Option<String>, u64, f64, usize), mongodb::error::Error> {
  use db::media_session::{MediaSession, MediaSessionKind};
  let filter = doc! {
    MediaSession::KEY_ACCOUNT_ID: account_id,
    MediaSessionKind::KEY_ENUM_TAG: MediaSessionKind::TAG_PLAYLIST,
  };
  let sort = doc! {
    MediaSession::KEY_CREATED_AT: -1
  };

  let options = FindOneOptions::builder().sort(sort).build();

  let session = match MediaSession::cl().find_one(filter, options).await? {
    None => return Ok((None, None, 0, 0.0, 0)),
    Some(session) => session,
  };

  let (audio_file_id, i, parts) = match session.kind {
    // this will never happen
    MediaSessionKind::Live { .. } => return Ok((None, None, 0, 0.0, 0)),
    MediaSessionKind::Playlist {
      resumed_from: _,
      last_audio_file_id,
      last_audio_chunk_i,
      last_audio_chunk_skip_parts,
      last_audio_chunk_date: _,
    } => (
      last_audio_file_id,
      last_audio_chunk_i,
      last_audio_chunk_skip_parts,
    ),
  };

  let audio_file_id = match audio_file_id {
    None => return Ok((Some(session.id), None, 0, 0.0, 0)),
    Some(id) => id,
  };

  let filter = doc! { AudioFile::KEY_ACCOUNT_ID: account_id };
  let projection = db::id_document_projection();
  let options = FindOptions::builder().projection(projection).build();
  let mut cursor = AudioFile::cl_as::<db::IdDocument>()
    .find(filter, options)
    .await?;

  let mut skip = 0;
  while let Some(doc) = cursor.try_next().await? {
    if doc.id == audio_file_id {
      return Ok((Some(session.id), Some(audio_file_id), skip, i, parts));
    }
    skip += 1;
  }

  Ok((Some(session.id), None, 0, 0.0, 0))
}

#[derive(Debug, Clone)]
pub struct PlaylistIndexInfoOut(Arc<Inner>);

#[derive(Debug)]
struct Inner {
  file_id: Mutex<Option<String>>,
  i: AtomicU64,
  part: AtomicUsize,
}

impl PlaylistIndexInfoOut {
  pub fn i(&self) -> f64 {
    self.0.i.load(Ordering::SeqCst) as f64
  }

  pub fn part(&self) -> usize {
    self.0.part.load(Ordering::SeqCst)
  }

  pub fn file_id(&self) -> Option<String> {
    self.0.file_id.lock().clone()
  }

  pub fn set_i(&self, v: f64) {
    // info!("set i {v}");
    self.0.i.store(v as u64, Ordering::SeqCst);
  }

  pub fn increment_i(&self) {
    let _v = self.0.i.fetch_add(1, Ordering::SeqCst);
    // info!("increment i {}", v + 1);
  }

  pub fn set_part(&self, v: usize) {
    // info!("set part {v}");
    self.0.part.store(v, Ordering::SeqCst);
  }

  pub fn increment_part(&self) {
    let _ = self.0.part.fetch_add(1, Ordering::SeqCst);
    // info!("increment part {}", v + 1);
  }

  pub fn set_file_id(&self, id: impl Into<Option<String>>) {
    let id = id.into();
    // info!("set file_id {id:?}");
    *self.0.file_id.lock() = id;
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
    let i = self.out.i();
    let part = self.out.part();

    let doc = MediaSession {
      id: self.doc.id.clone(),
      account_id: self.doc.account_id.clone(),
      created_at: self.doc.created_at,
      updated_at: self.doc.updated_at,
      kind: MediaSessionKind::Playlist {
        resumed_from: self.doc.resumed_from().map(|s| s.to_string()),
        last_audio_file_id: file_id.clone(),
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
        "saving media session {} account_id={} file_id={:?} i={} part={}",
        doc.id, doc.account_id, file_id, i, part,
      );
      match MediaSession::replace(&doc.id, &doc).await {
        Err(e) => warn!(
          "error saving media session {} for account {}: {}",
          doc.id, doc.account_id, e
        ),
        Ok(r) => {
          if r.matched_count != 1 {
            warn!(
              "media session save id={} account_id={} returned matched count != 1 ({})",
              doc.id, doc.account_id, r.matched_count
            )
          }
        }
      }
      drop(token)
    });
  }
}

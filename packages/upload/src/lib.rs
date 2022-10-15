use std::fmt::Display;

use bytes::Bytes;
use chrono::Utc;
use constants::{AUDIO_FILE_BYTERATE, AUDIO_FILE_CHUNK_SIZE};
use db::audio_file::AudioFile;
use db::audio_upload_operation::AudioUploadOperation;
use db::{audio_chunk::AudioChunk, audio_upload_operation::State};
use log::*;
use md5::{Digest, Md5};
use std::error::Error;
use stream_util::*;
use tokio_stream::{Stream, StreamExt};

#[derive(Debug)]
pub enum UploadError<E> {
  Stream(E),
  Mongo(mongodb::error::Error),
  SizeExceeded,
  Empty,
}

impl<E: Error> Display for UploadError<E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Stream(e) => write!(f, "stream: {}", e),
      Self::Mongo(e) => write!(f, "mongo: {}", e),
      Self::SizeExceeded => write!(f, "size exceeded: file stream is too large"),
      Self::Empty => write!(f, "empty: file stream is empty"),
    }
  }
}

impl<E: Error> Error for UploadError<E> {}

impl<E> From<mongodb::error::Error> for UploadError<E> {
  fn from(e: mongodb::error::Error) -> Self {
    Self::Mongo(e)
  }
}

async fn upload_audio_file_internal<E: Error, S: Stream<Item = Result<Bytes, E>>>(
  account_id: String,
  audio_file_id: Option<String>,
  size_limit: usize,
  data: S,
) -> Result<AudioFile, UploadError<E>> {
  let data = data.chunked(AUDIO_FILE_CHUNK_SIZE);

  tokio::pin!(data);

  let mut hasher = Md5::new();

  let audio_file_id = audio_file_id.unwrap_or_else(|| db::audio_file::uid());

  let mut file_len = 0;
  let mut file_duration_ms = 0.0;
  let mut chunk_count = 0;

  loop {
    let item = data.next().await;
    match item {
      None => break,
      Some(Err(e)) => return Err(UploadError::Stream(e)),
      Some(Ok(bytes)) => {
        hasher.update(bytes.as_ref());

        let i = chunk_count;
        chunk_count += 1;

        let len = bytes.len();
        file_len += len;

        if file_len > size_limit {
          return Err(UploadError::SizeExceeded);
        }

        let duration_ms = bytes.len() as f64 / AUDIO_FILE_BYTERATE as f64 * 1000.0;

        let start_ms = file_duration_ms;
        file_duration_ms += duration_ms;

        let end_ms = start_ms + duration_ms;

        let document = AudioChunk {
          id: db::audio_chunk::uid(),
          account_id: account_id.clone(),
          audio_file_id: audio_file_id.clone(),
          duration_ms,
          start_ms,
          end_ms,
          i,
          len,
          bytes_sec: AUDIO_FILE_BYTERATE,
          data: bytes,
          created_at: Utc::now(),
        };

        db::audio_chunk::cl().insert_one(document, None).await?;
      }
    }
  }

  if file_len == 0 {
    return Err(UploadError::Empty);
  }

  let md5_array = hasher.finalize();
  let md5 = hex::encode(md5_array);

  let file = AudioFile {
    id: audio_file_id,
    account_id,
    md5,
    len: file_len,
    duration_ms: file_duration_ms,
    chunk_count,
    chunk_len: AUDIO_FILE_CHUNK_SIZE,
    chunk_duration_ms: AUDIO_FILE_CHUNK_SIZE as f64 / AUDIO_FILE_BYTERATE as f64 * 1000.0,
    bytes_sec: AUDIO_FILE_BYTERATE,
    created_at: Utc::now(),
  };

  db::audio_file::insert(&file).await?;

  Ok(file)
}

pub async fn upload_audio_file<E: Error, S: Stream<Item = Result<Bytes, E>>>(
  account_id: String,
  audio_file_id: Option<String>,
  size_limit: usize,
  data: S,
) -> Result<AudioFile, UploadError<E>> {
  let audio_file_id = audio_file_id.unwrap_or_else(|| db::audio_file::uid());

  let mut operation = AudioUploadOperation {
    id: audio_file_id.clone(),
    account_id: account_id.clone(),
    created_at: Utc::now(),
    state: db::audio_upload_operation::State::Pending,
  };

  db::audio_upload_operation::insert(&operation).await?;

  let result = upload_audio_file_internal(account_id, Some(audio_file_id), size_limit, data).await;

  match result.as_ref() {
    Ok(_) => {
      operation.state = State::Success {
        commited_at: Utc::now(),
      };

      let r = db::audio_upload_operation::replace(&operation.id, &operation).await;
      match r {
        Err(e) => warn!("error updating audio upload operation after success: {}", e),
        _ => {}
      }
    }

    Err(e) => {
      operation.state = State::Error {
        cancelled_at: Utc::now(),
        error: format!("{}", e),
        error_debug: format!("{:#?}", e),
      };

      let r = db::audio_upload_operation::replace(&operation.id, &operation).await;
      match r {
        Err(e) => warn!("error updating audio upload operation after error: {}", e),
        _ => {}
      }
    }
  }

  result
}

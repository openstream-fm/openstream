use std::process::ExitStatus;

use bytes::Bytes;
use constants::{AUDIO_FILE_BYTERATE, AUDIO_FILE_CHUNK_SIZE};
use db::audio_chunk::AudioChunk;
use db::audio_file::{AudioFile, Metadata};
use db::audio_upload_operation::{AudioUploadOperation, State};
use db::Model;
use ffmpeg::{transform, FfmpegConfig, TransformError};
use log::*;
use md5::{Digest, Md5};
use serde_util::DateTime;
use std::error::Error;
use tokio_stream::{Stream, StreamExt};

#[derive(Debug, thiserror::Error)]
pub enum UploadError<E> {
  #[error("stream: {0}")]
  Stream(E),
  #[error("mongo: {0}")]
  Mongo(#[from] mongodb::error::Error),
  #[error("ffmpeg spawn io: {0}")]
  FfmpegSpawn(std::io::Error),
  #[error("ffmpeg exit: status: {status}, stderr: {stderr:?}")]
  FfmpegExit {
    status: ExitStatus,
    stderr: Option<String>,
  },
  #[error("ffmpeg io: {0}")]
  FfmpegIo(std::io::Error),
  #[error("size exceeded")]
  SizeExceeded,
  #[error("file empty")]
  Empty,
}

impl<E> From<TransformError> for UploadError<E> {
  fn from(e: TransformError) -> Self {
    match e {
      TransformError::Io(e) => UploadError::FfmpegIo(e),
      TransformError::Exit { status, stderr } => UploadError::FfmpegExit { status, stderr },
    }
  }
}

async fn upload_audio_file_internal<E: Error, S: Stream<Item = Result<Bytes, E>>>(
  account_id: String,
  audio_file_id: String,
  size_limit: usize,
  filename: String,
  data: S,
) -> Result<AudioFile, UploadError<E>> {
  tokio::pin!(data);

  let (meta_tx, meta_rx) = tokio::sync::mpsc::channel(1);
  let meta_get = ffmpeg::metadata::get(meta_rx);

  let config = FfmpegConfig {
    format: ffmpeg::Format::MP3,
    kbitrate: AUDIO_FILE_BYTERATE * 8 / 1000,
    ..Default::default()
  };

  let (writer, mut reader) =
    transform(config, AUDIO_FILE_CHUNK_SIZE).map_err(UploadError::FfmpegSpawn)?;

  let writer_f = async move {
    loop {
      trace!("upload writer recv loop");
      let item = data.next().await;
      match item {
        None => {
          trace!("upload writer recv end");
          break;
        }
        Some(Err(e)) => {
          trace!("upload writer recv error: {:?}", e);
          return Err(UploadError::Stream(e));
        }
        Some(Ok(bytes)) => {
          let len = bytes.len();
          trace!("upload writer recv item: {len} bytes");
          let _ = meta_tx.send(bytes.clone()).await;
          match writer.send(bytes).await {
            Ok(()) => {
              trace!("upload writer send item: {len} bytes");
              continue;
            }
            Err(_e) => {
              trace!("upload writer send error: SendError");
              break;
            }
          }
        }
      }
    }

    Ok(())
  };

  let reader_f = {
    let audio_file_id = audio_file_id.clone();
    let account_id = account_id.clone();

    async move {
      let mut hasher = Md5::new();

      let mut file_len = 0;
      let mut file_duration_ms = 0.0;
      let mut chunk_count = 0;

      loop {
        let bytes = match reader.recv().await {
          None => {
            trace!("upload reader recv end");
            break;
          }
          Some(Err(e)) => {
            trace!("upload reader recv error: {:?}", e);
            return Err(e.into());
          }
          Some(Ok(bytes)) => {
            trace!("upload reader recv item: {} bytes", bytes.len());
            bytes
          }
        };

        hasher.update(bytes.as_ref());

        let i = chunk_count;
        chunk_count += 1;

        let len = bytes.len();
        file_len += len;

        if file_len > size_limit {
          trace!("upload error size exceeded");
          return Err(UploadError::SizeExceeded);
        }

        let duration_ms = bytes.len() as f64 / AUDIO_FILE_BYTERATE as f64 * 1000.0;

        let start_ms = file_duration_ms;
        file_duration_ms += duration_ms;

        let end_ms = start_ms + duration_ms;

        let document = AudioChunk {
          id: AudioChunk::uid(),
          account_id: account_id.clone(),
          audio_file_id: audio_file_id.clone(),
          duration_ms,
          start_ms,
          end_ms,
          i,
          len,
          bytes_sec: AUDIO_FILE_BYTERATE,
          data: bytes,
          created_at: DateTime::now(),
        };

        AudioChunk::insert(&document).await?;
        trace!("upload audio chunk #{i} inserted");
      }

      let md5_array = hasher.finalize();
      let md5 = hex::encode(md5_array);

      Ok((file_len, file_duration_ms, chunk_count, md5))
    }
  };

  let (meta_get, write, read) = tokio::join!(meta_get, writer_f, reader_f);
  write?;
  let (file_len, file_duration_ms, chunk_count, md5) = read?;

  if file_len == 0 {
    return Err(UploadError::Empty);
  }

  let metadata = match meta_get {
    Err(e) => {
      warn!("upload metadata error: {} => {:?}", e, e);
      Metadata::default()
    }
    Ok(map) => Metadata::from(map.into_iter()),
  };

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
    created_at: DateTime::now(),
    filename,
    metadata,
  };

  AudioFile::insert(&file).await?;
  trace!("upload audio file uploaded");

  Ok(file)
}

pub async fn upload_audio_file<E: Error, S: Stream<Item = Result<Bytes, E>>>(
  account_id: String,
  audio_file_id: Option<String>,
  size_limit: usize,
  filename: String,
  data: S,
) -> Result<AudioFile, UploadError<E>> {
  let audio_file_id = audio_file_id.unwrap_or_else(AudioFile::uid);

  let mut operation = AudioUploadOperation {
    id: audio_file_id.clone(),
    account_id: account_id.clone(),
    created_at: DateTime::now(),
    state: db::audio_upload_operation::State::Pending,
  };

  AudioUploadOperation::insert(&operation).await?;

  let result =
    upload_audio_file_internal(account_id, audio_file_id, size_limit, filename, data).await;

  match result.as_ref() {
    Ok(_) => {
      operation.state = State::Success {
        commited_at: DateTime::now(),
      };

      let r = AudioUploadOperation::replace(&operation.id, &operation).await;
      if let Err(e) = r {
        warn!(
          "error updating audio upload operation after success: {} => {:?}",
          &e, &e
        )
      }
    }

    Err(e) => {
      operation.state = State::Error {
        cancelled_at: DateTime::now(),
        error: format!("{}", e),
        error_debug: format!("{:?}", e),
      };

      let r = AudioUploadOperation::replace(&operation.id, &operation).await;
      if let Err(e) = r {
        warn!(
          "error updating audio upload operation after error: {} => {:?}",
          &e, &e
        )
      }
    }
  }

  result
}

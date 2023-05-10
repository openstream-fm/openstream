#![allow(clippy::uninlined_format_args)]

use std::process::ExitStatus;

use bytes::Bytes;
use constants::{AUDIO_FILE_BYTERATE, AUDIO_FILE_CHUNK_SIZE};
use db::account::Account;
use db::audio_chunk::AudioChunk;
use db::audio_file::{AudioFile, Metadata};
use db::audio_upload_operation::{AudioUploadOperation, State};
use db::station::Station;
use db::{run_transaction, storage_quota, Model};
use ffmpeg::{transform, FfmpegConfig, TransformError};
use log::*;
use serde_util::DateTime;
use sha2::{Digest, Sha256};
use std::error::Error;
use tokio_stream::{Stream, StreamExt};

macro_rules! check_quota {
  ($station_id:expr, $file_len:expr) => {
    match storage_quota!($station_id) {
      None => {
        trace!("upload error station not found: {}", $station_id);
        return Err(UploadError::StationNotFound($station_id.to_string()));
      }

      Some(max) => {
        if $file_len > max {
          trace!("upload error quota exceeded (1)");
          return Err(UploadError::QuotaExceeded);
        }
      }
    }
  };
}

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
  #[error("station not found: {0}")]
  StationNotFound(String),
  #[error("account not found: {0}")]
  AccountNotFound(String),
  #[error("quota exceeded")]
  QuotaExceeded,
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
  station_id: String,
  audio_file_id: String,
  estimated_len: Option<u64>,
  filename: String,
  data: S,
) -> Result<AudioFile, UploadError<E>> {
  let station = match Station::get_by_id(&station_id).await? {
    Some(station) => station,
    None => return Err(UploadError::StationNotFound(station_id)),
  };

  let account = match Account::get_by_id(&station.account_id).await? {
    Some(account) => account,
    None => return Err(UploadError::AccountNotFound(station_id)),
  };

  if let Some(len) = estimated_len {
    check_quota!(&account.id, len);
  }

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
    let station_id = station_id.clone();

    async move {
      let mut hasher = Sha256::new();

      let mut file_len = 0u64;
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
        file_len += len as u64;

        check_quota!(&account.id, file_len);

        let duration_ms = bytes.len() as f64 / AUDIO_FILE_BYTERATE as f64 * 1000.0;

        let start_ms = file_duration_ms;
        file_duration_ms += duration_ms;

        let end_ms = start_ms + duration_ms;

        let document = AudioChunk {
          id: AudioChunk::uid(),
          station_id: station_id.clone(),
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

      let sha256_array = hasher.finalize();
      let sha256 = hex::encode(sha256_array);

      Ok((file_len, file_duration_ms, chunk_count, sha256))
    }
  };

  let (meta_get, write, read) = tokio::join!(meta_get, writer_f, reader_f);
  write?;
  let (file_len, file_duration_ms, chunk_count, sha256) = read?;

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

  let order = AudioFile::next_max_order(&station_id, None).await?;

  let file = AudioFile {
    id: audio_file_id,
    station_id,
    sha256,
    len: file_len,
    duration_ms: file_duration_ms,
    chunk_count,
    chunk_len: AUDIO_FILE_CHUNK_SIZE,
    chunk_duration_ms: AUDIO_FILE_CHUNK_SIZE as f64 / AUDIO_FILE_BYTERATE as f64 * 1000.0,
    bytes_sec: AUDIO_FILE_BYTERATE,
    filename,
    metadata,
    order,
    created_at: DateTime::now(),
  };

  run_transaction!(session => {

    let station = match tx_try!(Station::get_by_id_with_session(&file.station_id, &mut session).await) {
      None => return Err(UploadError::StationNotFound(file.station_id)),
      Some(station) => station,
    };

    let mut account = match tx_try!(Account::get_by_id_with_session(&station.account_id, &mut session).await) {
      None => return Err(UploadError::AccountNotFound(station.account_id)),
      Some(account) => account,
    };

    if account.limits.storage.avail() < file.len {
      return Err(UploadError::QuotaExceeded);
    }

    account.limits.storage.used += file.len;

    tx_try!(Account::replace_with_session(&account.id, &account, &mut session).await);
    tx_try!(AudioFile::insert_with_session(&file, &mut session).await);
    trace!("audio file uploaded station_id={}, audio_file_id={}", station.id, file.id);
  });

  Ok(file)
}

async fn upload_audio_file_inner_spawn<
  E: Error + Send + Sync + 'static,
  S: Stream<Item = Result<Bytes, E>> + Send + 'static,
>(
  deployment_id: String,
  station_id: String,
  audio_file_id: Option<String>,
  estimated_len: Option<u64>,
  filename: String,
  data: S,
) -> Result<AudioFile, UploadError<E>> {
  let audio_file_id = audio_file_id.unwrap_or_else(AudioFile::uid);

  let mut operation = AudioUploadOperation {
    id: audio_file_id.clone(),
    deployment_id,
    station_id: station_id.clone(),
    created_at: DateTime::now(),
    state: db::audio_upload_operation::State::Pending,
  };

  AudioUploadOperation::insert(&operation).await?;

  let result =
    upload_audio_file_internal(station_id, audio_file_id, estimated_len, filename, data).await;

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

    Err(upload_error) => {
      operation.state = State::Error {
        cancelled_at: DateTime::now(),
        error_display: format!("{}", upload_error),
        error_debug: format!("{:?}", upload_error),
      };

      let r = AudioUploadOperation::replace(&operation.id, &operation).await;
      if let Err(e) = r {
        warn!(
          "error updating audio upload operation after error: {} => {:?}",
          &e, &e
        )
      }

      let r = AudioUploadOperation::clean_up_chunks_after_error(&operation.id).await;
      if let Err(e) = r {
        warn!(
          "error cleaning up chunks of upload operation after error: {} => {:?}",
          &e, &e
        )
      }
    }
  }

  result
}

pub async fn upload_audio_file<
  E: Error + Send + Sync + 'static,
  S: Stream<Item = Result<Bytes, E>> + Send + 'static,
>(
  deployment_id: String,
  station_id: String,
  audio_file_id: Option<String>,
  estimated_len: Option<u64>,
  filename: String,
  data: S,
) -> Result<AudioFile, UploadError<E>> {
  tokio::spawn(upload_audio_file_inner_spawn(
    deployment_id,
    station_id,
    audio_file_id,
    estimated_len,
    filename,
    data,
  ))
  .await
  .unwrap()
}

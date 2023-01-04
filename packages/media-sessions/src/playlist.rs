use constants::STREAM_CHUNK_SIZE;
use db::{audio_chunk::AudioChunk, audio_file::AudioFile, Model};
use log::info;

use crate::{SendError, Transmitter};
use futures_util::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOneOptions};
use shutdown::Shutdown;
use stream_util::{IntoTryBytesStreamChunked, IntoTryBytesStreamRated};

pub fn run_playlist_session(
  tx: Transmitter,
  shutdown: Shutdown,
) -> tokio::task::JoinHandle<Result<(), mongodb::error::Error>> {
  tokio::spawn(async move {
    let account_id = tx.info.station_id.as_str();
    let filter = doc! { AudioFile::KEY_ACCOUNT_ID: account_id };

    let mut skip: u64 = 0;

    info!("media session (playlist) start for account {account_id}");

    'files: loop {
      if shutdown.is_closed() {
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

      let stream = AudioChunk::stream(&file.id);
      let stream = stream.chunked(STREAM_CHUNK_SIZE).rated(file.bytes_sec);
      tokio::pin!(stream);

      'chunks: loop {
        if shutdown.is_closed() {
          return Ok(());
        }

        match stream.try_next().await? {
          None => break 'chunks,

          Some(bytes) => {
            if shutdown.is_closed() {
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

    Ok(())
  })
}

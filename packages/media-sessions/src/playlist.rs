use constants::STREAM_CHUNK_SIZE;
use db::{audio_chunk::AudioChunk, audio_file::AudioFile, Model};

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
    let filter = doc! { "accountId": account_id };

    let mut skip: u64 = 0;

    'files: loop {
      if shutdown.is_closed() {
        return Ok(());
      }

      let options = FindOneOptions::builder().skip(skip).build();
      let file = AudioFile::cl().find_one(filter.clone(), options).await?;
      let file = match file {
        Some(file) => file,
        None => {
          if skip == 0 {
            return Ok(());
          } else {
            skip = 0;
            continue;
          }
        }
      };

      skip += 1;

      let stream = AudioChunk::stream(&file.id);
      let stream = stream.chunked(STREAM_CHUNK_SIZE).rated(file.bytes_sec);
      tokio::pin!(stream);

      #[allow(unused_labels)]
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

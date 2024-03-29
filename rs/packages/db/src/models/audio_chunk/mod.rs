use crate::{storage_db, Model};
use async_stream::try_stream;
use bytes::Bytes;
use futures_util::stream::Stream;
use mongodb::results::DeleteResult;
use mongodb::ClientSession;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::as_f64;
use serde_util::DateTime;
use ts_rs::TS;

crate::register!(AudioChunk);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct AudioChunk {
  #[serde(rename = "_id")]
  pub id: String,
  pub audio_file_id: String,
  pub station_id: String,

  pub start_ms: f64,
  pub end_ms: f64,

  #[serde(with = "as_f64")]
  pub i: usize,

  #[serde(with = "as_f64")]
  pub len: usize,

  pub duration_ms: f64,

  #[serde(with = "as_f64")]
  pub bytes_sec: usize,

  #[serde(with = "serde_util::bytes")]
  #[ts(type = "string")]
  pub data: Bytes,

  pub created_at: DateTime,
}

impl AudioChunk {
  pub async fn delete_by_audio_file_id_with_session(
    audio_file_id: &str,
    session: &mut ClientSession,
  ) -> Result<DeleteResult, mongodb::error::Error> {
    let filter = doc! { Self::KEY_AUDIO_FILE_ID: audio_file_id };
    let r = Self::cl()
      .delete_many_with_session(filter, None, session)
      .await?;

    Ok(r)
  }

  pub fn stream(
    file_id: &str,
  ) -> impl Stream<Item = Result<Bytes, mongodb::error::Error>> + Send + 'static {
    Self::stream_from(file_id, 0.0)
  }

  pub fn stream_from(
    file_id: &str,
    skip: f64,
  ) -> impl Stream<Item = Result<Bytes, mongodb::error::Error>> + Send + 'static {
    let file_id = file_id.to_string();
    try_stream! {
      let mut i = skip;
      loop {
        let filter = doc!{ Self::KEY_AUDIO_FILE_ID: &file_id, Self::KEY_I: i };
        let item = match Self::get(filter).await? {
          Some(item) => item,
          None => break,
        };

        yield item.data;
        i += 1.0;
      };
    }
  }
}

impl Model for AudioChunk {
  const UID_LEN: usize = 16;
  const CL_NAME: &'static str = "audio_chunks";

  fn db() -> mongodb::Database {
    storage_db()
  }

  fn indexes() -> Vec<IndexModel> {
    let station_id = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1 })
      .build();
    let audio_file_id = IndexModel::builder()
      .keys(doc! { Self::KEY_AUDIO_FILE_ID: 1 })
      .build();

    let unique = IndexOptions::builder().unique(true).build();
    let audio_file_id_with_index = IndexModel::builder()
      .keys(doc! { Self::KEY_AUDIO_FILE_ID: 1, Self::KEY_I: 1 })
      .options(unique)
      .build();

    vec![station_id, audio_file_id, audio_file_id_with_index]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, AudioChunk::KEY_ID);
  }
}

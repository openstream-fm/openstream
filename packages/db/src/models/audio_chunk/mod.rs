use crate::Model;
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(rename_all = "camelCase")]
pub struct AudioChunk {
  #[serde(rename = "_id")]
  pub id: String,
  pub audio_file_id: String,
  pub account_id: String,

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
    let filter = doc! { "audioFileId": audio_file_id };
    let r = Self::cl()
      .delete_many_with_session(filter, None, session)
      .await?;

    Ok(r)
  }

  pub fn stream(
    file_id: &str,
  ) -> impl Stream<Item = Result<Bytes, mongodb::error::Error>> + Send + 'static {
    let file_id = file_id.to_string();
    try_stream! {
      let mut i = 0.0;
      loop {
        let filter = doc!{ "audioFileId": &file_id, "i": i };
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

  fn indexes() -> Vec<IndexModel> {
    let account_id = IndexModel::builder().keys(doc! { "accountId": 1 }).build();
    let audio_file_id = IndexModel::builder()
      .keys(doc! { "audioFileId": 1 })
      .build();

    let unique = IndexOptions::builder().unique(true).build();
    let audio_file_id_with_index = IndexModel::builder()
      .keys(doc! { "audioFileId": 1, "i": 1 })
      .options(unique)
      .build();

    vec![account_id, audio_file_id, audio_file_id_with_index]
  }
}

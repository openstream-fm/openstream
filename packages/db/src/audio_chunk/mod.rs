use crate::Model;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::as_f64;
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
  /// ts: base64 bytes
  pub data: Bytes,

  #[serde(with = "serde_util::datetime")]
  /// ts: ISODate
  pub created_at: DateTime<Utc>,
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

use crate::Model;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::as_f64;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
  pub data: Bytes,

  #[serde(with = "serde_util::datetime")]
  pub created_at: DateTime<Utc>,
}

impl Model for AudioChunk {
  fn uid_len() -> usize {
    16
  }

  fn cl_name() -> &'static str {
    "audio_chunks"
  }

  fn indexes() -> Vec<IndexModel> {
    let account_id = IndexModel::builder().keys(doc! { "accountId": 1 }).build();
    let audio_file_id = IndexModel::builder()
      .keys(doc! { "audio_file_id": 1 })
      .build();

    vec![account_id, audio_file_id]
  }
}

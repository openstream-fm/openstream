use crate::model;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_util::as_f64;

pub const CL_NAME: &str = "audio_chunks";
pub const UID_LEN: usize = 16;

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

model!(AudioChunk);

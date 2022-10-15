use crate::model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_util::as_f64;

pub const CL_NAME: &str = "audio_file";
pub const UID_LEN: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFile {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,
  pub md5: String,

  #[serde(with = "as_f64")]
  pub len: usize,
  pub duration_ms: f64,
  #[serde(with = "as_f64")]
  pub bytes_sec: usize,

  #[serde(with = "as_f64")]
  pub chunk_count: usize,
  #[serde(with = "as_f64")]
  pub chunk_len: usize,

  pub chunk_duration_ms: f64,

  #[serde(with = "serde_util::datetime")]
  pub created_at: DateTime<Utc>,
}

model!(AudioFile);

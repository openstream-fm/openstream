use crate::model;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_util::as_f64;

pub const CL_NAME: &str = "audio_chunks";
pub const UID_LEN: usize = 16;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioChunk {
  #[serde(rename = "_id")]
  id: String,
  audio_file_id: String,
  account_id: String,

  start_time: f64,
  end_time: f64,

  #[serde(with = "as_f64")]
  i: usize,

  #[serde(with = "as_f64")]
  size: usize,
  duration: f64,
  #[serde(with = "as_f64")]
  rate: usize,

  #[serde(with = "serde_util::bytes")]
  data: Bytes,
}

model!(AudioChunk);

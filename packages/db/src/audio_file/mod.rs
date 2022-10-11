use crate::model;
use serde::{Deserialize, Serialize};

use serde_util::as_f64;

pub const CL_NAME: &str = "audio_files";
pub const UID_LEN: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFile {
  #[serde(rename = "_id")]
  id: String,
  account_id: String,
  md5: String,

  #[serde(with = "as_f64")]
  size: usize,
  duration: f64,
  #[serde(with = "as_f64")]
  rate: usize,

  #[serde(with = "as_f64")]
  chunks: usize,

  #[serde(with = "as_f64")]
  chunk_size: usize,

  chunk_duration: f64,
}

model!(AudioFile);

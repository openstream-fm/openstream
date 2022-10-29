use crate::Model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_util::datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  #[serde(rename = "_id")]
  id: String,
  name: String,
  email: String,
  password: Option<String>,
  #[serde(with = "datetime")]
  created_at: DateTime<Utc>,
  #[serde(with = "datetime")]
  updated_at: DateTime<Utc>,
}

impl Model for Account {
  fn uid_len() -> usize {
    8
  }

  fn cl_name() -> &'static str {
    "accounts"
  }
}

use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use serde_util::datetime;

use crate::model;

pub const CL_NAME: &str = "accounts";
pub const UID_LEN: usize = 8;

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

model!(Account);

use crate::metadata::Metadata;
use crate::Model;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  #[serde(rename = "_id")]
  id: String,
  name: String,
  owner_id: String,
  #[serde(with = "datetime")]
  created_at: DateTime<Utc>,
  #[serde(with = "datetime")]
  updated_at: DateTime<Utc>,
  user_metadata: Metadata,
  system_metadata: Metadata,
}

impl Model for Account {
  fn uid_len() -> usize {
    8
  }

  fn cl_name() -> &'static str {
    "accounts"
  }

  fn indexes() -> Vec<IndexModel> {
    let owner_id = IndexModel::builder().keys(doc! { "ownerId": 1 }).build();
    vec![owner_id]
  }
}

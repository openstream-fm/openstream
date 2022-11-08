use crate::metadata::Metadata;
use crate::Model;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
  #[serde(rename = "_id")]
  id: String,
  account_ids: Vec<String>,
  name: String,
  email: String,
  password: Option<String>,
  #[serde(with = "datetime")]
  created_at: DateTime<Utc>,
  #[serde(with = "datetime")]
  updated_at: DateTime<Utc>,
  user_metadata: Metadata,
  system_metadata: Metadata,
}

impl Model for User {
  fn uid_len() -> usize {
    6
  }

  fn cl_name() -> &'static str {
    "users"
  }

  fn indexes() -> Vec<IndexModel> {
    let account_ids = IndexModel::builder().keys(doc! { "accountIds": 1 }).build();
    let email_opts = IndexOptions::builder().unique(true).build();
    let email = IndexModel::builder()
      .keys(doc! { "email": 1 })
      .options(email_opts)
      .build();

    vec![account_ids, email]
  }
}

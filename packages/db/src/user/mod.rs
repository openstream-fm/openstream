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
  pub id: String,
  pub account_ids: Vec<String>,
  pub name: String,
  pub email: String,
  pub password: Option<String>,
  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "datetime")]
  pub updated_at: DateTime<Utc>,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
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

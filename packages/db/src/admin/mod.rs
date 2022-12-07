use crate::metadata::Metadata;
use crate::Model;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::datetime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(rename_all = "camelCase")]
pub struct Admin {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub email: String,

  pub password: Option<String>,

  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,

  #[serde(with = "datetime")]
  pub updated_at: DateTime<Utc>,

  pub system_metadata: Metadata,
}

impl Model for Admin {
  const UID_LEN: usize = 6;
  const CL_NAME: &'static str = "admins";

  fn indexes() -> Vec<IndexModel> {
    let email_opts = IndexOptions::builder().unique(true).build();
    let email = IndexModel::builder()
      .keys(doc! { "email": 1 })
      .options(email_opts)
      .build();

    vec![email]
  }
}

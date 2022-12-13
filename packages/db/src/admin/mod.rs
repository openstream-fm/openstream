use crate::metadata::Metadata;
use crate::Model;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
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
  pub created_at: DateTime,
  pub updated_at: DateTime,
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

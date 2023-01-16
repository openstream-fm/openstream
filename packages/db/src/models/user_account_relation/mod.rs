use mongodb::IndexModel;
use mongodb::{bson::doc, options::IndexOptions};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use crate::Model;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[macros::keys]
pub struct UserAccountRelation {
  #[serde(rename = "_id")]
  pub id: String,
  pub user_id: String,
  pub account_id: String,
  pub kind: UserAccountRelationKind,
  pub created_at: DateTime,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
pub enum UserAccountRelationKind {
  #[serde(rename = "owner")]
  Owner,
}

impl UserAccountRelationKind {
  pub const TAG_OWNER: &str = "owner";
}

impl Model for UserAccountRelation {
  const CL_NAME: &'static str = "user_account_relations";
  const UID_LEN: usize = 8;

  fn indexes() -> Vec<IndexModel> {
    let user_id = IndexModel::builder()
      .keys(doc! {
        UserAccountRelation::KEY_USER_ID: 1,
      })
      .build();

    let account_id = IndexModel::builder()
      .keys(doc! {
        UserAccountRelation::KEY_ACCOUNT_ID: 1,
      })
      .build();

    let opts = IndexOptions::builder().unique(true).build();
    let user_account = IndexModel::builder()
      .keys(doc! {
        UserAccountRelation::KEY_USER_ID: 1,
        UserAccountRelation::KEY_ACCOUNT_ID: 1,
      })
      .options(opts)
      .build();

    vec![user_id, account_id, user_account]
  }
}

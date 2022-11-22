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
  pub id: String,
  pub name: String,
  pub owner_id: String, // user
  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "datetime")]
  pub updated_at: DateTime<Utc>,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPublicAccount {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub owner_id: String, // user
  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "datetime")]
  pub updated_at: DateTime<Utc>,
  pub user_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublicAccount {
  Admin(Account),
  User(UserPublicAccount),
}

impl Account {
  pub fn into_public(self, is_admin_scope: bool) -> PublicAccount {
    match is_admin_scope {
      true => PublicAccount::Admin(self),
      false => PublicAccount::User(UserPublicAccount {
        id: self.id,
        name: self.name,
        owner_id: self.owner_id,
        created_at: self.created_at,
        updated_at: self.updated_at,
        user_metadata: self.user_metadata,
      }),
    }
  }
}

impl Model for Account {
  const UID_LEN: usize = 8;
  const CL_NAME: &'static str = "accounts";

  fn indexes() -> Vec<IndexModel> {
    let owner_id = IndexModel::builder().keys(doc! { "ownerId": 1 }).build();
    vec![owner_id]
  }
}

use crate::Model;
use crate::{metadata::Metadata, PublicScope};
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

impl From<Account> for UserPublicAccount {
  fn from(account: Account) -> Self {
    Self {
      id: account.id,
      name: account.name,
      owner_id: account.owner_id,
      created_at: account.created_at,
      updated_at: account.updated_at,
      user_metadata: account.user_metadata,
    }
  }
}

impl Account {
  pub fn into_public(self, scope: PublicScope) -> PublicAccount {
    match scope {
      PublicScope::Admin => PublicAccount::Admin(self),
      PublicScope::User => PublicAccount::User(self.into()),
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

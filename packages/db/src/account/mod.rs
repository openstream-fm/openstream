use crate::Model;
use crate::{metadata::Metadata, PublicScope};
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::datetime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(rename_all = "camelCase")]
pub struct Account {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  /// uid of User
  pub owner_id: String,
  pub limits: Limits,

  #[serde(with = "datetime")]
  /// ts: ISODate
  pub created_at: DateTime<Utc>,

  #[serde(with = "datetime")]
  /// ts: ISODate
  pub updated_at: DateTime<Utc>,

  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/")]
#[serde(rename_all = "camelCase")]
pub struct UserPublicAccount {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub owner_id: String, // user
  pub limits: Limits,

  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,

  #[serde(with = "datetime")]
  pub updated_at: DateTime<Utc>,

  pub user_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/")]
pub struct AdminPublicAccount(Account);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/")]
#[serde(untagged)]
pub enum PublicAccount {
  Admin(AdminPublicAccount),
  User(UserPublicAccount),
}

impl From<Account> for UserPublicAccount {
  fn from(account: Account) -> Self {
    Self {
      id: account.id,
      name: account.name,
      owner_id: account.owner_id,
      limits: account.limits,
      created_at: account.created_at,
      updated_at: account.updated_at,
      user_metadata: account.user_metadata,
    }
  }
}

impl From<Account> for AdminPublicAccount {
  fn from(account: Account) -> Self {
    Self(account)
  }
}

impl Account {
  pub fn into_public(self, scope: PublicScope) -> PublicAccount {
    match scope {
      PublicScope::Admin => PublicAccount::Admin(self.into()),
      PublicScope::User => PublicAccount::User(self.into()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/")]
#[ts(rename = "AccountLimits")]
#[serde(rename_all = "camelCase")]
pub struct Limits {
  pub listeners: Limit,
  pub transfer: Limit,
  pub storage: Limit,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/")]
#[ts(rename = "AccountLimit")]
#[serde(rename_all = "camelCase")]
pub struct Limit {
  #[serde(with = "serde_util::as_f64")]
  #[ts(type = "number")]
  pub used: u64,
  #[serde(with = "serde_util::as_f64")]
  #[ts(type = "number")]
  pub avail: u64,
}

impl Model for Account {
  const UID_LEN: usize = 8;
  const CL_NAME: &'static str = "accounts";

  fn indexes() -> Vec<IndexModel> {
    let owner_id = IndexModel::builder().keys(doc! { "ownerId": 1 }).build();
    vec![owner_id]
  }
}

use crate::audio_file::AudioFile;
use crate::station::Station;
use crate::stream_connection::lite::StreamConnectionLite;
use crate::Model;
use crate::{metadata::Metadata, PublicScope};
use constants::validate::*;
use mongodb::bson::{doc, Bson};
use mongodb::ClientSession;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::collections::HashMap;
use ts_rs::TS;

use modify::Modify;
use validator::Validate;

crate::register!(Account);

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Account {
  #[serde(rename = "_id")]
  pub id: String,
  pub plan_id: String,
  pub payment_method_id: Option<String>,
  pub name: String,
  pub limits: Limits,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "snake_case")]
pub struct UserPublicAccount {
  #[serde(rename = "_id")]
  pub id: String,
  pub plan_id: String,
  pub payment_method_id: Option<String>,
  pub name: String,
  pub limits: Limits,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub user_metadata: Metadata,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/")]
pub struct AdminPublicAccount(pub Account);

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/")]
#[serde(untagged)]
pub enum PublicAccount {
  Admin(AdminPublicAccount),
  User(UserPublicAccount),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
#[ts(export, export_to = "../../../defs/ops/")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct AccountPatch {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[modify(trim)]
  #[validate(
    length(
      min = "VALIDATE_ACCOUNT_NAME_MIN_LEN",
      max = "VALIDATE_ACCOUNT_NAME_MAX_LEN",
      message = "Account name is either too short or too long",
    ),
    non_control_character(message = "Account name cannot have control characters")
  )]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub plan_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_metadata: Option<Metadata>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_metadata: Option<Metadata>,
}

impl Account {
  pub async fn increment_used_transfer(
    id: &str,
    size: usize,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    const KEY: &str = crate::key!(Account::KEY_LIMITS, Limits::KEY_TRANSFER, Limit::KEY_USED);
    Self::update_by_id(id, doc! { "$inc": { KEY: size as f64 } }).await
  }

  pub async fn increment_used_listeners(
    id: &str,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    const KEY: &str = crate::key!(Account::KEY_LIMITS, Limits::KEY_LISTENERS, Limit::KEY_USED);
    let update = doc! { "$inc": { KEY: 1.0_f64 } };
    Self::update_by_id(id, update).await
  }

  pub async fn decrement_used_listeners(
    id: &str,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    const KEY: &str = crate::key!(Account::KEY_LIMITS, Limits::KEY_LISTENERS, Limit::KEY_USED);
    let update = doc! { "$inc": { KEY: -1.0_f64 } };
    Self::update_by_id(id, update).await
  }
}

impl From<Account> for UserPublicAccount {
  fn from(account: Account) -> Self {
    Self {
      id: account.id,
      plan_id: account.plan_id,
      payment_method_id: account.payment_method_id,
      name: account.name,
      limits: account.limits,
      created_at: account.created_at,
      updated_at: account.updated_at,
      user_metadata: account.user_metadata,
      deleted_at: account.deleted_at,
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

impl Model for Account {
  const UID_LEN: usize = 8;
  const CL_NAME: &'static str = "accounts";
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/", rename = "AccountLimits")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Limits {
  pub stations: Limit,
  pub listeners: Limit,
  pub transfer: Limit,
  pub storage: Limit,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/", rename = "AccountLimit")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Limit {
  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub used: u64,
  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub total: u64,
}

impl Limit {
  pub fn avail(&self) -> u64 {
    self.total.saturating_sub(self.used)
  }
}

pub async fn recalculate_used_listeners_quota(
  session: &mut ClientSession,
) -> Result<(), mongodb::error::Error> {
  let account_ids = Account::cl()
    .distinct_with_session(Account::KEY_ID, None, None, session)
    .await?;
  let mut account_counters = account_ids
    .into_iter()
    .map(|bson| match bson {
      Bson::String(v) => (v, 0),
      _ => unreachable!(),
    })
    .collect::<HashMap<String, u64>>();

  let mut station_account_map = HashMap::<String, String>::new();
  {
    let mut stations = Station::cl().find_with_session(None, None, session).await?;
    while let Some(station) = stations.next(session).await.transpose()? {
      station_account_map.insert(station.id.clone(), station.account_id.clone());
    }
  }

  let filter = doc! { StreamConnectionLite::KEY_IS_OPEN: true };
  let mut conns = StreamConnectionLite::cl()
    .find_with_session(filter, None, session)
    .await?;

  while let Some(conn) = conns.next(session).await.transpose()? {
    let account_id = station_account_map.get(&conn.station_id);
    if let Some(account_id) = account_id {
      *account_counters.entry(account_id.to_string()).or_insert(0) += 1;
    }
  }

  for (account_id, v) in account_counters.into_iter() {
    const KEY_LIMITS_LISTENERS_USED: &str =
      crate::key!(Account::KEY_LIMITS, Limits::KEY_LISTENERS, Limit::KEY_USED);

    let update = doc! { "$set": { KEY_LIMITS_LISTENERS_USED: v as f64 } };

    Account::update_by_id_with_session(&account_id, update, session).await?;
  }

  Ok(())
}

pub async fn recalculate_storage_quota(
  session: &mut ClientSession,
) -> Result<(), mongodb::error::Error> {
  let mut stations = Station::cl().find_with_session(None, None, session).await?;
  let mut station_account_map = HashMap::<String, String>::new();
  {
    while let Some(station) = stations.next(session).await.transpose()? {
      station_account_map.insert(station.id.clone(), station.account_id.clone());
    }
  }

  let mut account_used_map = HashMap::<String, u64>::new();

  let mut files = AudioFile::cl()
    .find_with_session(None, None, session)
    .await?;

  while let Some(file) = files.next(session).await.transpose()? {
    let account_id = station_account_map.get(&file.station_id);
    if let Some(account_id) = account_id {
      *account_used_map.entry(account_id.to_string()).or_insert(0) += file.len;
    }
  }

  const KEY_LIMITS_STORAGE_USED: &str =
    crate::key!(Account::KEY_LIMITS, Limits::KEY_STORAGE, Limit::KEY_USED);

  let non_zero_account_ids = account_used_map.keys().cloned().collect::<Vec<_>>();

  let filter = doc! { Account::KEY_ID: { "$nin": &non_zero_account_ids } };

  let update = doc! { "$set": { KEY_LIMITS_STORAGE_USED: 0 as f64 } };

  Account::cl()
    .update_many_with_session(filter, update, None, session)
    .await?;

  for (account_id, v) in account_used_map.into_iter() {
    let update = doc! { "$set": { KEY_LIMITS_STORAGE_USED: v as f64 } };

    Account::update_by_id_with_session(&account_id, update, session).await?;
  }

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, Account::KEY_ID);
  }
}

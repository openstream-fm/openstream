use crate::error::ApplyPatchError;
use crate::Model;
use crate::{metadata::Metadata, PublicScope};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Account {
  #[serde(rename = "_id")]
  pub id: String,
  pub plan_id: String,
  pub name: String,
  pub limits: Limits,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "snake_case")]
pub struct UserPublicAccount {
  #[serde(rename = "_id")]
  pub id: String,
  pub plan_id: String,
  pub name: String,
  pub limits: Limits,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub user_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
pub struct AdminPublicAccount(pub Account);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
#[serde(untagged)]
pub enum PublicAccount {
  Admin(AdminPublicAccount),
  User(UserPublicAccount),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/ops/")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct AccountPatch {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limits: Option<AccountPatchLimits>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_metadata: Option<Metadata>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/ops/")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct AccountPatchLimits {
  #[serde(skip_serializing_if = "Option::is_none")]
  stations: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  listeners: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  storage: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  transfer: Option<u64>,
}

impl Account {
  pub fn apply_patch(
    &mut self,
    patch: AccountPatch,
    scope: PublicScope,
  ) -> Result<(), ApplyPatchError> {
    match scope {
      PublicScope::User => {
        if patch.system_metadata.is_some() || patch.limits.is_some() {
          return Err(ApplyPatchError::out_of_scope(
            "system_metadata field is out of scope",
          ));
        }

        if patch.name.is_none() && patch.user_metadata.is_none() {
          return Err(ApplyPatchError::PatchEmpty);
        }
      }

      PublicScope::Admin => {
        if patch.name.is_none() && patch.user_metadata.is_none() && patch.system_metadata.is_none()
        {
          return Err(ApplyPatchError::PatchEmpty);
        }
      }
    }

    if let Some(ref name) = patch.name {
      let name = name.trim();
      if name.is_empty() {
        return Err(ApplyPatchError::invalid("The name cannot be empty"));
      }

      self.name = name.into();
    }

    if let Some(metadata) = patch.user_metadata {
      self.user_metadata.merge(metadata);
    }

    match scope {
      PublicScope::User => {}
      PublicScope::Admin => {
        if let Some(metadata) = patch.system_metadata {
          self.system_metadata.merge(metadata);
        }

        if let Some(limits) = patch.limits {
          if let Some(stations) = limits.stations {
            self.limits.stations.total = stations;
          }

          if let Some(listeners) = limits.listeners {
            self.limits.listeners.total = listeners;
          }

          if let Some(storage) = limits.storage {
            self.limits.storage.total = storage;
          }

          if let Some(transfer) = limits.transfer {
            self.limits.transfer.total = transfer;
          }
        }
      }
    }

    self.updated_at = DateTime::now();

    Ok(())
  }

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
    log::info!("increment_used_listeners account={id} update: {:?}", update);
    Self::update_by_id(id, update).await
  }

  pub async fn decrement_used_listeners(
    id: &str,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    const KEY: &str = crate::key!(Account::KEY_LIMITS, Limits::KEY_LISTENERS, Limit::KEY_USED);
    let update = doc! { "$inc": { KEY: -1.0_f64 } };
    log::info!("decrement_used_listeners account={id} update: {:?}", update);
    Self::update_by_id(id, update).await
  }
}

impl From<Account> for UserPublicAccount {
  fn from(account: Account) -> Self {
    Self {
      id: account.id,
      plan_id: account.plan_id,
      name: account.name,
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

impl Model for Account {
  const UID_LEN: usize = 8;
  const CL_NAME: &'static str = "accounts";
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/", rename = "AccountLimits")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Limits {
  pub stations: Limit,
  pub listeners: Limit,
  pub transfer: Limit,
  pub storage: Limit,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../defs/", rename = "AccountLimit")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Limit {
  #[serde(with = "serde_util::as_f64")]
  pub used: u64,
  #[serde(with = "serde_util::as_f64")]
  pub total: u64,
}

impl Limit {
  pub fn avail(&self) -> u64 {
    self.total.saturating_sub(self.used)
  }
}
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, Account::KEY_ID);
  }
}

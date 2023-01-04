use crate::error::ApplyPatchError;
use crate::Model;
use crate::{metadata::Metadata, PublicScope};
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[serde(rename_all = "camelCase")]
#[macros::keys]
pub struct Account {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  /// uid of User
  pub owner_id: String,
  pub limits: Limits,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
  pub source_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/")]
#[serde(rename_all = "camelCase")]
pub struct UserPublicAccount {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub owner_id: String, // user
  pub limits: Limits,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub user_metadata: Metadata,
  pub source_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/")]
pub struct AdminPublicAccount(Account);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/")]
#[serde(untagged)]
pub enum PublicAccount {
  Admin(AdminPublicAccount),
  User(UserPublicAccount),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/ops/")]
#[serde(rename_all = "camelCase")]
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
#[ts(export, export_to = "../../defs/ops/")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AccountPatchLimits {
  #[serde(skip_serializing_if = "Option::is_none")]
  storage: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  transfer: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  listeners: Option<u64>,
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
            "Some of the specified fields are out of scope",
          ));
        }

        if patch.name.is_none() && patch.user_metadata.is_none() {
          return Err(ApplyPatchError::PatchEmpty);
        }
      }

      PublicScope::Admin => {
        if patch.name.is_none()
          && patch.user_metadata.is_none()
          && patch.system_metadata.is_none()
          && patch.limits.is_none()
        {
          return Err(ApplyPatchError::PatchEmpty);
        }
      }
    }

    if let Some(ref name) = patch.name {
      let name = name.trim();
      if name.is_empty() {
        return Err(ApplyPatchError::invalid("name cannot be empty"));
      }

      self.name = name.into();
    }

    if let Some(metadata) = patch.user_metadata {
      self.user_metadata.merge(metadata);
    }

    if scope.is_admin() {
      if let Some(metadata) = patch.system_metadata {
        self.system_metadata.merge(metadata);
      }

      if let Some(limits) = patch.limits {
        if let Some(storage) = limits.storage {
          self.limits.storage.total = storage;
        }

        if let Some(transfer) = limits.transfer {
          self.limits.transfer.total = transfer;
        }

        if let Some(listeners) = limits.listeners {
          self.limits.listeners.total = listeners;
        }
      }
    }

    self.updated_at = DateTime::now();

    Ok(())
  }

  pub fn apply_admin_patch(&mut self, patch: AccountPatch) -> Result<(), ApplyPatchError> {
    if patch.name.is_none() && patch.user_metadata.is_none() && patch.system_metadata.is_none() {
      return Err(ApplyPatchError::PatchEmpty);
    }

    if let Some(ref name) = patch.name {
      let name = name.trim();
      if name.is_empty() {
        return Err(ApplyPatchError::invalid("name cannot be empty"));
      }

      self.name = name.into()
    }

    if let Some(metadata) = patch.user_metadata {
      self.user_metadata.merge(metadata);
    }

    if let Some(metadata) = patch.system_metadata {
      self.system_metadata.merge(metadata);
    }

    self.updated_at = DateTime::now();

    Ok(())
  }

  pub async fn increment_used_transfer(
    id: &str,
    size: usize,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    const KEY: &str = crate::key!(Account::KEY_LIMITS, Limits::KEY_TRANSFER, Limit::KEY_USED);

    Self::cl()
      .update_one(
        doc! { Account::KEY_ID: id },
        doc! { "$inc": { KEY: size as f64 } },
        None,
      )
      .await
  }

  pub async fn increment_used_listeners(
    id: &str,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    const KEY: &str = crate::key!(Account::KEY_LIMITS, Limits::KEY_LISTENERS, Limit::KEY_USED);

    Self::cl()
      .update_one(
        doc! { Account::KEY_ID: id },
        doc! { "$inc": { KEY: 1 } },
        None,
      )
      .await
  }

  pub async fn decrement_used_listeners(
    id: &str,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    const KEY: &str = crate::key!(Account::KEY_LIMITS, Limits::KEY_LISTENERS, Limit::KEY_USED);

    Self::cl()
      .update_one(
        doc! { Account::KEY_ID: id },
        doc! { "$inc": { KEY: -1 } },
        None,
      )
      .await
  }
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
      source_password: account.source_password,
    }
  }
}

impl From<Account> for AdminPublicAccount {
  fn from(account: Account) -> Self {
    Self(account)
  }
}

impl Account {
  pub const SOURCE_PASSWORD_LEN: usize = 32;

  pub fn into_public(self, scope: PublicScope) -> PublicAccount {
    match scope {
      PublicScope::Admin => PublicAccount::Admin(self.into()),
      PublicScope::User => PublicAccount::User(self.into()),
    }
  }

  pub fn random_source_password() -> String {
    uid::uid(Self::SOURCE_PASSWORD_LEN)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/", rename = "AccountLimits")]
#[serde(rename_all = "camelCase")]
#[macros::keys]
pub struct Limits {
  pub listeners: Limit,
  pub transfer: Limit,
  pub storage: Limit,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../defs/", rename = "AccountLimit")]
#[serde(rename_all = "camelCase")]
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

impl Model for Account {
  const UID_LEN: usize = 8;
  const CL_NAME: &'static str = "accounts";

  fn indexes() -> Vec<IndexModel> {
    let owner_id = IndexModel::builder()
      .keys(doc! { Account::KEY_OWNER_ID: 1 })
      .build();
    vec![owner_id]
  }
}

#[macro_export]
macro_rules! storage_quota {
  ($account_id:expr) => {
    match $crate::account::Account::get_by_id($account_id).await? {
      None => None,
      Some(account) => Some(account.limits.storage.avail()),
    }
  };
}

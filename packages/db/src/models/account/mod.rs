use crate::error::ApplyPatchError;
use crate::Model;
use crate::{metadata::Metadata, PublicScope};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Account {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/")]
#[serde(rename_all = "snake_case")]
pub struct UserPublicAccount {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub user_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/")]
pub struct AdminPublicAccount(pub Account);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/")]
#[serde(untagged)]
pub enum PublicAccount {
  Admin(AdminPublicAccount),
  User(UserPublicAccount),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/ops/")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct AccountPatch {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_metadata: Option<Metadata>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_metadata: Option<Metadata>,
}

impl Account {
  pub fn apply_patch(
    &mut self,
    patch: AccountPatch,
    scope: PublicScope,
  ) -> Result<(), ApplyPatchError> {
    match scope {
      PublicScope::User => {
        if patch.system_metadata.is_some() {
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

    if scope.is_admin() {
      if let Some(metadata) = patch.system_metadata {
        self.system_metadata.merge(metadata);
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
}

impl From<Account> for UserPublicAccount {
  fn from(account: Account) -> Self {
    Self {
      id: account.id,
      name: account.name,
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, Account::KEY_ID);
  }
}

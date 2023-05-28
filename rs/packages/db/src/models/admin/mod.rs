use crate::Model;
use crate::{error::ApplyPatchError, metadata::Metadata};
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

crate::register!(Admin);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Admin {
  #[serde(rename = "_id")]
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub password: String,
  pub system_metadata: Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "snake_case")]
pub struct PublicAdmin {
  #[serde(rename = "_id")]
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub system_metadata: Metadata,
}

impl Admin {
  pub fn into_public(self) -> PublicAdmin {
    PublicAdmin {
      id: self.id,
      first_name: self.first_name,
      last_name: self.last_name,
      email: self.email,
      created_at: self.created_at,
      updated_at: self.updated_at,
      system_metadata: self.system_metadata,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/ops/")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct AdminPatch {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub first_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_metadata: Option<Metadata>,
}

impl Admin {
  pub fn apply_patch(&mut self, patch: AdminPatch) -> Result<(), ApplyPatchError> {
    if patch.first_name.is_none() && patch.last_name.is_none() && patch.system_metadata.is_none() {
      return Err(ApplyPatchError::PatchEmpty);
    }

    if let Some(ref first_name) = patch.first_name {
      let first_name = first_name.trim();
      if first_name.is_empty() {
        return Err(ApplyPatchError::invalid("firstName cannot be empty"));
      }

      self.first_name = first_name.into();
    }

    if let Some(ref last_name) = patch.last_name {
      let last_name = last_name.trim();
      if last_name.is_empty() {
        return Err(ApplyPatchError::invalid("lastName cannot be empty"));
      }

      self.last_name = last_name.into();
    }

    if let Some(metadata) = patch.system_metadata {
      self.system_metadata.merge(metadata);
    }

    self.updated_at = DateTime::now();

    Ok(())
  }
}

impl Model for Admin {
  const UID_LEN: usize = 6;
  const CL_NAME: &'static str = "admins";

  fn indexes() -> Vec<IndexModel> {
    let email_opts = IndexOptions::builder().unique(true).build();
    let email = IndexModel::builder()
      .keys(doc! { Admin::KEY_EMAIL: 1 })
      .options(email_opts)
      .build();

    vec![email]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, Admin::KEY_ID);
  }
}

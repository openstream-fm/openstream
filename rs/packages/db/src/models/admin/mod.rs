use crate::Model;
use crate::{error::ApplyPatchError, metadata::Metadata};
use constants::validate::*;
use modify::Modify;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;
use validator::Validate;

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
  pub language: Option<String>,
  pub system_metadata: Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "snake_case")]
pub struct PublicAdmin {
  #[serde(rename = "_id")]
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub language: Option<String>,
  pub system_metadata: Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

impl Admin {
  pub fn into_public(self) -> PublicAdmin {
    PublicAdmin {
      id: self.id,
      first_name: self.first_name,
      last_name: self.last_name,
      email: self.email,
      language: self.language,
      system_metadata: self.system_metadata,
      created_at: self.created_at,
      updated_at: self.updated_at,
      deleted_at: self.deleted_at,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
#[ts(export, export_to = "../../../defs/ops/")]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct AdminPatch {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[modify(trim)]
  #[validate(
    length(
      min = 1,
      max = "VALIDATE_ADMIN_FIRST_NAME_MAX_LEN",
      message = "First name is either too short or too long",
    ),
    non_control_character(message = "Fist name cannot contain control characters")
  )]
  pub first_name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[modify(trim)]
  #[validate(
    length(
      min = 1,
      max = "VALIDATE_ADMIN_LAST_NAME_MAX_LEN",
      message = "Last name is either too short or too long",
    ),
    non_control_character(message = "Last name cannot contain control characters")
  )]
  pub last_name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  #[serde(deserialize_with = "serde_util::map_some")]
  #[modify(trim)]
  #[validate(
    length(
      min = 1,
      max = 60,
      message = "Language is either too short or too long",
    ),
    non_control_character(message = "Language cannot contain control characters")
  )]
  pub language: Option<Option<String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_metadata: Option<Metadata>,
}

impl Admin {
  pub fn apply_patch(&mut self, patch: AdminPatch) -> Result<(), ApplyPatchError> {
    if patch.first_name.is_none()
      && patch.last_name.is_none()
      && patch.language.is_none()
      && patch.system_metadata.is_none()
    {
      return Err(ApplyPatchError::PatchEmpty);
    }

    macro_rules! apply {
      ($key:ident) => {{
        if let Some($key) = patch.$key {
          self.$key = $key;
        }
      }};
    }

    apply!(first_name);
    apply!(last_name);
    apply!(language);

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

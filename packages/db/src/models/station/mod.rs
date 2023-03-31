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
pub struct Station {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub limits: Limits,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub playlist_is_randomly_shuffled: bool,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
  pub source_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/")]
#[serde(rename_all = "snake_case")]
pub struct UserPublicStation {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub limits: Limits,
  pub playlist_is_randomly_shuffled: bool,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub user_metadata: Metadata,
  pub source_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/")]
pub struct AdminPublicStation(pub Station);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/")]
#[serde(untagged)]
pub enum PublicStation {
  Admin(AdminPublicStation),
  User(UserPublicStation),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/ops/")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct StationPatch {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limits: Option<StationPatchLimits>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_metadata: Option<Metadata>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/ops/")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct StationPatchLimits {
  #[serde(skip_serializing_if = "Option::is_none")]
  storage: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  transfer: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  listeners: Option<u64>,
}

impl Station {
  pub fn apply_patch(
    &mut self,
    patch: StationPatch,
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

  pub fn apply_admin_patch(&mut self, patch: StationPatch) -> Result<(), ApplyPatchError> {
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
    const KEY: &str = crate::key!(Station::KEY_LIMITS, Limits::KEY_TRANSFER, Limit::KEY_USED);

    Self::cl()
      .update_one(
        doc! { Station::KEY_ID: id },
        doc! { "$inc": { KEY: size as f64 } },
        None,
      )
      .await
  }

  pub async fn increment_used_listeners(
    id: &str,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    const KEY: &str = crate::key!(Station::KEY_LIMITS, Limits::KEY_LISTENERS, Limit::KEY_USED);

    Self::cl()
      .update_one(
        doc! { Station::KEY_ID: id },
        doc! { "$inc": { KEY: 1 } },
        None,
      )
      .await
  }

  pub async fn decrement_used_listeners(
    id: &str,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    const KEY: &str = crate::key!(Station::KEY_LIMITS, Limits::KEY_LISTENERS, Limit::KEY_USED);

    Self::cl()
      .update_one(
        doc! { Station::KEY_ID: id },
        doc! { "$inc": { KEY: -1 } },
        None,
      )
      .await
  }
}

impl From<Station> for UserPublicStation {
  fn from(station: Station) -> Self {
    Self {
      id: station.id,
      name: station.name,
      limits: station.limits,
      playlist_is_randomly_shuffled: station.playlist_is_randomly_shuffled,
      created_at: station.created_at,
      updated_at: station.updated_at,
      user_metadata: station.user_metadata,
      source_password: station.source_password,
    }
  }
}

impl From<Station> for AdminPublicStation {
  fn from(station: Station) -> Self {
    Self(station)
  }
}

impl Station {
  pub const SOURCE_PASSWORD_LEN: usize = 32;

  pub fn into_public(self, scope: PublicScope) -> PublicStation {
    match scope {
      PublicScope::Admin => PublicStation::Admin(self.into()),
      PublicScope::User => PublicStation::User(self.into()),
    }
  }

  pub fn random_source_password() -> String {
    uid::uid(Self::SOURCE_PASSWORD_LEN)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/", rename = "StationLimits")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Limits {
  pub listeners: Limit,
  pub transfer: Limit,
  pub storage: Limit,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../defs/", rename = "StationLimit")]
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

impl Model for Station {
  const UID_LEN: usize = 8;
  const CL_NAME: &'static str = "stations";
}

#[macro_export]
macro_rules! storage_quota {
  ($station_id:expr) => {
    match $crate::station::Station::get_by_id($station_id).await? {
      None => None,
      Some(station) => Some(station.limits.storage.avail()),
    }
  };
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, Station::KEY_ID);
  }
}

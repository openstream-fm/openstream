use self::validation::*;
use crate::error::ApplyPatchError;
use crate::Model;
use crate::{metadata::Metadata, PublicScope};
use mongodb::bson::doc;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::map_some;
use serde_util::DateTime;
use ts_rs::TS;
use validate::url::patterns::*;
use validify::validify;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[validify]
#[macros::keys]
pub struct Station {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,

  pub picture_id: String,

  // profile data
  #[modify(trim)]
  #[validate(length(min = "NAME_MIN", max = "NAME_MAX"), non_control_character)]
  pub name: String,

  pub slug: String,

  #[modify(trim)]
  #[validate(length(min = "SLOGAN_MIN", max = "SLOGAN_MAX"), non_control_character)]
  pub slogan: Option<String>,

  #[modify(trim)]
  #[validate(length(min = "DESC_MIN", max = "DESC_MAX"))]
  pub description: Option<String>,

  // location and language
  // pub language_id: Option<String>,
  // pub region_id: Option<String>,
  #[validate]
  pub frequencies: Vec<StationFrequency>,

  // pìcs
  // pub picture_id: String,
  // pub hero_picture_id: Option<String>,

  // contact
  #[modify(trim, lowercase)]
  #[validate(email, length(max = "EMAIL_MAX"), non_control_character)]
  pub email: Option<String>,

  #[modify(trim)]
  #[validate(phone, length(max = "PHONE_MAX"), non_control_character)]
  pub phone: Option<String>,

  #[modify(trim)]
  #[validate(phone, length(max = "PHONE_MAX"), non_control_character)]
  pub whatsapp: Option<String>,

  // links
  #[modify(trim)]
  #[validate(
    url,
    regex = "WEBSITE",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub website_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url,
    regex = "TWITTER",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub twitter_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url,
    regex = "FACEBOOK",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub facebook_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url,
    regex = "INSTAGRAM",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub instagram_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url,
    regex = "YOUTUBE",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub youtube_url: Option<String>,

  #[modify(trim)]
  #[validate(url, regex = "TWITCH", length(max = "URLS_MAX"), non_control_character)]
  pub twitch_url: Option<String>,

  // app links
  #[modify(trim)]
  #[validate(
    url,
    regex = "GOOGLE_PLAY",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub google_play_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url,
    regex = "APP_STORE",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub app_store_url: Option<String>,

  // metadata
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,

  // misc
  pub limits: Limits,
  pub playlist_is_randomly_shuffled: bool,

  // auth
  pub source_password: String,

  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

pub mod validation {
  pub const NAME_MIN: u64 = 3;
  pub const NAME_MAX: u64 = 40;

  pub const SLOGAN_MIN: u64 = 3;
  pub const SLOGAN_MAX: u64 = 50;

  pub const DESC_MIN: u64 = 1;
  pub const DESC_MAX: u64 = 2000;

  pub const EMAIL_MAX: u64 = 70;
  pub const PHONE_MAX: u64 = 30;

  pub const URLS_MAX: u64 = 150;

  pub const FREQUENCY_MAX: f64 = 100_000.0;
  pub const FREQUENCY_MIN: f64 = 0.0;
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "snake_case")]
pub struct UserPublicStation {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,
  pub picture_id: String,

  // profile data
  pub name: String,
  pub slug: String,
  pub slogan: Option<String>,
  pub description: Option<String>,

  // location and language
  // pub language_id: Option<String>,
  // pub region_id: Option<String>,
  pub frequencies: Vec<StationFrequency>,

  // pìcs
  // pub picture_id: String,
  // pub hero_picture_id: Option<String>,

  // contact
  pub email: Option<String>,
  pub phone: Option<String>,
  pub whatsapp: Option<String>,

  // links
  pub website_url: Option<String>,
  pub twitter_url: Option<String>,
  pub instagram_url: Option<String>,
  pub twitch_url: Option<String>,
  pub facebook_url: Option<String>,
  pub youtube_url: Option<String>,

  // app links
  pub app_store_url: Option<String>,
  pub google_play_url: Option<String>,

  // metadata
  pub user_metadata: Metadata,

  // misc
  pub limits: Limits,
  pub playlist_is_randomly_shuffled: bool,

  // auth
  pub source_password: String,

  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "snake_case")]
#[validify]
#[macros::keys]
pub struct StationFrequency {
  kind: StationFrequencyKind,
  #[validate(range(min = "FREQUENCY_MIN", max = "FREQUENCY_MAX"))]
  freq: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "kebab-case")]
pub enum StationFrequencyKind {
  Am,
  Fm,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
pub struct AdminPublicStation(pub Station);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
#[serde(untagged)]
pub enum PublicStation {
  Admin(AdminPublicStation),
  User(UserPublicStation),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/ops/")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[validify]
pub struct StationPatch {
  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[modify(trim)]
  #[validate(length(min = "NAME_MIN", max = "NAME_MAX"), non_control_character)]
  pub name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub picture_id: Option<String>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    //skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(length(min = "SLOGAN_MIN", max = "SLOGAN_MAX"), non_control_character)]
  pub slogan: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(length(min = "DESC_MIN", max = "DESC_MAX"))]
  pub description: Option<Option<String>>,

  // location and language
  // pub language_id: Option<String>,
  // pub region_id: Option<String>,
  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  pub frequencies: Option<Vec<StationFrequency>>,

  // pìcs
  // pub picture_id: String,
  // pub hero_picture_id: Option<String>,

  // contact
  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim, lowercase)]
  #[validate(email, length(max = "EMAIL_MAX"), non_control_character)]
  pub email: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(phone, length(max = "PHONE_MAX"), non_control_character)]
  pub phone: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(phone, length(max = "PHONE_MAX"), non_control_character)]
  pub whatsapp: Option<Option<String>>,

  // links
  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url,
    regex = "WEBSITE",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub website_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url,
    regex = "TWITTER",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub twitter_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url,
    regex = "FACEBOOK",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub facebook_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url,
    regex = "INSTAGRAM",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub instagram_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url,
    regex = "YOUTUBE",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub youtube_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(url, regex = "TWITCH", length(max = "URLS_MAX"), non_control_character)]
  pub twitch_url: Option<Option<String>>,

  // app links
  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url,
    regex = "GOOGLE_PLAY",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub google_play_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url,
    regex = "APP_STORE",
    length(max = "URLS_MAX"),
    non_control_character
  )]
  pub app_store_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limits: Option<StationPatchLimits>,

  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_metadata: Option<Metadata>,

  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/ops/")]
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
    mut patch: StationPatch,
    scope: PublicScope,
  ) -> Result<(), ApplyPatchError> {
    match scope {
      PublicScope::User => {
        if patch.system_metadata.is_some() || patch.limits.is_some() {
          return Err(ApplyPatchError::out_of_scope(
            "Some of the specified fields are out of scope",
          ));
        }
      }

      PublicScope::Admin => {}
    }

    macro_rules! apply {
      ($name:ident) => {
        if let Some($name) = patch.$name.take() {
          self.$name = $name;
        }
      };
    }

    apply!(picture_id);

    apply!(name);
    apply!(slogan);
    apply!(description);

    apply!(email);
    apply!(whatsapp);

    apply!(website_url);
    apply!(twitter_url);
    apply!(facebook_url);
    apply!(instagram_url);
    apply!(youtube_url);
    apply!(twitch_url);

    apply!(google_play_url);
    apply!(app_store_url);

    apply!(frequencies);

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
    }

    self.updated_at = DateTime::now();

    Ok(())
  }

  // pub fn apply_admin_patch(&mut self, patch: StationPatch) -> Result<(), ApplyPatchError> {
  //   if patch.name.is_none() && patch.user_metadata.is_none() && patch.system_metadata.is_none() {
  //     return Err(ApplyPatchError::PatchEmpty);
  //   }

  //   if let Some(ref name) = patch.name {
  //     let name = name.trim();
  //     if name.is_empty() {
  //       return Err(ApplyPatchError::invalid("name cannot be empty"));
  //     }

  //     self.name = name.into()
  //   }

  //   if let Some(metadata) = patch.user_metadata {
  //     self.user_metadata.merge(metadata);
  //   }

  //   if let Some(metadata) = patch.system_metadata {
  //     self.system_metadata.merge(metadata);
  //   }

  //   self.updated_at = DateTime::now();

  //   Ok(())
  // }

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
      account_id: station.account_id,
      picture_id: station.picture_id,

      //language_id: station.language_id,
      //region_id: station.region_id,
      frequencies: station.frequencies,

      //picture_id: station.picture_id,
      //hero_picture_id: station.hero_picture_id,
      name: station.name,
      slug: station.slug,
      slogan: station.slogan,
      description: station.description,

      email: station.email,
      phone: station.phone,
      whatsapp: station.whatsapp,

      website_url: station.website_url,
      twitter_url: station.twitter_url,
      instagram_url: station.instagram_url,
      twitch_url: station.twitch_url,
      facebook_url: station.facebook_url,
      youtube_url: station.youtube_url,

      app_store_url: station.app_store_url,
      google_play_url: station.google_play_url,

      limits: station.limits,
      playlist_is_randomly_shuffled: station.playlist_is_randomly_shuffled,
      source_password: station.source_password,

      user_metadata: station.user_metadata,

      created_at: station.created_at,
      updated_at: station.updated_at,
      deleted_at: station.deleted_at,
    }
  }
}

impl From<Station> for AdminPublicStation {
  fn from(station: Station) -> Self {
    Self(station)
  }
}

impl Station {
  pub const SOURCE_PASSWORD_LEN: usize = 16;

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
#[ts(export, export_to = "../../../defs/", rename = "StationLimits")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Limits {
  pub listeners: Limit,
  pub transfer: Limit,
  pub storage: Limit,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../defs/", rename = "StationLimit")]
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

  fn indexes() -> Vec<IndexModel> {
    let account_id = IndexModel::builder()
      .keys(doc! { Self::KEY_ACCOUNT_ID: 1 })
      .build();

    let created_at = IndexModel::builder()
      .keys(doc! { Self::KEY_CREATED_AT: 1 })
      .build();

    let updated_at = IndexModel::builder()
      .keys(doc! { Self::KEY_UPDATED_AT: 1 })
      .build();

    let deleted_at = IndexModel::builder()
      .keys(doc! { Self::KEY_DELETED_AT: 1 })
      .build();

    vec![account_id, created_at, updated_at, deleted_at]
  }
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

use crate::audio_file::AudioFile;
use crate::error::ApplyPatchError;
use crate::{current_filter_doc, Model};
use crate::{metadata::Metadata, PublicScope};
use constants::validate::*;
use drop_tracer::Token;
use geoip::CountryCode;
use lang::LangCode;
use modify::Modify;
use mongodb::bson::{doc, Bson, SerializerOptions};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{ClientSession, IndexModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_util::map_some;
use serde_util::DateTime;
use ts_rs::TS;
use validate::url::patterns::*;
use validator::Validate;

crate::register!(Station);

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Station {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,

  pub picture_id: String,

  // profile data
  #[modify(trim)]
  #[validate(
    length(
      min = "VALIDATE_STATION_NAME_MIN_LEN",
      max = "VALIDATE_STATION_NAME_MAX_LEN",
      message = "Station name is empty or too long"
    ),
    non_control_character(message = "Station name cannot have control characters")
  )]
  pub name: String,

  #[modify(trim)]
  #[validate(
    regex(
      path = "VALIDATE_STATION_SLUG_PATTERN",
      message = "Station slug can only contains letters, numbers, dashes, underscores and dots"
    ),
    length(
      min = "VALIDATE_STATION_SLUG_MIN_LEN",
      max = "VALIDATE_STATION_SLUG_MAX_LEN",
      message = "Station slug is empty or too long"
    ),
    non_control_character(message = "Station slug cannot have control characters")
  )]
  pub slug: Option<String>,

  #[modify(trim)]
  #[validate(
    length(
      min = "VALIDATE_STATION_SLOGAN_MIN_LEN",
      max = "VALIDATE_STATION_SLOGAN_MAX_LEN",
      message = "Slogan is empty or too long"
    ),
    non_control_character(message = "Slogan cannot have control characters")
  )]
  pub slogan: Option<String>,

  pub type_of_content: StationTypeOfContent,
  pub country_code: CountryCode,
  pub lang_code: LangCode,

  #[modify(trim)]
  #[validate(length(
    min = "VALIDATE_STATION_DESC_MIN_LEN",
    max = "VALIDATE_STATION_DESC_MAX_LEN",
    message = "Description is either too short or too long"
  ))]
  pub description: Option<String>,

  // location and language
  // pub language_id: Option<String>,
  // pub region_id: Option<String>,
  // #[validate]
  // #[deprecated(since = "0.8.4", note = "use frequency instead")]
  // pub frequencies: Vec<StationFrequency>,
  #[validate]
  pub frequency: Option<StationFrequency>,

  // contact
  #[modify(trim, lowercase)]
  #[validate(
    email(message = "Email is invalid"),
    length(max = "VALIDATE_STATION_EMAIL_MAX_LEN", message = "Email is too long"),
    non_control_character(message = "Email cannot have control characters")
  )]
  pub email: Option<String>,

  #[modify(trim)]
  #[validate(
    phone(message = "Phone is invalid"),
    length(max = "VALIDATE_STATION_PHONE_MAX_LEN", message = "Phone is too long"),
    non_control_character(message = "Phone is invalid")
  )]
  pub phone: Option<String>,

  #[modify(trim)]
  #[validate(
    phone(message = "WhatsApp number is invalid"),
    length(
      max = "VALIDATE_STATION_WHATSAPP_MAX_LEN",
      message = "WhatsApp number is too long"
    ),
    non_control_character(message = "WhatsApp number cannot have control characters")
  )]
  pub whatsapp: Option<String>,

  // links
  #[modify(trim)]
  #[validate(
    url(message = "Website URL is invalid"),
    regex(path = "WEBSITE", message = "Website URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Website URL is too long"
    ),
    non_control_character(message = "Website URL cannot have control characters")
  )]
  pub website_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "Twitter URL is invalid"),
    regex(path = "TWITTER", message = "Twitter URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Twitter URL is too long"
    ),
    non_control_character(message = "Twitter URL cannot have control characters")
  )]
  pub twitter_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "Facebook URL is invalid"),
    regex(path = "FACEBOOK", message = "Facebook URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Facebook URL is too long"
    ),
    non_control_character(message = "Facebook URL cannot have control characters")
  )]
  pub facebook_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "Instagram URL is invalid"),
    regex(path = "INSTAGRAM", message = "Instagram URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Instagram URL is too long"
    ),
    non_control_character(message = "Instagram URL cannot have control characters")
  )]
  pub instagram_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "Threads URL is invalid"),
    regex(path = "THREADS", message = "Threads URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Threads URL is too long"
    ),
    non_control_character(message = "Threads URL cannot have control characters")
  )]
  pub threads_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "Youtube URL is invalid"),
    regex(path = "YOUTUBE", message = "Youtube URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Youtube URL is too long"
    ),
    non_control_character(message = "Youtube URL cannot have control characters")
  )]
  pub youtube_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "Twitch URL is invalid"),
    regex(path = "TWITCH", message = "Twitch URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Twitch URL is too long"
    ),
    non_control_character(message = "Twitch URL cannot have control characters")
  )]
  pub twitch_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "Twtich URL is invalid"),
    regex(path = "TIKTOK", message = "TikTok URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "TikTok URL is invalid"
    ),
    non_control_character(message = "TikTok URL cannot have control characters")
  )]
  pub tiktok_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "Spotify URL is invalid"),
    regex(path = "SPOTIFY", message = "Spotify URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Spotify URL is invalid"
    ),
    non_control_character(message = "Spotify URL cannot have control characters")
  )]
  pub spotify_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "RadioCut URL is invalid"),
    regex(path = "RADIOCUT", message = "RadioCut URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "RadioCut URL is invalid"
    ),
    non_control_character(message = "RadioCut URL cannot have control characters")
  )]
  pub radiocut_url: Option<String>,

  // app links
  #[modify(trim)]
  #[validate(
    url(message = "Google Play URL is invalid"),
    regex(path = "GOOGLE_PLAY", message = "Google Play URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Google Play URL is too long"
    ),
    non_control_character(message = "Google Play URL cannot have control characters")
  )]
  pub google_play_url: Option<String>,

  #[modify(trim)]
  #[validate(
    url(message = "App Store URL is invalid"),
    regex(path = "APP_STORE", message = "App Store URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "App Store URL is too long"
    ),
    non_control_character(message = "App Store URL cannot have control characters")
  )]
  pub app_store_url: Option<String>,

  // metadata
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,

  // external-relay
  pub external_relay_url: Option<String>,
  pub external_relay_redirect: bool,

  // auth
  pub source_password: String,

  // runtime
  pub owner_deployment_info: Option<OwnerDeploymentInfo>,
  pub last_external_relay_probe_started_at: Option<DateTime>,
  pub playlist_is_randomly_shuffled: bool,

  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(
  Debug,
  Clone,
  Copy,
  Serialize,
  Deserialize,
  strum::AsRefStr,
  strum::Display,
  strum::EnumCount,
  strum::EnumIter,
  strum::EnumVariantNames,
  TS,
  JsonSchema,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[ts(export, export_to = "../../../defs/db/")]
#[macros::keys]
pub enum StationTypeOfContent {
  Comedy,
  Educational,
  General,
  Music,
  News,
  Religious,
  Sports,
  Talk,
}

impl StationTypeOfContent {
  pub fn display_name(&self) -> &'static str {
    use StationTypeOfContent::*;
    match self {
      General => "General",
      News => "News",
      Talk => "Talk",
      Music => "Music",
      Educational => "Educational",
      Sports => "Sports",
      Religious => "Religious",
      Comedy => "Comedy",
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct OwnerDeploymentInfo {
  pub deployment_id: String,
  pub task_id: String,
  pub content_type: String,

  // this Option<> is for backwards compatibility
  // it should be removed in the future
  pub health_checked_at: Option<DateTime>,
}

impl From<OwnerDeploymentInfo> for Bson {
  fn from(info: OwnerDeploymentInfo) -> Bson {
    mongodb::bson::to_bson_with_options(
      &info,
      SerializerOptions::builder().human_readable(false).build(),
    )
    .unwrap()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "snake_case")]
pub struct UserPublicStation {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,
  pub picture_id: String,

  // profile data
  pub name: String,
  pub slug: Option<String>,
  pub slogan: Option<String>,
  pub description: Option<String>,

  pub type_of_content: StationTypeOfContent,
  pub country_code: CountryCode,
  pub lang_code: LangCode,
  // location and language
  // pub language_id: Option<String>,
  // pub region_id: Option<String>,
  pub frequency: Option<StationFrequency>,

  // contact
  pub email: Option<String>,
  pub phone: Option<String>,
  pub whatsapp: Option<String>,

  // links
  pub website_url: Option<String>,
  pub twitter_url: Option<String>,
  pub facebook_url: Option<String>,
  pub instagram_url: Option<String>,
  pub threads_url: Option<String>,
  pub twitch_url: Option<String>,
  pub tiktok_url: Option<String>,
  pub youtube_url: Option<String>,
  pub spotify_url: Option<String>,
  pub radiocut_url: Option<String>,

  // app links
  pub app_store_url: Option<String>,
  pub google_play_url: Option<String>,

  // metadata
  pub user_metadata: Metadata,

  // external-relay
  pub external_relay_url: Option<String>,
  pub external_relay_redirect: bool,

  // misc
  pub playlist_is_randomly_shuffled: bool,

  // auth
  pub source_password: String,

  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StationFrequency {
  kind: StationFrequencyKind,
  #[validate(range(
    min = "VALIDATE_STATION_FREQUENCY_MIN",
    max = "VALIDATE_STATION_FREQUENCY_MAX",
    message = "Station frequency is either too low or too high"
  ))]
  freq: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/")]
#[serde(rename_all = "kebab-case")]
pub enum StationFrequencyKind {
  Am,
  Fm,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/")]
pub struct AdminPublicStation(pub Station);

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/")]
#[serde(untagged)]
pub enum PublicStation {
  Admin(AdminPublicStation),
  User(UserPublicStation),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
#[ts(export, export_to = "../../../defs/ops/")]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct StationPatch {
  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[modify(trim)]
  #[validate(
    length(
      min = "VALIDATE_STATION_NAME_MIN_LEN",
      max = "VALIDATE_STATION_NAME_MAX_LEN",
      message = "Station name is empty or too long"
    ),
    non_control_character(message = "Station name cannot have control characters")
  )]
  pub name: Option<String>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    regex(
      path = "VALIDATE_STATION_SLUG_PATTERN",
      message = "Station slug can only contain numbers, letters, dashes, underscores and dots",
    ),
    length(
      min = "VALIDATE_STATION_SLUG_MIN_LEN",
      max = "VALIDATE_STATION_SLUG_MAX_LEN",
      message = "Station slug is empty or too long"
    ),
    non_control_character(message = "Station slug cannot have control characters")
  )]
  pub slug: Option<Option<String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub picture_id: Option<String>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    length(
      min = "VALIDATE_STATION_SLOGAN_MIN_LEN",
      max = "VALIDATE_STATION_SLOGAN_MAX_LEN",
      message = "Slogan is empty or too long"
    ),
    non_control_character(message = "Slogan cannot have control characters")
  )]
  pub slogan: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(length(
    min = "VALIDATE_STATION_DESC_MIN_LEN",
    max = "VALIDATE_STATION_DESC_MAX_LEN",
    message = "Description is either too short or too long"
  ))]
  pub description: Option<Option<String>>,

  #[ts(optional)]
  pub type_of_content: Option<StationTypeOfContent>,

  #[ts(optional)]
  pub country_code: Option<CountryCode>,

  #[ts(optional)]
  pub lang_code: Option<LangCode>,

  // location and language
  // pub language_id: Option<String>,
  // pub region_id: Option<String>,

  // #[ts(optional)]
  //#[serde(skip_serializing_if = "Option::is_none")]
  // #[validate]
  // pub frequencies: Option<Vec<StationFrequency>>,
  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[validate]
  pub frequency: Option<Option<StationFrequency>>,

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
  #[validate(
    email(message = "Email is invalid"),
    length(max = "VALIDATE_STATION_EMAIL_MAX_LEN", message = "Email is too long"),
    non_control_character(message = "Email cannot have control characters")
  )]
  pub email: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    phone(message = "Phone is invalid"),
    length(max = "VALIDATE_STATION_PHONE_MAX_LEN", message = "Phone is too long"),
    non_control_character(message = "Phone cannot have control characters")
  )]
  pub phone: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    phone(message = "WhatsApp number is invalid"),
    length(
      max = "VALIDATE_STATION_WHATSAPP_MAX_LEN",
      message = "WhatsApp number is too long"
    ),
    non_control_character(message = "WhatsApp number cannot have control characters")
  )]
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
    url(message = "Website URL is invalid"),
    regex(path = "WEBSITE", message = "Website URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Website URL is too long"
    ),
    non_control_character(message = "Website URL cannot have control characters")
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
    url(message = "Twitter URL is invalid"),
    regex(path = "TWITTER", message = "Twitter URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Twitter URL is too long"
    ),
    non_control_character(message = "Twitter URL cannot have control characters")
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
    url(message = "Facebook URL is invalid"),
    regex(path = "FACEBOOK", message = "Facebook URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Facebook URL is too long"
    ),
    non_control_character(message = "Facebook URL cannot have control characters")
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
    url(message = "Instagram URL is invalid"),
    regex(path = "INSTAGRAM", message = "Instagram URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Instagram URL is too long"
    ),
    non_control_character(message = "Instagram URL cannot have control characters")
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
    url(message = "Threads URL is invalid"),
    regex(path = "THREADS", message = "Threads URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Threads URL is too long"
    ),
    non_control_character(message = "Threads URL cannot have control characters")
  )]
  pub threads_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url(message = "Youtube URL is invalid"),
    regex(path = "YOUTUBE", message = "Youtube URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Youtube URL is too long"
    ),
    non_control_character(message = "Youtube URL cannot have control characters")
  )]
  pub youtube_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url(message = "Twitch URL is invalid"),
    regex(path = "TWITCH", message = "Twitch URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Twitch URL is too long"
    ),
    non_control_character(message = "Twitch URL cannot have control characters")
  )]
  pub twitch_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url(message = "Twtich URL is invalid"),
    regex(path = "TIKTOK", message = "TikTok URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "TikTok URL is invalid"
    ),
    non_control_character(message = "TikTok URL cannot have control characters")
  )]
  pub tiktok_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url(message = "Spotify URL is invalid"),
    regex(path = "SPOTIFY", message = "Spotify URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Spotify URL is invalid"
    ),
    non_control_character(message = "Spotify URL cannot have control characters")
  )]
  pub spotify_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url(message = "RadioCut URL is invalid"),
    regex(path = "RADIOCUT", message = "RadioCut URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "RadioCut URL is invalid"
    ),
    non_control_character(message = "RadioCut URL cannot have control characters")
  )]
  pub radiocut_url: Option<Option<String>>,

  // app links
  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url(message = "Google Play URL is invalid"),
    regex(path = "GOOGLE_PLAY", message = "Google Play URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "Google Play URL is too long"
    ),
    non_control_character(message = "Google Play URL cannot have control characters")
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
    url(message = "App Store URL is invalid"),
    regex(path = "APP_STORE", message = "App Store URL is invalid"),
    length(
      max = "VALIDATE_STATION_URLS_MAX_LEN",
      message = "App Store URL is too long"
    ),
    non_control_character(message = "App Store URL cannot have control characters")
  )]
  pub app_store_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(
    default,
    deserialize_with = "map_some",
    skip_serializing_if = "Option::is_none"
  )]
  #[modify(trim)]
  #[validate(
    url(message = "External Relay URL is invalid"),
    regex(path = "WEBSITE", message = "External Relay URL is invalid"),
    length(
      max = "VALIDATE_STATION_EXTERNAL_RELAY_URL_MAX_LEN",
      message = "External Relay URL is too long"
    ),
    non_control_character(message = "External Relay URL cannot have control characters")
  )]
  pub external_relay_url: Option<Option<String>>,

  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub external_relay_redirect: Option<bool>,

  //#[ts(optional)]
  //#[serde(skip_serializing_if = "Option::is_none")]
  //pub limits: Option<StationPatchLimits>,
  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_metadata: Option<Metadata>,

  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_metadata: Option<Metadata>,
}

impl Station {
  pub fn random_owner_task_id() -> String {
    uid::uid(6)
  }

  pub async fn try_set_owner_deployment_info(
    station_id: &str,
    info: OwnerDeploymentInfo,
    token: Token,
  ) -> Result<
    Result<(Station, DeploymentTakeDropper), Option<(Station, OwnerDeploymentInfo)>>,
    mongodb::error::Error,
  > {
    let filter = current_filter_doc! {
      Station::KEY_ID: station_id,
    };

    let update = vec![doc! {
      "$set": {
        Station::KEY_OWNER_DEPLOYMENT_INFO: {
          "$ifNull": [
            const_str::concat!("$", Station::KEY_OWNER_DEPLOYMENT_INFO),
            info.clone(),
          ]
        }
      }
    }];

    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::Before)
      .build();

    let r = Station::cl()
      .find_one_and_update(filter, update, options)
      .await?;

    let station = match r {
      None => return Ok(Err(None)),
      Some(doc) => doc,
    };

    match &station.owner_deployment_info {
      Some(owner_deploymeny_info) => {
        let info = owner_deploymeny_info.clone();
        Ok(Err(Some((station, info))))
      }

      None => {
        let inner = DeploymentTakeDropperInner {
          station_id: station_id.to_string(),
          task_id: info.task_id,
          token,
        };

        let dropper = DeploymentTakeDropper(Some(inner));

        Ok(Ok((station, dropper)))
      }
    }
  }

  pub fn apply_patch(
    &mut self,
    mut patch: StationPatch,
    scope: PublicScope,
  ) -> Result<(), ApplyPatchError> {
    match scope {
      PublicScope::User => {
        if patch.system_metadata.is_some() {
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
    apply!(type_of_content);
    apply!(country_code);
    apply!(lang_code);

    apply!(email);
    apply!(whatsapp);

    apply!(website_url);
    apply!(twitter_url);
    apply!(facebook_url);
    apply!(instagram_url);
    apply!(threads_url);
    apply!(youtube_url);
    apply!(twitch_url);
    apply!(tiktok_url);
    apply!(spotify_url);
    apply!(radiocut_url);

    apply!(google_play_url);
    apply!(app_store_url);

    apply!(frequency);

    // let prev_external_relay_url = self.external_relay_url.clone();
    match &patch.external_relay_url {
      None => {}
      Some(opt) => {
        if opt.is_some() && opt != &self.external_relay_url {
          self.last_external_relay_probe_started_at = None;
        }
      }
    }
    apply!(external_relay_url);
    apply!(external_relay_redirect);

    if let Some(metadata) = patch.user_metadata {
      self.user_metadata.merge(metadata);
    }

    self.updated_at = DateTime::now();

    Ok(())
  }

  pub async fn get_used_storage_with_session(
    station_id: &str,
    session: &mut ClientSession,
  ) -> Result<u64, mongodb::error::Error> {
    let filter = doc! { AudioFile::KEY_STATION_ID: station_id };
    let mut cursor = AudioFile::cl()
      .find_with_session(filter, None, session)
      .await?;
    let mut acc: u64 = 0;
    while let Some(file) = cursor.next(session).await.transpose()? {
      acc += file.len;
    }

    Ok(acc)
  }
}

impl From<Station> for UserPublicStation {
  fn from(station: Station) -> Self {
    Self {
      id: station.id,
      account_id: station.account_id,
      picture_id: station.picture_id,
      type_of_content: station.type_of_content,
      country_code: station.country_code,
      lang_code: station.lang_code,
      // language_id: station.language_id,
      // region_id: station.region_id,
      // frequencies: station.frequencies,
      frequency: station.frequency,

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
      facebook_url: station.facebook_url,
      instagram_url: station.instagram_url,
      threads_url: station.threads_url,
      twitch_url: station.twitch_url,
      tiktok_url: station.tiktok_url,
      youtube_url: station.youtube_url,
      spotify_url: station.spotify_url,
      radiocut_url: station.radiocut_url,

      app_store_url: station.app_store_url,
      google_play_url: station.google_play_url,

      playlist_is_randomly_shuffled: station.playlist_is_randomly_shuffled,

      external_relay_url: station.external_relay_url,
      external_relay_redirect: station.external_relay_redirect,

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
  ($account_id:expr) => {
    match $crate::account::Account::get_by_id($account_id).await? {
      None => None,
      Some(account) => Some(account.limits.storage.avail()),
    }
  };
}

#[derive(Debug)]
#[must_use]
pub struct DeploymentTakeDropper(Option<DeploymentTakeDropperInner>);

#[derive(Debug)]
pub struct DeploymentTakeDropperInner {
  station_id: String,
  task_id: String,
  token: Token,
}

impl Drop for DeploymentTakeDropper {
  fn drop(&mut self) {
    if let Some(inner) = self.0.take() {
      tokio::spawn(async move {
        let DeploymentTakeDropperInner {
          station_id,
          task_id,
          token,
        } = inner;

        const KEY_OWNER_TASK: &str = const_str::concat!(
          Station::KEY_OWNER_DEPLOYMENT_INFO,
          ".",
          OwnerDeploymentInfo::KEY_TASK_ID
        );

        let filter = doc! {
          Station::KEY_ID: &station_id,
          KEY_OWNER_TASK: &task_id,
        };

        let update = doc! {
          "$set": {
            Station::KEY_OWNER_DEPLOYMENT_INFO: null,
          }
        };

        if let Err(e) = Station::cl().update_one(filter, update, None).await {
          log::error!(
            "error setting owner_deployment_id back to null for station={} task={} => {} => {:?}",
            station_id,
            task_id,
            e,
            e
          );
        };

        drop(token);
      });
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, Station::KEY_ID);
  }
}

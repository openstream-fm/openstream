use crate::Model;
use mongodb::{
  bson::{doc, SerializerOptions},
  options::FindOneOptions,
  results::UpdateResult,
  IndexModel,
};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

crate::register!(MediaSession);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct MediaSession {
  #[serde(rename = "_id")]
  pub id: String,
  pub station_id: String,

  pub deployment_id: String,

  pub state: MediaSessionState,

  #[serde(flatten)]
  pub kind: MediaSessionKind,

  pub now_playing: Option<MediaSessionNowPlaying>,

  #[serde(with = "serde_util::as_f64")]
  pub transfer_bytes: u64,
  pub closed_at: Option<DateTime>,
  #[serde(with = "serde_util::as_f64::option")]
  pub duration_ms: Option<u64>,

  // TODO: this Option<> is for back compat only
  // create a migration and change this to DateTime
  pub health_checked_at: Option<DateTime>,

  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
#[macros::keys]
#[allow(clippy::large_enum_variant)]
pub enum MediaSessionKind {
  #[serde(rename = "playlist")]
  Playlist {
    resumed_from: Option<String>,
    last_audio_file_id: String,
    last_audio_file_order: f64,
    last_audio_chunk_i: f64,
    #[serde(with = "serde_util::as_f64")]
    last_audio_chunk_skip_parts: usize,
    last_audio_chunk_date: DateTime,
  },

  #[serde(rename = "live")]
  Live { request: crate::http::Request },

  #[serde(rename = "external-relay")]
  ExternalRelay { url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub enum MediaSessionState {
  Open,
  Closed,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct MediaSessionNowPlaying {
  pub title: String,
  pub artist: Option<String>,
}

impl From<MediaSessionNowPlaying> for mongodb::bson::Bson {
  fn from(value: MediaSessionNowPlaying) -> Self {
    mongodb::bson::to_bson_with_options(
      &value,
      SerializerOptions::builder().human_readable(false).build(),
    )
    .expect("error convering MediaSessionNowPlaying to Bson")
  }
}

impl From<MediaSessionNowPlaying> for mongodb::bson::Document {
  fn from(value: MediaSessionNowPlaying) -> Self {
    mongodb::bson::to_document(&value).expect("error convering MediaSessionNowPlaying to Document")
  }
}

impl MediaSession {
  pub fn resumed_from(&self) -> Option<&str> {
    match &self.kind {
      MediaSessionKind::Live { .. } => None,
      MediaSessionKind::ExternalRelay { .. } => None,
      MediaSessionKind::Playlist { resumed_from, .. } => resumed_from.as_ref().map(|s| s.as_str()),
    }
  }

  pub async fn get_current_for_station(
    station_id: &str,
  ) -> Result<Option<MediaSession>, mongodb::error::Error> {
    
    // TODO: improve this
    let open_deployment_ids = crate::deployment::Deployment::cl().distinct(
      crate::deployment::Deployment::KEY_ID, 
      doc! {
        crate::deployment::Deployment::KEY_STATE: crate::deployment::DeploymentState::KEY_ENUM_VARIANT_ACTIVE
      },
      None
    ).await?;

    let filter = doc! {
      MediaSession::KEY_DEPLOYMENT_ID: { "$in": open_deployment_ids },
      MediaSession::KEY_STATION_ID: station_id,
      MediaSession::KEY_STATE: MediaSessionState::KEY_ENUM_VARIANT_OPEN
    };

    let sort = doc! {
      MediaSession::KEY_CREATED_AT: -1
    };

    let options = FindOneOptions::builder().sort(sort).build();

    Self::cl().find_one(filter, options).await
  }

  pub async fn set_file_chunk_part(
    id: &str,
    file_id: &str,
    chunk: f64,
    part: f64,
  ) -> Result<UpdateResult, mongodb::error::Error> {
    let update = doc! {
      "$set": {
        MediaSessionKind::KEY_LAST_AUDIO_FILE_ID: file_id,
        MediaSessionKind::KEY_LAST_AUDIO_CHUNK_I: chunk,
        MediaSessionKind::KEY_LAST_AUDIO_CHUNK_SKIP_PARTS: part
      }
    };

    MediaSession::update_by_id(id, update).await
  }
}

impl Model for MediaSession {
  const CL_NAME: &'static str = "media_sessions";
  const UID_LEN: usize = 16;

  fn indexes() -> Vec<IndexModel> {
    let station_id = IndexModel::builder()
      .keys(doc! { MediaSession::KEY_STATION_ID: 1 })
      .build();
    let state = IndexModel::builder()
      .keys(doc! { MediaSession::KEY_STATE: 1 })
      .build();
    let kind = IndexModel::builder()
      .keys(doc! { MediaSessionKind::KEY_ENUM_TAG: 1 })
      .build();
    let created_at = IndexModel::builder()
      .keys(doc! { MediaSession::KEY_CREATED_AT: 1 })
      .build();
    let closed_at = IndexModel::builder()
      .keys(doc! { MediaSession::KEY_CLOSED_AT: 1 })
      .build();

    vec![station_id, state, kind, created_at, closed_at]
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{audio_file::AudioFile, station::Station};

  #[test]
  fn serde() {
    let doc = MediaSession {
      id: MediaSession::uid(),
      station_id: Station::uid(),
      deployment_id: Station::uid(),
      created_at: DateTime::now(),
      updated_at: DateTime::now(),
      transfer_bytes: 0,
      kind: MediaSessionKind::Playlist {
        resumed_from: None,
        last_audio_file_id: AudioFile::uid(),
        last_audio_file_order: 0.0,
        last_audio_chunk_i: 0.0,
        last_audio_chunk_skip_parts: 1,
        last_audio_chunk_date: DateTime::now(),
      },
      now_playing: None,
      state: MediaSessionState::Closed,
      health_checked_at: Some(DateTime::now()),
      closed_at: Some(DateTime::now()),
      duration_ms: Some(100),
    };

    let buf = mongodb::bson::to_vec(&doc).unwrap();

    let target: MediaSession = mongodb::bson::from_slice(&buf).unwrap();

    assert_eq!(doc, target);
  }

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, MediaSession::KEY_ID);
  }
}

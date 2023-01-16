use crate::Model;
use mongodb::{bson::doc, options::FindOneOptions, results::UpdateResult, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct MediaSession {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,

  #[ts(skip)]
  #[serde(flatten)]
  pub kind: MediaSessionKind,

  #[ts(skip)]
  #[serde(flatten)]
  pub state: MediaSessionState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
#[macros::keys]
#[allow(clippy::large_enum_variant)]
pub enum MediaSessionKind {
  #[serde(rename = "playlist")]
  Playlist {
    resumed_from: Option<String>,
    last_audio_file_id: Option<String>,
    last_audio_chunk_i: f64,
    #[serde(with = "serde_util::as_f64")]
    last_audio_chunk_skip_parts: usize,
    last_audio_chunk_date: DateTime,
  },

  #[serde(rename = "live")]
  Live { request: crate::http::Request },
}

impl MediaSessionKind {
  pub const TAG_PLAYLIST: &str = "playlist";
  pub const TAG_LIVE: &str = "live";
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "state")]
#[macros::keys]
pub enum MediaSessionState {
  Open,
  Closed {
    closed_at: DateTime,
    #[serde(with = "serde_util::as_f64")]
    duration_ms: u64,
    // error: Option<MediaSessionError>,
  },
}

impl MediaSessionState {
  pub const TAG_OPEN: &str = "open";
  pub const TAG_CLOSED: &str = "closed";
}

// #[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[ts(export, export_to = "../../defs/db/")]
// #[serde(rename_all = "snake_case")]
// #[macros::keys]
// pub struct MediaSessionError {
//   pub debug: String,
//   pub display: String,
// }

// impl<E: std::error::Error> From<E> for MediaSessionError {
//   fn from(e: E) -> Self {
//     Self {
//       debug: format!("{:?}", e),
//       display: format!("{}", e),
//     }
//   }
// }

impl MediaSession {
  pub fn resumed_from(&self) -> Option<&str> {
    match &self.kind {
      MediaSessionKind::Live { .. } => None,
      MediaSessionKind::Playlist { resumed_from, .. } => resumed_from.as_ref().map(|s| s.as_str()),
    }
  }

  pub async fn get_current_for_account(
    account_id: &str,
  ) -> Result<Option<MediaSession>, mongodb::error::Error> {
    let filter = doc! {
      MediaSession::KEY_ACCOUNT_ID: account_id,
      MediaSessionState::KEY_ENUM_TAG: MediaSessionState::TAG_OPEN
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
    let account_id = IndexModel::builder()
      .keys(doc! { MediaSession::KEY_ACCOUNT_ID: 1 })
      .build();
    let state = IndexModel::builder()
      .keys(doc! { MediaSessionState::KEY_ENUM_TAG: 1 })
      .build();
    let kind = IndexModel::builder()
      .keys(doc! { MediaSessionKind::KEY_ENUM_TAG: 1 })
      .build();
    let closed_at = IndexModel::builder()
      .keys(doc! { MediaSessionState::KEY_CLOSED_AT: 1 })
      .build();

    vec![account_id, state, kind, closed_at]
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{account::Account, audio_file::AudioFile};

  #[test]
  fn serde() {
    logger::init();

    let doc = MediaSession {
      id: MediaSession::uid(),
      account_id: Account::uid(),
      created_at: DateTime::now(),
      updated_at: DateTime::now(),
      kind: MediaSessionKind::Playlist {
        resumed_from: None,
        last_audio_file_id: Some(AudioFile::uid()),
        last_audio_chunk_i: 0.0,
        last_audio_chunk_skip_parts: 1,
        last_audio_chunk_date: DateTime::now(),
      },
      state: MediaSessionState::Closed {
        closed_at: DateTime::now(),
        duration_ms: 100,
      },
    };

    let buf = mongodb::bson::to_vec(&doc).unwrap();

    let target: MediaSession = mongodb::bson::from_slice(&buf).unwrap();

    assert_eq!(doc, target);
  }
}

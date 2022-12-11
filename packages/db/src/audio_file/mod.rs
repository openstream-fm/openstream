use crate::Model;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::as_f64;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(rename_all = "camelCase")]
pub struct AudioFile {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,
  pub md5: String,

  #[serde(with = "as_f64")]
  #[ts(type = "number")]
  pub len: usize,

  pub duration_ms: f64,

  #[serde(with = "as_f64")]
  #[ts(type = "number")]
  pub bytes_sec: usize,

  #[serde(with = "as_f64")]
  #[ts(type = "number")]
  pub chunk_count: usize,

  #[serde(with = "as_f64")]
  #[ts(type = "number")]
  pub chunk_len: usize,

  pub chunk_duration_ms: f64,

  #[serde(with = "serde_util::datetime")]
  pub created_at: DateTime<Utc>,

  pub filename: String,

  pub metadata: Metadata,
}

impl Model for AudioFile {
  const UID_LEN: usize = 12;
  const CL_NAME: &'static str = "audio_files";

  fn indexes() -> Vec<IndexModel> {
    let account_id = IndexModel::builder().keys(doc! { "accountId": 1 }).build();
    let md5 = IndexModel::builder().keys(doc! { "md5": 1 }).build();

    vec![account_id, md5]
  }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[ts(rename = "AudioMetadata")]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
  pub title: Option<String>,
  pub artist: Option<String>,
  pub album: Option<String>,
  pub album_artist: Option<String>,
  pub genre: Option<String>,
  pub year: Option<i32>,
  pub comment: Option<String>,
  #[serde(with = "as_f64::option")]
  pub track: Option<u16>,
}

impl Metadata {
  pub fn from_pairs(iter: impl Iterator<Item = (String, String)>) -> Self {
    let mut meta = Self::default();
    for (name, value) in iter {
      match name.as_ref() {
        "title" => meta.title = Some(value),
        "artist" => meta.artist = Some(value),
        "album" => meta.album = Some(value),
        "album_artist" => meta.album_artist = Some(value),
        "genre" => meta.genre = Some(value),
        "comment" => meta.comment = Some(value),
        "track" => {
          let r = value.parse();
          match r {
            Err(_) => continue,
            Ok(v) => meta.track = Some(v),
          }
        }
        "date" => {
          let r = value.parse();
          match r {
            Err(_) => continue,
            Ok(v) => meta.year = Some(v),
          }
        }
        _ => continue,
      }
    }
    meta
  }
}

impl<I: Iterator<Item = (String, String)>> From<I> for Metadata {
  fn from(iter: I) -> Self {
    Metadata::from_pairs(iter)
  }
}

use crate::{account::Account, audio_chunk::AudioChunk, run_transaction, Model};
use log::warn;
use mongodb::{bson::doc, ClientSession, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::{as_f64, DateTime};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct AudioFile {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,
  pub md5: String,

  #[serde(with = "as_f64")]
  pub len: u64,

  pub duration_ms: f64,

  #[serde(with = "as_f64")]
  pub bytes_sec: usize,

  #[serde(with = "as_f64")]
  pub chunk_count: usize,

  #[serde(with = "as_f64")]
  pub chunk_len: usize,

  pub chunk_duration_ms: f64,

  pub created_at: DateTime,

  pub filename: String,

  pub metadata: Metadata,
}

impl AudioFile {
  pub async fn delete_audio_file_with_session(
    account_id: &str,
    file_id: &str,
    session: &mut ClientSession,
  ) -> Result<Option<AudioFile>, mongodb::error::Error> {
    let audio_file = match Self::get_by_id_with_session(file_id, session).await? {
      None => return Ok(None),
      Some(audio_file) => {
        if audio_file.account_id == account_id {
          audio_file
        } else {
          return Ok(None);
        }
      }
    };

    // delete chunks
    AudioChunk::delete_by_audio_file_id_with_session(&audio_file.id, session).await?;

    // delete file
    AudioFile::delete_by_id_with_session(&audio_file.id, session).await?;

    // get account
    let account = Account::get_by_id_with_session(account_id, session).await?;

    // this should always be Some
    if let Some(mut account) = account {
      // applying limits update
      account.limits.storage.used = account.limits.storage.used.saturating_sub(audio_file.len);
      Account::replace_with_session(&account.id, &account, session).await?;
    } else {
      warn!(
        "deleting audio file {}: account not found, account_id = {}",
        &audio_file.id, account_id
      )
    }

    Ok(Some(audio_file))
  }

  pub async fn delete_audio_file(
    account_id: &str,
    file_id: &str,
  ) -> Result<Option<AudioFile>, mongodb::error::Error> {
    run_transaction!(session => {
      let file = tx_try!(Self::delete_audio_file_with_session(account_id, file_id, &mut session).await);
      Ok(file)
    })
  }
}

impl Model for AudioFile {
  const UID_LEN: usize = 16;
  const CL_NAME: &'static str = "audio_files";

  fn indexes() -> Vec<IndexModel> {
    let account_id = IndexModel::builder()
      .keys(doc! { Self::KEY_ACCOUNT_ID: 1 })
      .build();
    let md5 = IndexModel::builder()
      .keys(doc! { Self::KEY_MD5: 1 })
      .build();
    vec![account_id, md5]
  }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, TS)]
#[ts(export, export_to = "../../defs/db/", rename = "AudioMetadata")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
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

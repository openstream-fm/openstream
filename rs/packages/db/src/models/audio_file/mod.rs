use crate::{
  account::{Account, Limit, Limits},
  audio_chunk::AudioChunk,
  run_transaction, Model,
};
use mongodb::{
  bson::{doc, Document},
  options::FindOneOptions,
  ClientSession, IndexModel,
};
use serde::{Deserialize, Serialize};
use serde_util::{as_f64, DateTime};
use ts_rs::TS;

crate::register!(AudioFile);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct AudioFile {
  #[serde(rename = "_id")]
  pub id: String,
  pub station_id: String,
  pub sha256: String,

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

  pub filename: String,

  pub metadata: Metadata,

  pub order: f64,

  pub created_at: DateTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[macros::keys]
#[serde(rename_all = "snake_case")]
pub struct OrderDocument {
  pub order: f64,
}

impl OrderDocument {
  pub fn projection() -> Document {
    doc! { crate::KEY_ID: 0, OrderDocument::KEY_ORDER: 1 }
  }
}

impl AudioFile {
  pub async fn next_max_order(
    station_id: &str,
    session: Option<&mut ClientSession>,
  ) -> Result<f64, mongodb::error::Error> {
    let sort = doc! { AudioFile::KEY_ORDER: -1 };
    let options = FindOneOptions::builder()
      .sort(sort)
      .projection(OrderDocument::projection())
      .build();
    let filter = doc! { AudioFile::KEY_STATION_ID: station_id };

    let document = match session {
      None => {
        Self::cl_as::<OrderDocument>()
          .find_one(filter, options)
          .await?
      }
      Some(session) => {
        Self::cl_as::<OrderDocument>()
          .find_one_with_session(filter, options, session)
          .await?
      }
    };

    let n = match document {
      None => 1.0 + rand::random::<f64>(),
      Some(doc) => doc.order + 1.0 + rand::random::<f64>(),
    };

    Ok(n)
  }

  pub async fn next_min_order(
    station_id: &str,
    session: Option<&mut ClientSession>,
  ) -> Result<f64, mongodb::error::Error> {
    let sort = doc! { AudioFile::KEY_ORDER: 1 };
    let options = FindOneOptions::builder()
      .sort(sort)
      .projection(OrderDocument::projection())
      .build();
    let filter = doc! { AudioFile::KEY_STATION_ID: station_id };

    let document = match session {
      None => {
        Self::cl_as::<OrderDocument>()
          .find_one(filter, options)
          .await?
      }
      Some(session) => {
        Self::cl_as::<OrderDocument>()
          .find_one_with_session(filter, options, session)
          .await?
      }
    };

    let n = match document {
      None => -1.0 - rand::random::<f64>(),
      Some(doc) => doc.order - 1.0 - rand::random::<f64>(),
    };

    Ok(n)
  }

  pub async fn delete_audio_file_with_session(
    station_id: &str,
    file_id: &str,
    session: &mut ClientSession,
  ) -> Result<Option<AudioFile>, mongodb::error::Error> {
    let audio_file = match Self::get_by_id_with_session(file_id, session).await? {
      None => return Ok(None),
      Some(audio_file) => {
        if audio_file.station_id == station_id {
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

    // update station
    const KEY: &str = const_str::concat!(
      Account::KEY_LIMITS,
      ".",
      Limits::KEY_STORAGE,
      ".",
      Limit::KEY_USED
    );
    let update = doc! { "$inc": { KEY: (audio_file.len as f64) * -1.0 } };
    Account::update_by_id_with_session(&audio_file.station_id, update, session).await?;

    Ok(Some(audio_file))
  }

  pub async fn delete_audio_file(
    station_id: &str,
    file_id: &str,
  ) -> Result<Option<AudioFile>, mongodb::error::Error> {
    run_transaction!(session => {
      let file = tx_try!(Self::delete_audio_file_with_session(station_id, file_id, &mut session).await);
      Ok(file)
    })
  }

  pub async fn playlist_first(
    station_id: &str,
  ) -> Result<Option<AudioFile>, mongodb::error::Error> {
    let filter = doc! { Self::KEY_STATION_ID: station_id };
    let sort = doc! { Self::KEY_ORDER: 1 };
    let options = FindOneOptions::builder().sort(sort).build();
    Self::cl().find_one(filter, options).await
  }

  pub async fn playlist_next(
    station_id: &str,
    current_id: &str,
    current_order: f64,
  ) -> Result<Option<AudioFile>, mongodb::error::Error> {
    let filter = doc! { Self::KEY_ID: current_id, Self::KEY_STATION_ID: station_id };
    let order_projection = doc! { Self::KEY_ID: -1, Self::KEY_ORDER: 1 };
    let options = FindOneOptions::builder()
      .projection(order_projection)
      .build();
    let current_order_document = Self::cl_as::<OrderDocument>()
      .find_one(filter, options)
      .await?;

    // if cant update the current order we use the last one (when the file was present)
    let current_order = match current_order_document {
      None => current_order,
      Some(document) => document.order,
    };

    let filter =
      doc! { Self::KEY_STATION_ID: station_id, Self::KEY_ORDER: { "$gt": current_order } };
    let sort = doc! { Self::KEY_ORDER: 1 };

    let options = FindOneOptions::builder().sort(sort).build();

    let next = Self::cl().find_one(filter, options).await?;

    let next = match next {
      None => return Self::playlist_first(station_id).await,
      Some(next) => next,
    };

    Ok(Some(next))
  }
}

impl Model for AudioFile {
  const UID_LEN: usize = 16;
  const CL_NAME: &'static str = "audio_files";

  fn indexes() -> Vec<IndexModel> {
    let station_id = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1 })
      .build();

    let station_id_order = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1, Self::KEY_ORDER: 1 })
      .build();

    vec![station_id, station_id_order]
  }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, TS)]
#[ts(export, export_to = "../../../defs/db/", rename = "AudioMetadata")]
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, AudioFile::KEY_ID);
    assert_eq!(AudioFile::KEY_ORDER, OrderDocument::KEY_ORDER);
  }
}

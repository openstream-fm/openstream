use crate::json::JsonHandler;

use async_trait::async_trait;
use db::station::Station;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use db::{
    audio_file::AudioFile,
    media_session::{MediaSession, MediaSessionKind, MediaSessionNowPlaying},
    probe::Probe,
    Model,
  };
  use schemars::JsonSchema;

  use crate::error::ApiError;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/embed/station/[station]/GET/")]
  #[macros::schema_ts_export]
  pub struct Output {
    pub id: String,
    pub name: String,
    pub slogan: Option<String>,
    pub picture_id: String,
    pub now_playing: Option<String>,
    pub transmission_error: Option<String>,
  }

  #[derive(Debug, Clone, thiserror::Error)]
  pub enum ParseError {
    #[error("mongodb error")]
    Db(#[from] mongodb::error::Error),
    #[error("station with id {station_id} not found")]
    StationNotFound { station_id: String },
  }

  impl From<ParseError> for ApiError {
    fn from(err: ParseError) -> Self {
      match err {
        ParseError::Db(e) => e.into(),
        ParseError::StationNotFound { station_id } => ApiError::StationNotFound(station_id),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = mongodb::error::Error;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = req.param("station").unwrap();
      let station = match Station::get_by_id(station_id).await? {
        Some(station) => station,
        None => {
          return Err(ParseError::StationNotFound {
            station_id: station_id.to_string(),
          })
        }
      };

      Ok(Self::Input { station })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { station } = input;

      let mut transmission_error: Option<String> = None;
      let mut now_playing: Option<String> = None;

      match MediaSession::get_current_for_station(&station.id).await? {
        None => match station.external_relay_url {
          Some(url) => {
            let probe = Probe::last_for_url(&url).await?;

            match probe {
              None => {}
              Some(doc) => match doc.result {
                db::probe::ProbeResult::Ok { .. } => {}
                db::probe::ProbeResult::Error { .. } => {
                  transmission_error = Some("Error in relay".to_string());
                }
              },
            }
          }

          None => {
            let filter = doc! { AudioFile::KEY_STATION_ID: &station.id };
            let exists = AudioFile::exists(filter).await?;
            if !exists {
              transmission_error = Some("The station is not transmitting".to_string());
            }
          }
        },
        Some(media_session) => match media_session.kind {
          MediaSessionKind::Live { .. } => match media_session.now_playing {
            None => {}
            Some(MediaSessionNowPlaying { title, artist }) => match artist {
              Some(artist) => {
                now_playing = Some(format!("{} - {}", title, artist));
              }
              None => {
                now_playing = Some(title);
              }
            },
          },

          MediaSessionKind::ExternalRelay { .. } => {}

          MediaSessionKind::Playlist {
            last_audio_file_id, ..
          } => match AudioFile::get_by_id(&last_audio_file_id).await? {
            // this would never happen
            None => {}
            Some(file) => match (file.metadata.title, file.metadata.artist) {
              (Some(title), Some(artist)) => {
                now_playing = Some(format!("{} - {}", title, artist));
              }
              (Some(title), None) => {
                now_playing = Some(title);
              }
              (None, Some(artist)) => {
                now_playing = Some(artist);
              }
              (None, None) => {}
            },
          },
        },
      };

      let out = Output {
        id: station.id,
        name: station.name,
        slogan: station.slogan,
        picture_id: station.picture_id,
        now_playing,
        transmission_error,
      };

      Ok(out)
    }
  }
}

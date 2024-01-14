use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

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

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/now-playing/GET/"
  )]
  #[macros::schema_ts_export]
  #[serde(tag = "kind")]
  pub enum Output {
    #[serde(rename = "none")]
    None {
      start_on_connect: bool,
      external_relay_url: Option<String>,
      external_relay_error: Option<String>,
    },
    #[serde(rename = "live")]
    Live {
      title: Option<String>,
      artist: Option<String>,
    },
    #[serde(rename = "external-relay")]
    ExternalRelay { url: String },
    #[serde(rename = "playlist")]
    Playilist {
      file_id: String,
      filename: String,
      title: Option<String>,
      artist: Option<String>,
    },
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = mongodb::error::Error;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = req.param("station").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let station = access_token_scope.grant_station_scope(station_id).await?;
      Ok(Self::Input { station })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { station } = input;

      let out = match MediaSession::get_current_for_station(&station.id).await? {
        None => match station.external_relay_url {
          Some(url) => {
            let probe = Probe::last_for_url(&url).await?;

            let error_display = match probe {
              None => None,
              Some(doc) => match doc.result {
                db::probe::ProbeResult::Ok { .. } => None,
                db::probe::ProbeResult::Error { error_display, .. } => Some(error_display),
              },
            };

            Output::None {
              start_on_connect: true,
              external_relay_url: Some(url),
              external_relay_error: error_display,
            }
          }

          None => {
            let filter = doc! { AudioFile::KEY_STATION_ID: &station.id };
            let exists = AudioFile::exists(filter).await?;
            Output::None {
              start_on_connect: exists,
              external_relay_url: None,
              external_relay_error: None,
            }
          }
        },
        Some(media_session) => match media_session.kind {
          MediaSessionKind::Live { .. } => match media_session.now_playing {
            None => Output::Live {
              title: None,
              artist: None,
            },
            Some(MediaSessionNowPlaying { title, artist }) => Output::Live {
              title: Some(title),
              artist,
            },
          },

          MediaSessionKind::ExternalRelay { url } => Output::ExternalRelay { url },

          MediaSessionKind::Playlist {
            last_audio_file_id, ..
          } => match AudioFile::get_by_id(&last_audio_file_id).await? {
            // this would never happen
            None => {
              let filter = doc! { AudioFile::KEY_STATION_ID: &station.id };
              let exists = AudioFile::exists(filter).await?;
              Output::None {
                start_on_connect: exists,
                external_relay_url: None,
                external_relay_error: None,
              }
            }
            Some(file) => Output::Playilist {
              file_id: file.id,
              filename: file.filename,
              title: file.metadata.title,
              artist: file.metadata.artist,
            },
          },
        },
      };

      Ok(out)
    }
  }
}

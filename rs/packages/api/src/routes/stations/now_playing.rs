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
    Model,
  };

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/stations/[station]/now-playing/GET/")]
  #[serde(tag = "kind")]
  pub enum Output {
    #[serde(rename = "none")]
    None { start_on_connect: bool },
    #[serde(rename = "live")]
    Live {
      title: Option<String>,
      artist: Option<String>,
    },
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
        None => {
          let filter = doc! { AudioFile::KEY_STATION_ID: &station.id };
          let exists = AudioFile::exists(filter).await?;
          Output::None {
            start_on_connect: exists,
          }
        }
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
          MediaSessionKind::Playlist {
            last_audio_file_id, ..
          } => match AudioFile::get_by_id(&last_audio_file_id).await? {
            // this would never happen
            None => {
              let filter = doc! { AudioFile::KEY_STATION_ID: &station.id };
              let exists = AudioFile::exists(filter).await?;
              Output::None {
                start_on_connect: exists,
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

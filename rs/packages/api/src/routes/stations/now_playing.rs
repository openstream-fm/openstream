use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::Station;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use db::{
    audio_file::AudioFile,
    media_session::{MediaSession, MediaSessionKind},
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
    Live,
    #[serde(rename = "playlist")]
    Playilist { file: AudioFile },
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
        None => Output::None {
          start_on_connect: station.limits.storage.used != 0,
        },
        Some(media_session) => match media_session.kind {
          MediaSessionKind::Live { .. } => Output::Live,
          MediaSessionKind::Playlist {
            last_audio_file_id, ..
          } => match AudioFile::get_by_id(&last_audio_file_id).await? {
            // this would never happen
            None => Output::None {
              start_on_connect: station.limits.storage.used != 0,
            },
            Some(file) => Output::Playilist { file },
          },
        },
      };

      Ok(out)
    }
  }
}

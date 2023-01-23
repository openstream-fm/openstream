use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::Station;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod post {

  use db::media_session::{MediaSession, MediaSessionKind};
  use drop_tracer::DropTracer;
  use media_sessions::RestartError;
  use serde_util::empty_struct::EmptyStruct;
  use shutdown::Shutdown;

  use crate::error::ApiError;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub drop_tracer: DropTracer,
    pub shutdown: Shutdown,
    pub media_sessions: media_sessions::MediaSessionMap,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/stations/[station]/restart-playlist/POST/")]
  pub struct Output(EmptyStruct);

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("station is currently live streaming")]
    StationIsLive,
    #[error("no media files in station")]
    NoFiles,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::StationIsLive => ApiError::PlaylistStartIsLive,
        HandleError::NoFiles => ApiError::PlaylistStartNoFiles,
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = req.param("station").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let station = access_token_scope.grant_station_scope(station_id).await?;
      Ok(Self::Input { station })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { station } = input;

      match MediaSession::get_current_for_station(&station.id).await? {
        None => {
          if station.limits.storage.used == 0 {
            return Err(HandleError::NoFiles);
          }
        }

        Some(media_session) => match media_session.kind {
          MediaSessionKind::Live { .. } => return Err(HandleError::StationIsLive),
          MediaSessionKind::Playlist { .. } => {}
        },
      };

      match self.media_sessions.write().restart(
        &station.id,
        self.shutdown.clone(),
        self.drop_tracer.clone(),
      ) {
        Ok(_) => {}
        Err(e) => match e {
          RestartError::LiveStreaming => return Err(HandleError::StationIsLive),
        },
      }

      Ok(Output(EmptyStruct(())))
    }
  }
}

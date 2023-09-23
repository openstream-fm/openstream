use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod post {

  use std::convert::Infallible;

  use drop_tracer::DropTracer;
  use media_sessions::MediaSessionMap;
  use serde_util::empty_struct::EmptyStruct;
  use shutdown::Shutdown;

  use crate::error::ApiError;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub deployment_id: String,
    pub media_sessions: MediaSessionMap,
    pub drop_tracer: DropTracer,
    pub shutdown: Shutdown,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    station_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/runtime/restart-playlist/[station]/POST/")]
  pub struct Output(EmptyStruct);

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("station is live streaming")]
    StationIsLive,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::StationIsLive => ApiError::PlaylistStartIsLive,
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = Infallible;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = req.param("station").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let _ = access_token_scope.grant_station_scope(station_id).await?;
      Ok(Self::Input {
        station_id: station_id.to_string(),
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { station_id } = input;
      let mut lock = self.media_sessions.write();
      let _ = lock
        .restart(
          station_id.to_string(),
          self.deployment_id.to_string(),
          self.shutdown.clone(),
          self.drop_tracer.clone(),
        )
        .await;
      Ok(Output(EmptyStruct(())))
    }
  }
}

use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use media::MediaSessionMap;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_util::empty_struct::EmptyStruct;
use std::convert::Infallible;
use ts_rs::TS;

pub mod post {

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub media_sessions: MediaSessionMap,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    station_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/runtime/station-deleted/[station]/POST/")]
  pub struct Output(EmptyStruct);

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = Infallible;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = req.param("station").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let station = access_token_scope
        .grant_station_owner_scope(station_id)
        .await?;
      Ok(Self::Input {
        station_id: station.id,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { station_id } = input;
      perform(&self.media_sessions, &station_id).await;
      Ok(Output(EmptyStruct(())))
    }
  }
}

pub async fn perform(media_sessions: &MediaSessionMap, station_id: &str) {
  media_sessions.terminate(station_id).await;
}

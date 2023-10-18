use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::Station;
use media::{Kind, MediaSessionMap};
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
    station: Station,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/runtime/external-relay-updated/[station]/POST/")]
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
      let station = access_token_scope.grant_station_scope(station_id).await?;
      Ok(Self::Input { station })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { station } = input;
      perform(&self.media_sessions, &station).await;
      Ok(Output(EmptyStruct(())))
    }
  }
}

pub async fn perform(media_sessions: &MediaSessionMap, station: &Station) {
  let mut lock = media_sessions.lock(&station.id).await;
  match &*lock {
    None => {}
    Some(handle) => match (&station.external_relay_url, handle.info().kind) {
      (Some(_), Kind::ExternalRelay | Kind::Playlist) | (None, Kind::ExternalRelay) => {
        handle.terminate();
        *lock = None;
      }
      _ => {}
    },
  }
}

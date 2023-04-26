use std::collections::btree_map::Entry;

use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use media_sessions::MediaSessionMap;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod post {

  use std::convert::Infallible;

  use media_sessions::MediaSessionMap;
  use serde_util::empty_struct::EmptyStruct;

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
  #[ts(export_to = "../../../defs/api/runtime/source-password-updated/[station_id]/POST/")]
  pub struct Output(EmptyStruct);

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = Infallible;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = req.param("station_id").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let _ = access_token_scope.grant_station_scope(station_id).await?;
      Ok(Self::Input {
        station_id: station_id.to_string(),
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { station_id } = input;
      perform(&self.media_sessions, station_id);
      Ok(Output(EmptyStruct(())))
    }
  }
}

pub fn perform(media_sessions: &MediaSessionMap, station_id: String) {
  let mut lock = media_sessions.write();
  match lock.entry(&station_id) {
    Entry::Vacant(..) => {}
    Entry::Occupied(entry) => {
      let session = entry.get();
      if session.is_live() {
        entry.remove();
      }
    }
  }
}

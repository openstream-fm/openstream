use std::collections::btree_map::Entry;

use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::Station;
use media_sessions::MediaSessionMap;
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
      perform(&self.media_sessions, &station);
      Ok(Output(EmptyStruct(())))
    }
  }
}

pub fn perform(media_sessions: &MediaSessionMap, station: &Station) {
  let mut lock = media_sessions.write();
  match lock.entry(&station.id) {
    Entry::Vacant(..) => {}
    Entry::Occupied(entry) => {
      let session = entry.get();
      match &station.external_relay_url {
        Some(_) => {
          if session.is_external_relay() || session.is_playlist() {
            entry.remove();
          }
        }
        None => {
          if session.is_external_relay() {
            entry.remove();
          }
        }
      }
    }
  }
}

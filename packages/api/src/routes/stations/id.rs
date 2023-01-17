use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::PublicStation;
use db::station::Station;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use std::convert::Infallible;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/stations/[station]/GET/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    station: PublicStation,
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

      let station = access_token_scope.grant_station_scope(station_id).await?;

      Ok(Self::Input {
        access_token_scope,
        station,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope,
        station,
      } = input;

      let station = station.into_public(access_token_scope.as_public_scope());

      Ok(Output { station })
    }
  }
}

pub mod patch {

  use crate::error::ApiError;

  use super::*;
  use db::{
    error::ApplyPatchError, fetch_and_patch, run_transaction, station::StationPatch, Model,
  };
  use prex::request::ReadBodyJsonError;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/stations/[station]/PATCH/")]
  pub struct Payload(pub StationPatch);

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
    access_token_scope: AccessTokenScope,
    station: Station,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/stations/[station]/PATCH/")]
  pub struct Output(pub PublicStation);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => Self::from(e),
        ParseError::Payload(e) => Self::from(e),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("apply patch: {0}")]
    Patch(#[from] ApplyPatchError),
    #[error("station not found: {0}")]
    StationNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => Self::from(e),
        HandleError::Patch(e) => Self::from(e),
        HandleError::StationNotFound(id) => Self::StationNotFound(id),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = req.param("station").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let station = access_token_scope.grant_station_scope(station_id).await?;

      let payload: Payload = req.read_body_json(100_000).await?;

      Ok(Self::Input {
        payload,
        access_token_scope,
        station,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        payload: Payload(payload),
        access_token_scope,
        station,
      } = input;

      let id = station.id;

      let station = run_transaction!(session => {
        fetch_and_patch!(Station, station, &id, Err(HandleError::StationNotFound(id)), session, {
          station.apply_patch(payload.clone(), access_token_scope.as_public_scope())?;
        })
      });
      /*
      let station = run_transaction!(session => {

        let mut station = match Station::get_by_id_with_session(&station.id, &mut session).await? {
          Some(station) => station,
          None => return Err(HandleError::StationNotFound(station.id)),
        };

        match access_token_scope {
          AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
            station.apply_admin_patch(payload)?;
          }

          AccessTokenScope::User(_) => {
            station.apply_user_patch(payload)?;
          }
        }

        Station::replace_with_session(&station.id, &station, &mut session).await?;

        station
      });
      */

      let out = station.into_public(access_token_scope.as_public_scope());

      Ok(Output(out))
    }
  }
}

use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use crate::error::ApiError;
use crate::request_ext::AccessTokenScope;
use async_trait::async_trait;
use db::station::Station;
use db::Model;
use db::{account::Account, run_transaction};
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod post {

  use db::station::PublicStation;
  use modify::Modify;
  use schemars::JsonSchema;
  use validator::Validate;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/transfer/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Payload {
    target_account_id: String,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    station_id: String,
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/transfer/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Output {
    station: PublicStation,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("payload: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("station not found: {0}")]
    StationNotFound(String),
    #[error("source account is target account")]
    SourceAccountIsTargetAccount,
    #[error("source account not found: {0}")]
    SourceAccountNotFound(String),
    #[error("target account not found: {0}")]
    TargetAccountNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::Token(e) => e.into(),
        HandleError::StationNotFound(id) => ApiError::StationNotFound(id),
        HandleError::SourceAccountNotFound(id) => {
          ApiError::Internal(format!("station's account with id {} not found", id))
        }
        HandleError::TargetAccountNotFound(id) => {
          ApiError::BadRequestCustom(format!("Target account with id {} not found", id))
        }
        HandleError::SourceAccountIsTargetAccount => ApiError::BadRequestCustom(String::from(
          "cannot transfer station, source account is the same as target account",
        )),
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
      let station_id = req.param("station").unwrap().to_string();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let payload: Payload = req.read_body_json(1_000).await?;

      Ok(Input {
        station_id,
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      // this action requires owner scope to source account and read scope to target account

      let Input {
        access_token_scope,
        station_id,
        payload,
      } = input;
      let Payload { target_account_id } = payload;

      access_token_scope
        .grant_account_scope(&target_account_id)
        .await?;

      let station = run_transaction!(session => {
        let mut station = match tx_try!(Station::get_by_id_with_session(&station_id, &mut session).await) {
          Some(station) => station,
          None => return Err(HandleError::StationNotFound(station_id))
        };

        access_token_scope.grant_account_owner_scope(&station.account_id).await?;

        if station.account_id == target_account_id {
          return Err(HandleError::SourceAccountIsTargetAccount);
        }

        let mut source_account = match tx_try!(Account::get_by_id_with_session(&station.account_id, &mut session).await) {
          Some(account) => account,
          None => return Err(HandleError::SourceAccountNotFound(station.account_id))
        };

        let mut target_account = match tx_try!(Account::get_by_id_with_session(&target_account_id, &mut session).await) {
          Some(account) => account,
          None => return Err(HandleError::TargetAccountNotFound(target_account_id))
        };

        let used_storage = tx_try!(Station::get_used_storage_with_session(&station.id, &mut session).await);

        station.account_id = target_account.id.clone();
        // We do not update the listeners limits here, as when stream connections terminate
        // they will decrease the source account instead of the target account
        source_account.limits.stations.used = source_account.limits.stations.used.saturating_sub(1);
        source_account.limits.storage.used = source_account.limits.storage.used.saturating_sub(used_storage);

        target_account.limits.storage.used += used_storage;
        target_account.limits.stations.used += 1;

        tx_try!(Station::replace_with_session(&station.id, &station, &mut session).await);
        tx_try!(Account::replace_with_session(&source_account.id, &source_account, &mut session).await);
        tx_try!(Account::replace_with_session(&target_account.id, &target_account, &mut session).await);

        station
      });

      Ok(Output {
        station: station.into_public(access_token_scope.as_public_scope()),
      })
    }
  }
}

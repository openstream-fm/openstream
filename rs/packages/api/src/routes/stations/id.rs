use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::PublicStation;
use db::station::Station;
use db::station_picture::StationPicture;
use mongodb::bson::doc;
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
  #[ts(export_to = "../../../defs/api/stations/[station]/GET/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    pub station: PublicStation,
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

pub mod delete {

  use db::account::{Account, Limit, Limits};
  use db::audio_file::AudioFile;
  use db::{run_transaction, Model};
  use serde_util::DateTime;
  // use futures_util::TryStreamExt;
  use serde_util::empty_struct::EmptyStruct;

  use crate::error::ApiError;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    station_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/stations/[station]/DELETE/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output(EmptyStruct);

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("station not found: {0}")]
    NotFound(String),
    #[error("account not found: {0}")]
    AccountNotFound(String),
    #[error("station already deleted: {0}")]
    AlreadyDeleted(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::AlreadyDeleted(id) => ApiError::StationNotFound(id),
        HandleError::NotFound(id) => ApiError::StationNotFound(id),
        HandleError::AccountNotFound(id) => ApiError::AccountNotFound(id),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Input, Self::ParseError> {
      let station_id = req.param("station").unwrap();
      let access_token = request_ext::get_access_token_scope(&req).await?;
      let station = access_token.grant_station_owner_scope(station_id).await?;
      Ok(Input {
        station_id: station.id,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Input { station_id } = input;

      run_transaction!(session => {

        let station = match tx_try!(Station::get_by_id_with_session(&station_id, &mut session).await) {
          None => return Err(HandleError::NotFound(station_id)),
          Some(station) => station,
        };

        if station.deleted_at.is_some() {
          return Err(HandleError::AlreadyDeleted(station.id));
        }

        let mut storage_used: u64 = 0;

        let files_filter = doc! { AudioFile::KEY_STATION_ID: &station.id };

        let mut cursor = tx_try!(AudioFile::cl().find_with_session(files_filter, None, &mut session).await);

        while let Some(item) = tx_try!(cursor.next(&mut session).await.transpose()) {
          storage_used += item.len;
         // TODO:
         // should we delete files or create a background job that fully delete station files
         // when the station is finally deleted?
         // tx_try!(AudioFile::delete_by_id_with_session(&item.id, &mut session).await);
        };

        const KEY_STATIONS_USED: &str = db::key!(Account::KEY_LIMITS, Limits::KEY_STATIONS, Limit::KEY_USED);
        const KEY_STORAGE_USED: &str = db::key!(Account::KEY_LIMITS, Limits::KEY_STORAGE, Limit::KEY_USED);

        let now = DateTime::now();

        let account_update = doc!{
          "$set": {
            Account::KEY_UPDATED_AT: now,
          },
          "$inc": {
            KEY_STATIONS_USED: -1.0,
            KEY_STORAGE_USED: storage_used as f64 * -1.0
          }
        };

        tx_try!(Station::set_deleted_by_id_with_session(&station.id, &mut session).await);
        let result = tx_try!(Account::update_by_id_with_session(&station.account_id, account_update, &mut session).await);

        if result.matched_count != 1 {
          return Err(HandleError::AccountNotFound(station.account_id));
        }
      });

      Ok(Output(EmptyStruct(())))
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
  use validify::{ValidationErrors, Validify};

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/[station]/PATCH/")]
  pub struct Payload(pub StationPatch);

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
    access_token_scope: AccessTokenScope,
    station: Station,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/[station]/PATCH/")]
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
    #[error("picture not found {0}")]
    PictureNotFound(String),
    #[error("validation: {0}")]
    Validation(#[from] ValidationErrors),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => Self::from(e),
        HandleError::Patch(e) => Self::from(e),
        HandleError::StationNotFound(id) => Self::StationNotFound(id),
        HandleError::Validation(e) => Self::PayloadInvalid(format!("{e}")),
        HandleError::PictureNotFound(id) => {
          Self::PayloadInvalid(format!("Picture with id {id} not found"))
        }
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
        payload: Payload(patch),
        access_token_scope,
        station,
      } = input;

      let id = station.id;

      let patch: StationPatch = Validify::validify(patch.into())?;

      let station = run_transaction!(session => {
        fetch_and_patch!(Station, station, &id, Err(HandleError::StationNotFound(id)), session, {
          if let Some(picture_id) = &patch.picture_id {
            let filter = doc! { StationPicture::KEY_ACCOUNT_ID: &station.account_id, StationPicture::KEY_ID: picture_id };
            match tx_try!(StationPicture::exists_with_session(filter, &mut session).await) {
              true => {},
              false => {
                return Err(HandleError::PictureNotFound(picture_id.to_string()))
              }
            }
          }
          station.apply_patch(patch.clone(), access_token_scope.as_public_scope())?;
        })
      });

      let out = station.into_public(access_token_scope.as_public_scope());

      Ok(Output(out))
    }
  }
}

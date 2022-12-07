use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::account::Account;
use db::audio_file::AudioFile;
use db::Model;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

  use ts_rs::TS;

  use crate::error::{ApiError, Kind};

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    account: Account,
    #[allow(unused)]
    access_token_scope: AccessTokenScope,
    file_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/[account]/files/[file]/GET/")]
  #[serde(rename_all = "camelCase")]
  pub struct Output {
    item: AudioFile,
  }

  #[derive(Debug)]
  pub enum HandleError {
    Db(mongodb::error::Error),
    FileNotFound(String),
  }

  impl From<mongodb::error::Error> for HandleError {
    fn from(e: mongodb::error::Error) -> Self {
      Self::Db(e)
    }
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::FileNotFound(id) => Self::from(Kind::AudioFileNotFound(id)),
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
      let account_id = req.param("account").unwrap();
      let file_id = req.param("file").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let account = access_token_scope.grant_scope(account_id).await?;

      Ok(Self::Input {
        access_token_scope,
        account,
        file_id: file_id.to_string(),
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope: _,
        account,
        file_id,
      } = input;

      let filter = doc! { "_id": &file_id, "accountId": account.id };

      let item = match AudioFile::cl().find_one(filter, None).await? {
        None => return Err(HandleError::FileNotFound(file_id)),
        Some(f) => f,
      };

      Ok(Output { item })
    }
  }
}

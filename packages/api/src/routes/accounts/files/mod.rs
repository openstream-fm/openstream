pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::account::Account;
use db::audio_file::AudioFile;
use db::Model;
use db::Paged;
use prex::Request;
use serde::Deserialize;

pub mod get {

  use serde::Serialize;
  use ts_rs::TS;

  use crate::error::ApiError;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  pub const DEFAULT_SKIP: u64 = 0;
  pub const DEFAULT_LIMIT: i64 = 60;

  fn default_skip() -> u64 {
    DEFAULT_SKIP
  }

  fn default_limit() -> i64 {
    DEFAULT_LIMIT
  }

  #[derive(Debug, Serialize, Deserialize, Clone, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/[account]/files/GET/")]
  pub struct Output(Paged<AudioFile>);

  #[derive(Debug, Serialize, Deserialize, Default, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/[account]/files/GET/")]
  struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(type = "number")]
    skip: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(type = "number")]
    limit: Option<i64>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    account: Account,
    #[allow(unused)]
    access_token_scope: AccessTokenScope,
    skip: u64,
    limit: i64,
  }

  #[derive(Debug)]
  pub enum ParseError {
    Access(GetAccessTokenScopeError),
    QueryString(serde_querystring::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::QueryString(e) => e.into(),
      }
    }
  }

  impl From<GetAccessTokenScopeError> for ParseError {
    fn from(e: GetAccessTokenScopeError) -> Self {
      Self::Access(e)
    }
  }

  impl From<serde_querystring::Error> for ParseError {
    fn from(e: serde_querystring::Error) -> Self {
      Self::QueryString(e)
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = mongodb::error::Error;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let account_id = req.param("account").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let account = access_token_scope.grant_scope(account_id).await?;

      let Query { skip, limit } = match req.uri().query() {
        None => Default::default(),
        Some(qs) => serde_querystring::from_str(qs)?,
      };

      Ok(Self::Input {
        access_token_scope,
        account,
        skip: skip.unwrap_or_else(default_skip),
        limit: limit.unwrap_or_else(default_limit),
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope: _,
        account,
        skip,
        limit,
      } = input;

      let filter = mongodb::bson::doc! { "accountId": account.id };

      let page = AudioFile::paged(filter, skip, limit).await?;

      Ok(Output(page))
    }
  }
}

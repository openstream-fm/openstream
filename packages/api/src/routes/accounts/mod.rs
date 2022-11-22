pub mod files;
pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::account::Account;
use db::account::PublicAccount;
use prex::Request;
use serde::Deserialize;

pub mod get {

  use db::{Model, Paged};

  use crate::error::ApiError;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    skip: u64,
    limit: i64,
  }

  pub const DEFAULT_SKIP: u64 = 0;
  pub const DEFAULT_LIMIT: i64 = 60;

  fn default_skip() -> u64 {
    DEFAULT_SKIP
  }

  fn default_limit() -> i64 {
    DEFAULT_LIMIT
  }

  pub type Output = Paged<PublicAccount>;

  #[derive(Debug, Deserialize)]
  struct Query {
    #[serde(default = "default_skip")]
    skip: u64,
    #[serde(default = "default_limit")]
    limit: i64,
  }

  impl Default for Query {
    fn default() -> Self {
      Self {
        skip: DEFAULT_SKIP,
        limit: DEFAULT_LIMIT,
      }
    }
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
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let Query { skip, limit } = match req.uri().query() {
        None => Default::default(),
        Some(qs) => serde_querystring::from_str(qs)?,
      };

      Ok(Self::Input {
        access_token_scope,
        skip,
        limit,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope,
        skip,
        limit,
      } = input;

      match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin => {
          let page = Account::paged(None, skip, limit)
            .await?
            .map(|item| Account::into_public(item, true));

          Ok(page)
        }

        AccessTokenScope::User(user) => {
          let filter = mongodb::bson::doc! { "accountId": { "$in": user.account_ids } };
          let page = Account::paged(filter, skip, limit)
            .await?
            .map(|item| Account::into_public(item, false));
          Ok(page)
        }
      }
    }
  }
}

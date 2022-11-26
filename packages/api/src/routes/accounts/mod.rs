pub mod files;
pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use chrono::Utc;
use db::account::Account;
use db::account::PublicAccount;
use db::metadata::Metadata;
use db::run_transaction;
use db::{Model, Paged, PublicScope};
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

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
            .map(|item| item.into_public(PublicScope::Admin));

          Ok(page)
        }

        AccessTokenScope::User(user) => {
          let filter = mongodb::bson::doc! { "accountId": { "$in": user.account_ids } };
          let page = Account::paged(filter, skip, limit)
            .await?
            .map(|item| item.into_public(PublicScope::User));
          Ok(page)
        }
      }
    }
  }
}

pub mod post {

  use db::user::User;

  use crate::error::Kind;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct Payload {
    pub name: String,
    pub owner_id: Option<String>, // user
    pub user_metadata: Option<Metadata>,
    pub system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Output {
    account: PublicAccount,
  }

  #[derive(Debug)]
  pub enum ParseError {
    Token(GetAccessTokenScopeError),
    Payload(ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => ApiError::from(e),
        ParseError::Payload(e) => ApiError::from(e),
      }
    }
  }

  impl From<ReadBodyJsonError> for ParseError {
    fn from(e: ReadBodyJsonError) -> Self {
      ParseError::Payload(e)
    }
  }

  impl From<GetAccessTokenScopeError> for ParseError {
    fn from(e: GetAccessTokenScopeError) -> Self {
      Self::Token(e)
    }
  }

  #[derive(Debug)]
  pub enum HandleError {
    Db(mongodb::error::Error),
    NameMissing,
    OwnerIdMissing,
    OwnerIdOutOfScope,
    UserNotFound(String),
  }

  impl From<mongodb::error::Error> for HandleError {
    fn from(e: mongodb::error::Error) -> Self {
      Self::Db(e)
    }
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => ApiError::from(e),
        HandleError::NameMissing => {
          ApiError::from(Kind::PayloadInvalid(String::from("Name is required")))
        }
        HandleError::OwnerIdMissing => {
          ApiError::from(Kind::PayloadInvalid(String::from("ownerId is required")))
        }
        HandleError::OwnerIdOutOfScope => ApiError::from(Kind::PayloadInvalid(String::from(
          "Specifying ownerId field requires greater access scope",
        ))),
        HandleError::UserNotFound(id) => ApiError::from(Kind::UserNotFound(id)),
      }
    }
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let payload: Payload = req.read_body_json(10_000).await?;
      Ok(Input {
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        access_token_scope,
        payload,
      } = input;

      let Payload {
        name,
        owner_id,
        user_metadata,
        system_metadata,
      } = payload;

      let name = name.trim().to_string();

      if name.is_empty() {
        return Err(HandleError::NameMissing);
      }

      let (owner_id, system_metadata) = match &access_token_scope {
        AccessTokenScope::Admin | AccessTokenScope::Global => {
          let owner_id = match owner_id {
            None => return Err(HandleError::OwnerIdMissing),
            Some(v) => v,
          };

          (owner_id, system_metadata.unwrap_or_default())
        }

        AccessTokenScope::User(user) => {
          let owner_id = match owner_id {
            Some(_) => return Err(HandleError::OwnerIdOutOfScope),
            None => user.id.clone(),
          };

          (owner_id, Metadata::default())
        }
      };

      let now = Utc::now();

      let user_metadata = user_metadata.unwrap_or_default();

      let account = Account {
        id: Account::uid(),
        owner_id,
        name,
        system_metadata,
        user_metadata,
        created_at: now,
        updated_at: now,
      };

      run_transaction!(session => {
        let user_exists = User::exists_with_session(account.owner_id.as_ref(), &mut session).await?;
        if !user_exists {
          return Err(HandleError::UserNotFound(account.owner_id));
        }

        Account::insert_with_session(&account, &mut session).await?;
      });

      let out = Output {
        account: account.into_public(access_token_scope.as_public_scope()),
      };

      Ok(out)
    }
  }
}

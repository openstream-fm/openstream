use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use crate::error::Kind;
use crate::request_ext::get_access_token_scope;
use async_trait::async_trait;
use db::account::Account;
use db::metadata::Metadata;
use db::user::{PublicUser, User};
use db::{Model, Paged, PublicScope};
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use validate::email::is_valid_email;

pub mod id;

pub mod get {

  use ts_rs::TS;

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

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/users/GET/")]
  pub struct Output(pub Paged<PublicUser>);

  #[derive(Debug, Default, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/users/GET/")]
  struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
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
        skip: skip.unwrap_or_else(default_skip),
        limit: limit.unwrap_or_else(default_limit),
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope,
        skip,
        limit,
      } = input;

      let page = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => User::paged(None, skip, limit)
          .await?
          .map(|item| item.into_public(PublicScope::Admin)),

        AccessTokenScope::User(user) => Paged::<PublicUser> {
          skip,
          limit,
          total: 1,
          items: vec![user.into_public(PublicScope::User)],
        },
      };

      Ok(Output(page))
    }
  }
}

pub mod post {

  use db::run_transaction;
  use ts_rs::TS;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/users/POST/")]
  #[serde(rename_all = "camelCase")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    email: String,
    password: String,
    account_ids: Vec<String>,
    first_name: String,
    last_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
    #[allow(unused)]
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/users/POST/")]
  pub struct Output {
    user: PublicUser,
  }

  #[derive(Debug)]
  pub enum ParseError {
    Access(GetAccessTokenScopeError),
    Payload(ReadBodyJsonError),
  }

  impl From<GetAccessTokenScopeError> for ParseError {
    fn from(e: GetAccessTokenScopeError) -> Self {
      Self::Access(e)
    }
  }

  impl From<ReadBodyJsonError> for ParseError {
    fn from(e: ReadBodyJsonError) -> Self {
      Self::Payload(e)
    }
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug)]
  pub enum HandleError {
    Db(mongodb::error::Error),
    UserEmailExists,
    AccountNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => ApiError::from(Kind::Db(e)),
        HandleError::UserEmailExists => ApiError::from(Kind::UserEmailExists),
        HandleError::AccountNotFound(id) => ApiError::from(Kind::AccountNotFound(id)),
      }
    }
  }

  impl From<mongodb::error::Error> for HandleError {
    fn from(e: mongodb::error::Error) -> Self {
      Self::Db(e)
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

    async fn parse(&self, mut req: Request) -> Result<Self::Input, Self::ParseError> {
      let access_token_scope = get_access_token_scope(&req).await?;
      if !access_token_scope.has_full_access() {
        return Err(GetAccessTokenScopeError::OutOfScope.into());
      }

      let mut payload: Payload = req.read_body_json(1000 * 40).await?;

      payload.first_name = payload.first_name.trim().to_string();
      payload.last_name = payload.last_name.trim().to_string();
      payload.email = payload.email.trim().to_lowercase();

      if payload.first_name.is_empty() {
        return Err(
          ReadBodyJsonError::PayloadInvalid(String::from("First name is required")).into(),
        );
      }

      if payload.last_name.is_empty() {
        return Err(
          ReadBodyJsonError::PayloadInvalid(String::from("Last name is required")).into(),
        );
      }

      if payload.email.is_empty() {
        return Err(ReadBodyJsonError::PayloadInvalid(String::from("Email is required")).into());
      }

      if !is_valid_email(&payload.email) {
        return Err(ReadBodyJsonError::PayloadInvalid(String::from("Email is invalid")).into());
      }

      if payload.password.len() < 8 {
        return Err(
          ReadBodyJsonError::PayloadInvalid(String::from(
            "Password must have 8 characters or more",
          ))
          .into(),
        );
      }

      Ok(Self::Input {
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let payload = input.payload;

      let Payload {
        email,
        password,
        first_name,
        last_name,
        account_ids,
        user_metadata,
        system_metadata,
      } = payload;

      let user_metadata = user_metadata.unwrap_or_default();
      let system_metadata = system_metadata.unwrap_or_default();

      let password = crypt::hash(&password);

      let user = run_transaction!(session => {

        let email_exists = User::exists_with_session(doc! { "email": &email }, &mut session).await?;
        if email_exists {
          return Err(Self::HandleError::UserEmailExists);
        }

        for id in &account_ids {
          let exists = Account::exists_with_session(id.as_str(), &mut session).await?;
          if !exists {
            return Err(Self::HandleError::AccountNotFound(id.clone()));
          }
        }

        let now = DateTime::now();

        let user = User {
          id: User::uid(),
          email,
          password: Some(password),
          first_name,
          last_name,
          account_ids,
          user_metadata,
          system_metadata,
          created_at: now,
          updated_at: now,
        };

        User::insert_with_session(&user, &mut session).await?;

        user
      });

      Ok(Self::Output {
        user: user.into_public(PublicScope::Admin),
      })
    }
  }
}

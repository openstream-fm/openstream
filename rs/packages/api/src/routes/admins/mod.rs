pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::{Model, Paged};
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

  use db::admin::{Admin, PublicAdmin};
  use ts_rs::TS;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  pub const DEFAULT_SKIP: u64 = 0;
  pub const DEFAULT_LIMIT: i64 = 60;

  pub fn default_skip() -> u64 {
    DEFAULT_SKIP
  }

  pub fn default_limit() -> i64 {
    DEFAULT_LIMIT
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
  #[ts(export, export_to = "../../../defs/api/admins/GET/")]
  struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/admins/GET/")]
  pub struct Output(Paged<PublicAdmin>);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_querystring::de::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::QueryString(e) => e.into(),
      }
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

      if !access_token_scope.has_full_access() {
        return Err(GetAccessTokenScopeError::OutOfScope.into());
      }

      let query = match req.uri().query() {
        None => Default::default(),
        Some(qs) => serde_querystring::from_str(qs, serde_querystring::de::ParseMode::UrlEncoded)?,
      };

      Ok(Self::Input { query })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { query } = input;

      let Query { skip, limit } = query;

      let skip = skip.unwrap_or_else(default_skip);
      let limit = limit.unwrap_or_else(default_limit);
      let sort = doc! { Admin::KEY_CREATED_AT: 1 };

      let page = Admin::paged(None, Some(sort), skip, limit)
        .await?
        .map(Admin::into_public);

      Ok(Output(page))
    }
  }
}

pub mod post {

  use db::admin::{Admin, PublicAdmin};
  use db::metadata::Metadata;
  use db::run_transaction;
  use prex::request::ReadBodyJsonError;
  use serde_util::DateTime;
  use ts_rs::TS;
  use validate::email::is_valid_email;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/admins/POST/")]
  #[serde(rename_all = "snake_case")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/admins/POST/")]
  pub struct Output {
    admin: PublicAdmin,
  }

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
        ParseError::Token(e) => ApiError::from(e),
        ParseError::Payload(e) => ApiError::from(e),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("first name is empty")]
    FirstNameEmpty,
    #[error("last name is empty")]
    LastNameEmpty,
    #[error("email is empty")]
    EmailEmpty,
    #[error("email is invalid")]
    EmailInvalid,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("email already exists")]
    EmailExists,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::EmailEmpty => ApiError::PayloadInvalid(String::from("Email is required")),
        HandleError::FirstNameEmpty => {
          ApiError::PayloadInvalid(String::from("First name is required"))
        }
        HandleError::LastNameEmpty => {
          ApiError::PayloadInvalid(String::from("Last name is required"))
        }
        HandleError::EmailInvalid => ApiError::PayloadInvalid(String::from("Email is invalid")),
        HandleError::PasswordTooShort => {
          ApiError::PayloadInvalid(String::from("Password must have 8 characters or more"))
        }
        HandleError::EmailExists => ApiError::AdminEmailExists,
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

      if !access_token_scope.is_global() {
        return Err(GetAccessTokenScopeError::OutOfScope.into());
      }

      let payload: Payload = req.read_body_json(10_000).await?;
      Ok(Input { payload })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input { payload } = input;

      let Payload {
        first_name,
        last_name,
        email,
        password,
        system_metadata,
      } = payload;

      let email = email.trim().to_lowercase();
      let first_name = first_name.trim().to_string();
      let last_name = last_name.trim().to_string();
      let system_metadata = system_metadata.unwrap_or_default();

      if email.is_empty() {
        return Err(HandleError::EmailEmpty);
      }

      if !is_valid_email(&email) {
        return Err(HandleError::EmailInvalid);
      }

      if first_name.is_empty() {
        return Err(HandleError::FirstNameEmpty);
      }

      if last_name.is_empty() {
        return Err(HandleError::LastNameEmpty);
      }

      if password.len() < 8 {
        return Err(HandleError::PasswordTooShort);
      }

      let password = crypt::hash(password);

      let now = DateTime::now();

      let admin = Admin {
        id: Admin::uid(),
        first_name,
        last_name,
        email,
        password,
        system_metadata,
        created_at: now,
        updated_at: now,
        deleted_at: None,
      };

      run_transaction!(session => {
        let admin_exists = tx_try!(Admin::exists_with_session(doc!{ Admin::KEY_EMAIL: &admin.email }, &mut session).await);
        if admin_exists {
          return Err(HandleError::EmailExists);
        }

        tx_try!(Admin::insert_with_session(&admin, &mut session).await);
      });

      let out = Output {
        admin: admin.into_public(),
      };

      Ok(out)
    }
  }
}

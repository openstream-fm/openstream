use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::user::{PublicUser, User};
use db::Model;
use db::PublicScope;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

  use super::*;
  use std::convert::Infallible;
  use ts_rs::TS;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    user: User,
    public_scope: PublicScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/users/[user]/GET/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    user: PublicUser,
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = Infallible;

    async fn parse(&self, req: Request) -> Result<Input, GetAccessTokenScopeError> {
      let user_id = req.param("user").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let user = access_token_scope.grant_user_scope(user_id).await?;
      let public_scope = access_token_scope.as_public_scope();
      Ok(Self::Input { user, public_scope })
    }

    async fn perform(&self, input: Input) -> Result<Output, Infallible> {
      let Self::Input { user, public_scope } = input;
      Ok(Output {
        user: user.into_public(public_scope),
      })
    }
  }
}

pub mod patch {

  use crate::error::ApiError;

  use super::*;
  use db::{fetch_and_patch, run_transaction};
  use prex::request::ReadBodyJsonError;
  use serde_util::DateTime;
  use ts_rs::TS;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/users/[user]/PATCH/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Payload {
    first_name: Option<String>,
    last_name: Option<String>,
    #[serde(
      default,
      skip_serializing_if = "Option::is_none",
      deserialize_with = "serde_util::map_some"
    )]
    phone: Option<Option<String>>,
    password: Option<String>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    user: User,
    payload: Payload,
    public_scope: PublicScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/users/[user]/PATCH/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    user: PublicUser,
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
        ParseError::Token(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("invalid payload: {0}")]
    Payload(String),
    #[error("User not found: {0}")]
    UserNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::Payload(message) => ApiError::PayloadInvalid(message),
        HandleError::UserNotFound(id) => ApiError::UserNotFound(id),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let user_id = req.param("user").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let user = access_token_scope.grant_user_scope(user_id).await?;
      let public_scope = access_token_scope.as_public_scope();
      let payload: Payload = req.read_body_json(1000 * 5).await?;
      Ok(Self::Input {
        user,
        public_scope,
        payload,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Self::Input {
        user,
        public_scope,
        payload,
      } = input;

      let Payload {
        first_name,
        last_name,
        phone,
        password,
      } = payload;

      if let Some(first_name) = &first_name {
        if first_name.len() > 50 {
          return Err(HandleError::Payload(
            "First name must be of 50 characters or less".into(),
          ));
        }
      };

      if let Some(last_name) = &last_name {
        if last_name.len() > 50 {
          return Err(HandleError::Payload(
            "Last name must be of 50 characters or less".into(),
          ));
        }
      };

      if let Some(Some(phone)) = &phone {
        if phone.len() > 30 {
          return Err(HandleError::Payload(
            "Phone must be of 30 characters or less".into(),
          ));
        }
      };

      if let Some(password) = &password {
        if password.len() > 100 {
          return Err(HandleError::Payload(
            "Password must be of 50 characters or less".into(),
          ));
        }

        if password.len() < 8 {
          return Err(HandleError::Payload(
            "Password must be of 8 characters or more".into(),
          ));
        }
      }

      let password_hash = password.map(crypt::hash);

      let user = run_transaction!(session => {
        fetch_and_patch!(User, up_user, &user.id, Err(HandleError::UserNotFound(user.id)), session, {
          if let Some(first_name) = &first_name {
            up_user.first_name = first_name.clone();
          }

          if let Some(last_name) = &last_name {
            up_user.last_name = last_name.clone();
          }

          if let Some(opt_phone) = &phone {
            up_user.phone = opt_phone.clone();
          }

          if let Some(password_hash) = &password_hash {
            up_user.password = Some(password_hash.clone());
          }

          up_user.updated_at = DateTime::now();

        })
      });

      Ok(Output {
        user: user.into_public(public_scope),
      })
    }
  }
}

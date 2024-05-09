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
  use schemars::JsonSchema;
  use std::convert::Infallible;
  use ts_rs::TS;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    user: User,
    public_scope: PublicScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/users/[user]/GET/")]
  #[macros::schema_ts_export]
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
  use constants::validate::*;
  use db::{fetch_and_patch, run_transaction};
  use modify::Modify;
  use prex::request::ReadBodyJsonError;
  use schemars::JsonSchema;
  use serde_util::DateTime;
  use ts_rs::TS;
  use validator::Validate;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(export, export_to = "../../../defs/api/users/[user]/PATCH/")]
  #[macros::schema_ts_export]
  pub struct Payload {
    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_USER_FIRST_NAME_MAX_LEN",
        message = "First name is either too long or too short"
      ),
      non_control_character(message = "First name contains invalid characters")
    )]
    first_name: Option<String>,

    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_USER_LAST_NAME_MAX_LEN",
        message = "Last name is either too long or too short"
      ),
      non_control_character(message = "Last name contains invalid characters")
    )]
    last_name: Option<String>,

    #[serde(
      default,
      skip_serializing_if = "Option::is_none",
      deserialize_with = "serde_util::map_some"
    )]
    #[modify(trim)]
    #[validate(
      phone(message = "Phone is invalid"),
      length(
        min = 1,
        max = "VALIDATE_USER_PHONE_MAX_LEN",
        message = "Phone is either too long or too short"
      )
    )]
    phone: Option<Option<String>>,

    #[serde(
      default,
      skip_serializing_if = "Option::is_none",
      deserialize_with = "serde_util::map_some"
    )]
    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_USER_LANGUAGE_MAX_LEN",
        message = "Language is either too long or too short"
      ),
      non_control_character(message = "Language contains invalid characters")
    )]
    language: Option<Option<String>>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    user: User,
    payload: Payload,
    public_scope: PublicScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/users/[user]/PATCH/")]
  #[macros::schema_ts_export]
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
        language,
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

      if let Some(Some(language)) = &language {
        if language.len() > 10 {
          return Err(HandleError::Payload(
            "Language must be of 10 characters or less".into(),
          ));
        }
      };

      let user = run_transaction!(session => {
        fetch_and_patch!(User, up_user, &user.id, Err(HandleError::UserNotFound(user.id)), session, {
          if let Some(first_name) = &first_name {
            up_user.first_name.clone_from(first_name);
          }

          if let Some(last_name) = &last_name {
            up_user.last_name.clone_from(last_name);
          }

          if let Some(opt_phone) = &phone {
            up_user.phone = match opt_phone.as_ref().map(|v| v.trim())  {
              None => None,
              Some("") => None,
              Some(v) => Some(v.to_string()),
            };
          }

          if let Some(opt_language) = &language {
            up_user.language = match opt_language.as_ref().map(|v| v.trim()) {
              None => None,
              Some("") => None,
              Some(v) => Some(v.to_string()),
            }
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

pub mod delete {

  use crate::error::ApiError;

  use super::*;
  use db::{run_transaction, user::AdminPublicUser, user_account_relation::UserAccountRelation};
  use schemars::JsonSchema;
  use ts_rs::TS;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    user_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/users/[user]/DELETE/")]
  #[macros::schema_ts_export]
  pub struct Output {
    user: AdminPublicUser,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("user not found: {0}")]
    UserNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::UserNotFound(id) => ApiError::UserNotFound(id),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Input, GetAccessTokenScopeError> {
      let user_id = req.param("user").unwrap().to_string();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      if !access_token_scope.is_admin_or_global() {
        return Err(GetAccessTokenScopeError::OutOfScope);
      }
      Ok(Self::Input { user_id })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Self::Input { user_id } = input;
      let user = run_transaction!(session => {
        let now = serde_util::DateTime::now();
        let mut user = match tx_try!(User::get_by_id_with_session(&user_id, &mut session).await) {
          Some(user) if user.deleted_at.is_none() => user,
          _ => return Err(HandleError::UserNotFound(user_id))
        };

        user.deleted_at = Some(now);
        user.updated_at = now;

        let rel_filter = doc! { UserAccountRelation::KEY_USER_ID: &user_id };
        tx_try!(UserAccountRelation::cl().delete_many_with_session(rel_filter, None, &mut session).await);
        tx_try!(User::replace_with_session(&user.id, &user, &mut session).await);

        user
      });

      Ok(Output { user: user.into() })
    }
  }
}

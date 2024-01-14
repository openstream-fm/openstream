pub mod post {

  use mongodb::bson::doc;
  use schemars::JsonSchema;
  use std::net::IpAddr;

  use crate::{
    error::ApiError,
    ip_limit,
    json::JsonHandler,
    request_ext::{self, GetAccessTokenScopeError},
  };

  use db::{user::User, Model};

  use prex::{request::ReadBodyJsonError, Request};
  use serde::{Deserialize, Serialize};
  use serde_util::empty_struct::EmptyStruct;
  use ts_rs::TS;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/users/[user]/change-password/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Payload {
    pub current_password: String,
    pub new_password: String,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    user: User,
    payload: Payload,
    ip: IpAddr,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/users/[user]/change-password/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Output(EmptyStruct);

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
    #[error("current password mismatch")]
    CurrentPasswordMismatch,
    #[error("new password too short")]
    NewPasswordTooShort,
    #[error("ip limit reached")]
    IpLimit,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => Self::from(e),
        HandleError::CurrentPasswordMismatch => {
          Self::PayloadInvalid(String::from("Current password does not match"))
        }
        HandleError::NewPasswordTooShort => {
          Self::PayloadInvalid(String::from("New password must be of 8 characters or more"))
        }
        HandleError::IpLimit => ApiError::TooManyRequests,
      }
    }
  }

  #[async_trait::async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Self::Input, Self::ParseError> {
      let user_id = req.param("user").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let user = access_token_scope.grant_user_scope(user_id).await?;
      let ip = req.isomorphic_ip();
      let payload: Payload = req.read_body_json(100_000).await?;
      Ok(Self::Input { user, payload, ip })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        user,
        ip,
        payload: Payload {
          current_password,
          new_password,
        },
      } = input;

      if ip_limit::should_reject(ip) {
        return Err(HandleError::IpLimit);
      }

      ip_limit::hit(ip);

      match &user.password {
        None => return Err(HandleError::CurrentPasswordMismatch),
        Some(user_password) => {
          if !crypt::compare(&current_password, user_password) {
            return Err(HandleError::CurrentPasswordMismatch);
          }
        }
      }

      if new_password.len() < 8 {
        return Err(HandleError::NewPasswordTooShort);
      }

      let hashed = crypt::hash(&new_password);

      let update = doc! {
        "$set": { User::KEY_PASSWORD: hashed }
      };

      User::update_by_id(&user.id, update).await?;

      Ok(Output(EmptyStruct(())))
    }
  }
}

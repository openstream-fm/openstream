pub mod post {

  use crate::{
    error::ApiError,
    ip_limit,
    json::JsonHandler,
    request_ext::{self, GetAccessTokenScopeError},
  };
  use db::{admin::Admin, Model};
  use mongodb::bson::doc;
  use prex::{request::ReadBodyJsonError, Request};
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};
  use serde_util::empty_struct::EmptyStruct;
  use std::net::IpAddr;
  use ts_rs::TS;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/admins/[admin]/change-password/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Payload {
    pub current_password: String,
    pub new_password: String,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    admin: Admin,
    payload: Payload,
    ip: IpAddr,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/admins/[admin]/change-password/POST/"
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
      let admin_id = req.param("admin").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let admin = access_token_scope.grant_admin_write_scope(admin_id).await?;
      let ip = req.isomorphic_ip();
      let payload: Payload = req.read_body_json(100_000).await?;
      Ok(Self::Input { admin, payload, ip })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        admin,
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

      if !crypt::compare(&current_password, &admin.password) {
        return Err(HandleError::CurrentPasswordMismatch);
      }

      if new_password.len() < 8 {
        return Err(HandleError::NewPasswordTooShort);
      }

      let hashed = crypt::hash(&new_password);

      let update = doc! {
        "$set": { Admin::KEY_PASSWORD: hashed }
      };

      Admin::update_by_id(&admin.id, update).await?;

      Ok(Output(EmptyStruct(())))
    }
  }
}

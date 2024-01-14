use crate::json::JsonHandler;

use crate::error::ApiError;
use async_trait::async_trait;
use db::Model;
use db::{token_user_recovery::TokenUserRecovery, user::User};
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod set_password;

pub mod get {

  use std::net::IpAddr;

  use schemars::JsonSchema;

  use crate::ip_limit;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    ip: IpAddr,
    token: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/auth/user/recovery-token/[token]/GET/"
  )]
  #[macros::schema_ts_export]
  #[serde(tag = "kind", rename_all = "snake_case")]
  pub enum Output {
    Found {
      user_id: String,
      user_first_name: String,
      user_last_name: String,
      user_email: String,
      already_used: bool,
      expired: bool,
    },
    NotFound,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {}

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {}
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("token user not found")]
    TokenUserNotFound(String),
    #[error("ip limit")]
    IpLimit,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::TokenUserNotFound(id) => ApiError::TokenUserNotFound(id),
        HandleError::IpLimit => ApiError::TooManyRequests,
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Input, ParseError> {
      let token = req.param("token").unwrap().to_string();
      let ip = req.isomorphic_ip();
      Ok(Input { token, ip })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Input { token, ip } = input;

      if ip_limit::should_reject(ip) {
        return Err(HandleError::IpLimit);
      }

      ip_limit::hit(ip);

      let (id, key) = match token.split_once('-') {
        Some(pair) => pair,
        None => return Ok(Output::NotFound),
      };

      let hash = crypt::sha256(key);
      let filter = doc! { TokenUserRecovery::KEY_ID: id, TokenUserRecovery::KEY_HASH: hash };

      let token = match TokenUserRecovery::get(filter).await? {
        Some(token) => token,
        None => return Ok(Output::NotFound),
      };

      let already_used = token.used_at.is_some();
      let expired = token.is_expired();

      let user = match User::get_by_id(&token.user_id).await? {
        Some(user) => user,
        None => return Err(HandleError::TokenUserNotFound(token.user_id)),
      };

      Ok(Output::Found {
        user_id: user.id,
        user_first_name: user.first_name,
        user_last_name: user.last_name,
        user_email: user.email,
        already_used,
        expired,
      })
    }
  }
}

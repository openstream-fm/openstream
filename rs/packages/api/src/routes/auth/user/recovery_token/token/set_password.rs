use crate::error::ApiError;
use db::run_transaction;
use db::user::User;
use db::Model;
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

pub mod post {

  use std::net::IpAddr;

  use crate::{ip_limit, json::JsonHandler};
  use async_trait::async_trait;
  use constants::validate::*;
  use db::token_user_recovery::TokenUserRecovery;
  use modify::Modify;
  use prex::Request;
  use schemars::JsonSchema;
  use validator::Validate;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(
    export,
    export_to = "../../../defs/api/auth/user/recovery-token/[token]/set-password/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Payload {
    #[validate(length(
      min = "VALIDATE_USER_PASSWORD_MIN_LEN",
      max = "VALIDATE_USER_PASSWORD_MAX_LEN",
      message = "New password is either too short or too long",
    ))]
    new_password: String,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    token: String,
    payload: Payload,
    ip: IpAddr,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/auth/user/recovery-token/[token]/set-password/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Output {
    user_id: String,
    user_email: String,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("payload read: {0}")]
    PayloadRead(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::PayloadRead(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("token user not found")]
    TokenUserNotFound(String),
    #[error("not found")]
    NotFound,
    #[error("expired")]
    Expired,
    #[error("already used")]
    AlreadyUsed,
    #[error("password too short")]
    PasswordTooShort,
    #[error("ip limit")]
    IpLimit,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
          HandleError::Db(e) => e.into(),
          HandleError::TokenUserNotFound(id) => ApiError::TokenUserNotFound(id),
          HandleError::NotFound => ApiError::BadRequestCustom("Couldn't find this link, try creating a new recovery link from the user recovery page".into()),
          HandleError::Expired => ApiError::BadRequestCustom("This recovery link has expired, try creating a new recovery link from the user recovery page".into()),
          HandleError::AlreadyUsed => ApiError::BadRequestCustom("This recovery link has already been used, try creating a new recovery link from the user recovery page".into()),
          HandleError::PasswordTooShort => ApiError::BadRequestCustom("Password must be of 8 characters or more".into()),
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

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let token = req.param("token").unwrap().to_string();
      let payload: Payload = req.read_body_json(1000).await?;
      let ip = req.isomorphic_ip();
      Ok(Input { token, ip, payload })
    }

    async fn perform(&self, input: Self::Input) -> Result<Output, HandleError> {
      let Input { token, payload, ip } = input;
      let Payload { new_password } = payload;

      if ip_limit::should_reject(ip) {
        return Err(HandleError::IpLimit);
      }

      ip_limit::hit(ip);

      let (id, key) = match token.split_once('-') {
        Some(pair) => pair,
        None => return Err(HandleError::NotFound),
      };

      let hash = crypt::sha256(key);
      let filter = doc! { TokenUserRecovery::KEY_ID: id, TokenUserRecovery::KEY_HASH: hash };

      let token = match TokenUserRecovery::get(filter).await? {
        Some(token) => token,
        None => return Err(HandleError::NotFound),
      };

      if token.used_at.is_some() {
        return Err(HandleError::AlreadyUsed);
      }

      if token.is_expired() {
        return Err(HandleError::Expired);
      }

      if new_password.len() < 8 {
        return Err(HandleError::PasswordTooShort);
      }

      let password_hash = crypt::hash(&new_password);

      let user = run_transaction!(session => {

        let filter = doc! { TokenUserRecovery::KEY_ID: &token.user_id };

        let user_update = doc! {
          "$set": {
            User::KEY_UPDATED_AT: DateTime::now(),
            User::KEY_PASSWORD: &password_hash,
          }
        };

        let user = match tx_try!(User::cl().find_one_and_update_with_session(filter, user_update, None, &mut session).await) {
          Some(user) => user,
          None => return Err(HandleError::TokenUserNotFound(token.user_id))
        };

        let token_update = doc! {
          "$set": {
            TokenUserRecovery::KEY_USED_AT: DateTime::now(),
          }
        };

        tx_try!(TokenUserRecovery::update_by_id_with_session(&token.id, token_update, &mut session).await);

        user
      });

      Ok(Output {
        user_id: user.id,
        user_email: user.email,
      })
    }
  }
}

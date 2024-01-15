pub mod post {
  use async_trait::async_trait;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::user::{User, UserPublicUser};
  use db::{current_filter_doc, run_transaction, Model};
  use modify::Modify;
  use mongodb::bson::doc;
  use prex::{request::ReadBodyJsonError, Request};
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};
  use serde_util::DateTime;
  use std::net::IpAddr;
  use ts_rs::TS;
  use user_agent::{UserAgent, UserAgentExt};
  use validator::Validate;

  use crate::error::ApiError;
  use crate::ip_limit::{hit, should_reject};
  use crate::json::JsonHandler;

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(export, export_to = "../../../defs/api/auth/user/login/POST/")]
  #[macros::schema_ts_export]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    email: String,
    password: String,
    device_id: String,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
    ip: IpAddr,
    user_agent: UserAgent,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/auth/user/login/POST/")]
  #[macros::schema_ts_export]
  pub struct Output {
    user: UserPublicUser,
    token: String,
    media_key: String,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("too many requests")]
    TooManyRequests,
    #[error("email missing")]
    EmailMissing,
    #[error("password missing")]
    PasswordMissing,
    #[error("device id invalid")]
    DeviceIdInvalid,
    #[error("no match email")]
    NoMatchEmail,
    #[error("no password")]
    NoPassword,
    #[error("no match password")]
    NoMatchPassword,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::TooManyRequests => ApiError::TooManyRequests,
        HandleError::NoMatchEmail => ApiError::UserAuthFailed,
        HandleError::NoPassword => ApiError::UserAuthFailed,
        HandleError::NoMatchPassword => ApiError::UserAuthFailed,
        HandleError::EmailMissing => ApiError::PayloadInvalid("email is required".into()),
        HandleError::PasswordMissing => ApiError::PayloadInvalid("password is required".into()),
        HandleError::DeviceIdInvalid => ApiError::PayloadInvalid("device_id is invalid".into()),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type ParseError = ReadBodyJsonError;
    type HandleError = HandleError;
    type Input = Input;
    type Output = Output;

    async fn parse(&self, mut req: Request) -> Result<Input, Self::ParseError> {
      let payload: Payload = req.read_body_json(1000 * 5).await?;

      let ip = req.isomorphic_ip();

      let user_agent = req.parse_ua();

      Ok(Input {
        ip,
        payload,
        user_agent,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Input {
        ip,
        payload,
        user_agent,
      } = input;

      let Payload {
        email,
        password,
        device_id,
      } = payload;

      let email = email.trim().to_lowercase();

      if email.is_empty() {
        return Err(HandleError::EmailMissing);
      };

      if password.is_empty() {
        return Err(HandleError::PasswordMissing);
      };

      if !AccessToken::is_device_id_valid(&device_id) {
        return Err(HandleError::DeviceIdInvalid);
      };

      if should_reject(ip) {
        return Err(HandleError::TooManyRequests);
      }

      hit(ip);

      let user = match User::find_by_email(&email, Some(true)).await? {
        None => return Err(HandleError::NoMatchEmail),
        Some(user) => user,
      };

      let user_password = match user.password {
        None => return Err(HandleError::NoPassword),
        Some(ref v) => v.as_str(),
      };

      if !crypt::compare(&password, user_password) {
        return Err(HandleError::NoMatchPassword);
      }

      let user_id = user.id.clone();

      let key = AccessToken::random_key();

      let media_key = AccessToken::random_media_key();

      let delete_filter = current_filter_doc! {
        GeneratedBy::KEY_ENUM_TAG: { "$in": [ GeneratedBy::KEY_ENUM_VARIANT_LOGIN, GeneratedBy::KEY_ENUM_VARIANT_REGISTER ] },
        GeneratedBy::KEY_DEVICE_ID: &device_id,
        Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_USER,
        Scope::KEY_USER_ID: &user.id,
      };

      let token = AccessToken {
        id: AccessToken::uid(),
        hash: crypt::sha256(&key),
        media_hash: crypt::sha256(&media_key),
        scope: Scope::User { user_id },
        generated_by: GeneratedBy::Login {
          ip,
          user_agent,
          device_id,
        },
        created_at: DateTime::now(),
        last_used_at: None,
        hits: 0,
        deleted_at: None,
      };

      run_transaction!(session => {
        tx_try!(AccessToken::set_deleted_with_session(delete_filter.clone(), &mut session).await);
        tx_try!(AccessToken::insert_with_session(&token, &mut session).await);
      });

      let user = UserPublicUser::from(user);

      let out = Output {
        user,
        token: format!("{}-{}", token.id, key),
        media_key: format!("{}-{}", token.id, media_key),
      };

      Ok(out)
    }
  }
}

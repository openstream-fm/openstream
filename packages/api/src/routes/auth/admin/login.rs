pub mod post {
  use async_trait::async_trait;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::admin::{Admin, PublicAdmin};
  use db::Model;
  use mongodb::bson::doc;
  use prex::{request::ReadBodyJsonError, Request};
  use serde::{Deserialize, Serialize};
  use serde_util::DateTime;
  use std::net::IpAddr;
  use ts_rs::TS;
  use user_agent::{UserAgent, UserAgentExt};

  use crate::error::ApiError;
  use crate::ip_limit::{hit, should_reject};
  use crate::json::JsonHandler;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/auth/admin/login/POST/")]
  #[serde(rename_all = "snake_case")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    email: String,
    password: String,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
    ip: IpAddr,
    user_agent: UserAgent,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/auth/admin/login/POST/")]
  #[serde(rename_all = "snake_case")]
  pub struct Output {
    admin: PublicAdmin,
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
    #[error("no match email")]
    NoMatchEmail,
    #[error("no match password")]
    NoMatchPassword,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::TooManyRequests => ApiError::TooManyRequests,
        HandleError::NoMatchEmail => ApiError::AuthFailed,
        HandleError::NoMatchPassword => ApiError::AuthFailed,
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
      let mut payload: Payload = req.read_body_json(1000 * 5).await?;

      payload.email = payload.email.trim().to_string();

      if payload.email.is_empty() {
        return Err(ReadBodyJsonError::PayloadInvalid(String::from(
          "Email is required",
        )));
      }

      if payload.password.is_empty() {
        return Err(ReadBodyJsonError::PayloadInvalid(String::from(
          "Password is required",
        )));
      };

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

      if should_reject(ip) {
        return Err(HandleError::TooManyRequests);
      }

      hit(ip);

      let Payload { email, password } = payload;

      let email = email.trim().to_lowercase();

      let admin = match Admin::get(doc! { Admin::KEY_EMAIL: email }).await? {
        None => return Err(HandleError::NoMatchEmail),
        Some(admin) => admin,
      };

      let is_match = crypt::compare(&password, &admin.password);

      if !is_match {
        return Err(HandleError::NoMatchPassword);
      }

      let admin_id = admin.id.clone();

      let key = AccessToken::random_key();

      let media_key = AccessToken::random_media_key();

      let token = AccessToken {
        id: AccessToken::uid(),
        key,
        media_key,
        scope: Scope::Admin { admin_id },
        generated_by: GeneratedBy::Login { ip, user_agent },
        last_used_at: None,
        hits: 0,
        created_at: DateTime::now(),
        deleted_at: None,
      };

      AccessToken::insert(&token).await?;

      let admin = admin.into_public();

      let out = Output {
        admin,
        token: token.key,
        media_key: token.media_key,
      };

      Ok(out)
    }
  }
}

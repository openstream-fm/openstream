pub mod post {
  use async_trait::async_trait;
  use chrono::Utc;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::user::User;
  use db::Model;
  use mongodb::bson::doc;
  use prex::{request::ReadBodyJsonError, Request};
  use serde::{Deserialize, Serialize};
  use std::net::IpAddr;
  use user_agent::{UserAgent, UserAgentExt};

  use crate::error::{ApiError, Kind};
  use crate::ip_limit::{hit, should_reject};
  use crate::json::JsonHandler;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(rename = "camelCase")]
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

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct Output {
    token: String,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug)]
  pub enum HandleError {
    TooManyRequests,
    NoMatchEmail,
    NoPassword,
    NoMatchPassword,
    Db(mongodb::error::Error),
  }

  impl From<mongodb::error::Error> for HandleError {
    fn from(e: mongodb::error::Error) -> Self {
      Self::Db(e)
    }
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::TooManyRequests => ApiError::from(Kind::TooManyRequests),
        HandleError::NoMatchEmail => ApiError::from(Kind::AuthFailed),
        HandleError::NoPassword => ApiError::from(Kind::AuthFailed),
        HandleError::NoMatchPassword => ApiError::from(Kind::AuthFailed),
        HandleError::Db(e) => ApiError::from(Kind::Db(e)),
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

    async fn perform(
      &self,
      Input {
        ip,
        payload,
        user_agent,
      }: Input,
    ) -> Result<Output, Self::HandleError> {
      if should_reject(ip) {
        return Err(HandleError::TooManyRequests);
      }

      hit(ip);

      let Payload { email, password } = payload;

      let user = match User::cl().find_one(doc! { "email": email }, None).await? {
        None => return Err(HandleError::NoMatchEmail),
        Some(user) => user,
      };

      let user_password = match user.password {
        None => return Err(HandleError::NoPassword),
        Some(ref v) => v.as_str(),
      };

      let is_match = crypt::compare(&password, user_password);

      #[allow(clippy::bool_comparison)]
      if is_match == false {
        return Err(HandleError::NoMatchPassword);
      }

      let token = AccessToken {
        id: AccessToken::uid(),
        scope: Scope::User { user_id: user.id },
        generated_by: GeneratedBy::Login { ip, user_agent },
        created_at: Utc::now(),
        last_used_at: None,
        hits: 0,
      };

      AccessToken::insert(&token).await?;

      let out = Output { token: token.id };

      Ok(out)
    }
  }
}
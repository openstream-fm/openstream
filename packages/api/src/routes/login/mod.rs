pub mod post {
  use async_trait::async_trait;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::user::{User, UserPublicUser};
  use db::Model;
  use mongodb::bson::doc;
  use prex::{request::ReadBodyJsonError, Request};
  use serde::{Deserialize, Serialize};
  use serde_util::DateTime;
  use std::net::IpAddr;
  use ts_rs::TS;
  use user_agent::{UserAgent, UserAgentExt};

  use crate::error::{ApiError, Kind};
  use crate::ip_limit::{hit, should_reject};
  use crate::json::JsonHandler;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/login/POST/")]
  #[serde(rename_all = "camelCase")]
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
  #[ts(export)]
  #[ts(export_to = "../../defs/api/login/POST/")]
  #[serde(rename_all = "camelCase")]
  pub struct Output {
    user: UserPublicUser,
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

      let user = match User::find_by_email(&email).await? {
        None => return Err(HandleError::NoMatchEmail),
        Some(user) => user,
      };

      let user_password = match user.password {
        None => return Err(HandleError::NoPassword),
        Some(ref v) => v.as_str(),
      };

      let is_match = crypt::compare(&password, user_password);

      if !is_match {
        return Err(HandleError::NoMatchPassword);
      }

      let user_id = user.id.clone();

      let token = AccessToken {
        id: AccessToken::uid(),
        key: AccessToken::random_key(),
        scope: Scope::User { user_id },
        generated_by: GeneratedBy::Login { ip, user_agent },
        created_at: DateTime::now(),
        last_used_at: None,
        hits: 0,
      };

      AccessToken::insert(&token).await?;

      let user = UserPublicUser::from(user);

      let out = Output {
        user,
        token: token.key,
      };

      Ok(out)
    }
  }
}

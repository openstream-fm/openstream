pub mod post {
  use std::net::IpAddr;

  use async_trait::async_trait;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::config::Config;
  use db::metadata::Metadata;
  use db::models::user_station_relation::{UserStationRelation, UserStationRelationKind};
  use db::station::{Limit, Limits, PublicStation, Station};
  use db::user::{PublicUser, User};
  use db::{run_transaction, Model, Singleton};
  use prex::{request::ReadBodyJsonError, Request};
  use serde::{Deserialize, Serialize};
  use serde_util::DateTime;
  use ts_rs::TS;
  use user_agent::{UserAgent, UserAgentExt};
  use validate::email::is_valid_email;

  use crate::error::ApiError;
  use crate::json::JsonHandler;
  use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

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
    #[error("mongodb: {0}")]
    Db(mongodb::error::Error),
    #[error("token out of scope")]
    TokenOutOfScope,
    #[error("station name is empty")]
    StationNameEmpty,
    #[error("first name is empty")]
    FirstNameEmpty,
    #[error("last name is empty")]
    LastNameEmpty,
    #[error("email is empty")]
    EmailEmpty,
    #[error("email is invalid")]
    EmailInvalid,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("email already exists")]
    EmailExists,
  }

  impl From<mongodb::error::Error> for HandleError {
    fn from(e: mongodb::error::Error) -> Self {
      Self::Db(e)
    }
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::TokenOutOfScope => ApiError::TokenOutOfScope,
        HandleError::StationNameEmpty => {
          ApiError::PayloadInvalid(String::from("Station name is required"))
        }
        HandleError::EmailEmpty => ApiError::PayloadInvalid(String::from("Email is required")),
        HandleError::FirstNameEmpty => {
          ApiError::PayloadInvalid(String::from("First name is required"))
        }
        HandleError::LastNameEmpty => {
          ApiError::PayloadInvalid(String::from("Last name is required"))
        }
        HandleError::EmailInvalid => ApiError::PayloadInvalid(String::from("Email is invalid")),
        HandleError::PasswordTooShort => {
          ApiError::PayloadInvalid(String::from("Password must have 8 characters or more"))
        }
        HandleError::EmailExists => ApiError::UserEmailExists,
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/auth/user/register/POST/")]
  // #[serde(rename_all = "camelCase")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    station_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<PayloadLimits>,

    #[serde(skip_serializing_if = "Option::is_none")]
    station_user_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    station_system_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_user_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/auth/user/register/POST/")]
  // #[serde(rename_all = "camelCase")]
  #[serde(deny_unknown_fields)]
  pub struct PayloadLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    listeners: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transfer: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<u64>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    pub ip: IpAddr,
    pub user_agent: UserAgent,
    pub access_token_scope: AccessTokenScope,
    pub payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/auth/user/register/POST/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    pub user: PublicUser,
    pub station: PublicStation,
    pub token: String,
    pub media_key: String,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let ip = req.isomorphic_ip();
      let user_agent = req.parse_ua();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let payload: Payload = req.read_body_json(10_000).await?;
      Ok(Input {
        ip,
        user_agent,
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        ip,
        user_agent,
        access_token_scope,
        payload,
      } = input;

      if !access_token_scope.has_full_access() {
        return Err(HandleError::TokenOutOfScope);
      }

      let Payload {
        email,
        password,
        first_name,
        last_name,
        station_name,
        station_user_metadata,
        station_system_metadata,
        user_user_metadata,
        user_system_metadata,
        limits: payload_limits,
      } = payload;

      let email = email.trim().to_lowercase();
      let first_name = first_name.trim().to_string();
      let last_name = last_name.trim().to_string();
      let station_name = station_name.trim().to_string();

      let payload_limits = payload_limits.unwrap_or_default();
      let station_user_metadata = station_user_metadata.unwrap_or_default();
      let station_system_metadata = station_system_metadata.unwrap_or_default();
      let user_user_metadata = user_user_metadata.unwrap_or_default();
      let user_system_metadata = user_system_metadata.unwrap_or_default();

      if email.is_empty() {
        return Err(HandleError::EmailEmpty);
      }

      if !is_valid_email(&email) {
        return Err(HandleError::EmailInvalid);
      }

      if first_name.is_empty() {
        return Err(HandleError::FirstNameEmpty);
      }

      if last_name.is_empty() {
        return Err(HandleError::LastNameEmpty);
      }

      if station_name.is_empty() {
        return Err(HandleError::StationNameEmpty);
      }

      if password.len() < 8 {
        return Err(HandleError::PasswordTooShort);
      }

      let config = <Config as Singleton>::get().await?;

      let limits = match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => Limits {
          listeners: Limit {
            used: 0,
            total: payload_limits.listeners.unwrap_or(config.limits.listeners),
          },
          transfer: Limit {
            used: 0,
            total: payload_limits.transfer.unwrap_or(config.limits.transfer),
          },
          storage: Limit {
            used: 0,
            total: payload_limits.storage.unwrap_or(config.limits.storage),
          },
        },
        AccessTokenScope::User(_) => Limits {
          listeners: Limit {
            used: 0,
            total: config.limits.listeners,
          },
          transfer: Limit {
            used: 0,
            total: config.limits.transfer,
          },
          storage: Limit {
            used: 0,
            total: config.limits.storage,
          },
        },
      };

      let password = crypt::hash(password);

      let now = DateTime::now();

      let user = User {
        id: User::uid(),
        email,
        first_name,
        last_name,
        password: Some(password),
        user_metadata: user_user_metadata,
        system_metadata: user_system_metadata,
        created_at: now,
        updated_at: now,
      };

      let station = Station {
        id: Station::uid(),
        name: station_name,
        limits,
        source_password: Station::random_source_password(),
        user_metadata: station_user_metadata,
        system_metadata: station_system_metadata,
        playlist_is_randomly_shuffled: false,
        created_at: now,
        updated_at: now,
      };

      let relation = UserStationRelation {
        id: UserStationRelation::uid(),
        user_id: user.id.clone(),
        station_id: station.id.clone(),
        kind: UserStationRelationKind::Owner,
        created_at: now,
      };

      let key = AccessToken::random_key();
      let media_key = AccessToken::random_media_key();

      let token = AccessToken {
        id: AccessToken::uid(),
        key,
        media_key,
        scope: Scope::User {
          user_id: user.id.clone(),
        },
        generated_by: GeneratedBy::Register { ip, user_agent },
        last_used_at: None,
        hits: 0,
        created_at: now,
        deleted_at: None,
      };

      run_transaction!(session => {
        let email_exists = tx_try!(User::email_exists_with_session(user.email.as_str(), &mut session).await);
        if email_exists {
          return Err(HandleError::EmailExists)
        }

        tx_try!(User::insert_with_session(&user, &mut session).await);
        tx_try!(Station::insert_with_session(&station, &mut session).await);
        tx_try!(UserStationRelation::insert_with_session(&relation, &mut session).await);
        tx_try!(AccessToken::insert_with_session(&token, &mut session).await);
      });

      let out = Output {
        user: user.into_public(access_token_scope.as_public_scope()),
        station: station.into_public(access_token_scope.as_public_scope()),
        token: token.key,
        media_key: token.media_key,
      };

      Ok(out)
    }
  }
}

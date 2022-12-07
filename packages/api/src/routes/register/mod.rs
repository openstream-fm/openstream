pub mod post {
  use std::net::IpAddr;

  use async_trait::async_trait;
  use chrono::Utc;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::account::{Account, Limit, Limits, PublicAccount};
  use db::config::Config;
  use db::metadata::Metadata;
  use db::user::{PublicUser, User};
  use db::{run_transaction, Model, Singleton};
  use prex::{request::ReadBodyJsonError, Request};
  use serde::{Deserialize, Serialize};
  use ts_rs::TS;
  use user_agent::{UserAgent, UserAgentExt};
  use validate::email::is_valid_email;

  use crate::error::{ApiError, Kind};
  use crate::json::JsonHandler;
  use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

  #[derive(Debug)]
  pub enum ParseError {
    Token(GetAccessTokenScopeError),
    Payload(ReadBodyJsonError),
  }

  impl From<GetAccessTokenScopeError> for ParseError {
    fn from(e: GetAccessTokenScopeError) -> Self {
      Self::Token(e)
    }
  }

  impl From<ReadBodyJsonError> for ParseError {
    fn from(e: ReadBodyJsonError) -> Self {
      Self::Payload(e)
    }
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug)]
  pub enum HandleError {
    Db(mongodb::error::Error),
    TokenOutOfScope,
    AccountNameEmpty,
    FirstNameEmpty,
    LastNameEmpty,
    EmailEmpty,
    EmailInvalid,
    PasswordTooShort,
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
        HandleError::TokenOutOfScope => ApiError::from(Kind::TokenOutOfScope),
        HandleError::AccountNameEmpty => ApiError::from(Kind::PayloadInvalid(String::from(
          "Account name is required",
        ))),
        HandleError::EmailEmpty => {
          ApiError::from(Kind::PayloadInvalid(String::from("Email is required")))
        }
        HandleError::FirstNameEmpty => {
          ApiError::from(Kind::PayloadInvalid(String::from("First name is required")))
        }
        HandleError::LastNameEmpty => {
          ApiError::from(Kind::PayloadInvalid(String::from("Last name is required")))
        }
        HandleError::EmailInvalid => {
          ApiError::from(Kind::PayloadInvalid(String::from("Email is invalid")))
        }
        HandleError::PasswordTooShort => ApiError::from(Kind::PayloadInvalid(String::from(
          "Password must have 8 characters or more",
        ))),
        HandleError::EmailExists => ApiError::from(Kind::UserEmailExists),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/register/POST/")]
  #[serde(rename_all = "camelCase")]
  pub struct Payload {
    email: String,

    password: String,

    first_name: String,

    last_name: String,

    account_name: String,

    #[serde(default)]
    #[ts(optional)]
    limits: Option<PayloadLimits>,

    #[serde(default)]
    #[ts(optional)]
    account_user_metadata: Option<Metadata>,

    #[serde(default)]
    #[ts(optional)]
    account_system_metadata: Option<Metadata>,

    #[serde(default)]
    #[ts(optional)]
    user_user_metadata: Option<Metadata>,

    #[serde(default)]
    #[ts(optional)]
    user_system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/register/POST/")]
  #[serde(rename_all = "camelCase")]
  pub struct PayloadLimits {
    #[ts(optional)]
    #[ts(type = "number")]
    listeners: Option<u64>,

    #[ts(optional)]
    #[ts(type = "number")]
    transfer: Option<u64>,

    #[ts(optional)]
    #[ts(type = "number")]
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
  #[ts(export)]
  #[ts(export_to = "../../defs/api/register/POST/")]
  #[serde(rename_all = "camelCase")]
  pub struct Output {
    pub account: PublicAccount,
    pub user: PublicUser,
    pub token: String,
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
        account_name,
        account_user_metadata,
        account_system_metadata,
        user_user_metadata,
        user_system_metadata,
        limits: payload_limits,
      } = payload;

      let email = email.trim().to_lowercase();
      let first_name = first_name.trim().to_string();
      let last_name = last_name.trim().to_string();
      let account_name = account_name.trim().to_string();

      let payload_limits = payload_limits.unwrap_or_default();
      let account_user_metadata = account_user_metadata.unwrap_or_default();
      let account_system_metadata = account_system_metadata.unwrap_or_default();
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

      if account_name.is_empty() {
        return Err(HandleError::AccountNameEmpty);
      }

      if password.len() < 8 {
        return Err(HandleError::PasswordTooShort);
      }

      let config = Config::get().await?;

      let limits = match &access_token_scope {
        AccessTokenScope::Admin | AccessTokenScope::Global => Limits {
          listeners: Limit {
            used: 0,
            avail: payload_limits.listeners.unwrap_or(config.limits.listeners),
          },
          transfer: Limit {
            used: 0,
            avail: payload_limits.transfer.unwrap_or(config.limits.transfer),
          },
          storage: Limit {
            used: 0,
            avail: payload_limits.storage.unwrap_or(config.limits.storage),
          },
        },
        AccessTokenScope::User(_) => Limits {
          listeners: Limit {
            used: 0,
            avail: config.limits.listeners,
          },
          transfer: Limit {
            used: 0,
            avail: config.limits.transfer,
          },
          storage: Limit {
            used: 0,
            avail: config.limits.storage,
          },
        },
      };

      let password = crypt::hash(password);

      let user_id = User::uid();
      let account_id = Account::uid();
      let now = Utc::now();

      let user = User {
        id: user_id.clone(),
        email,
        first_name,
        last_name,
        password: Some(password),
        account_ids: vec![account_id.clone()],
        user_metadata: user_user_metadata,
        system_metadata: user_system_metadata,
        created_at: now,
        updated_at: now,
      };

      let account = Account {
        id: account_id,
        owner_id: user_id.clone(),
        name: account_name,
        limits,
        user_metadata: account_user_metadata,
        system_metadata: account_system_metadata,
        created_at: now,
        updated_at: now,
      };

      let token = AccessToken {
        id: AccessToken::uid(),
        scope: Scope::User { user_id },
        generated_by: GeneratedBy::Register { ip, user_agent },
        last_used_at: None,
        created_at: now,
        hits: 0,
      };

      run_transaction!(session => {
        let email_exists = User::email_exists_with_session(user.email.as_str(), &mut session).await?;
        if email_exists {
          return Err(HandleError::EmailExists)
        }

        User::insert_with_session(&user, &mut session).await?;
        Account::insert_with_session(&account, &mut session).await?;
        AccessToken::insert_with_session(&token, &mut session).await?;
      });

      let out = Output {
        user: user.into_public(access_token_scope.as_public_scope()),
        account: account.into_public(access_token_scope.as_public_scope()),
        token: token.id,
      };

      Ok(out)
    }
  }
}

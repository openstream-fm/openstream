pub mod post {
  use std::net::IpAddr;

  use async_trait::async_trait;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::account::{Account, Limit, Limits, PublicAccount};
  use db::metadata::Metadata;
  use db::models::user_account_relation::{UserAccountRelation, UserAccountRelationKind};
  use db::plan::Plan;
  use db::user::{PublicUser, User};
  use db::{run_transaction, Model};
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
    #[error("account name is empty")]
    AccountNameEmpty,
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
    #[error("email is too long")]
    EmailTooLong,
    #[error("first name is too long")]
    FirstNameTooLong,
    #[error("last name is too long")]
    LastNameTooLong,
    #[error("phone is too long")]
    PhoneTooLong,
    #[error("account name is too long")]
    AccountNameTooLong,
    #[error("password too long")]
    PasswordTooLong,
    #[error("device id invalid")]
    DeviceIdInvalid,
    #[error("plan not found: {0}")]
    PlanNotFound(String),
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
        HandleError::AccountNameEmpty => {
          ApiError::PayloadInvalid(String::from("Account name is required"))
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
        HandleError::FirstNameTooLong => {
          ApiError::PayloadInvalid(String::from("First name must be of 50 characters or less"))
        }
        HandleError::LastNameTooLong => {
          ApiError::PayloadInvalid(String::from("Last name must be of 50 characters or less"))
        }
        HandleError::AccountNameTooLong => ApiError::PayloadInvalid(String::from(
          "Account name must be of 30 characters or less",
        )),
        HandleError::PhoneTooLong => {
          ApiError::PayloadInvalid(String::from("Phone must be of 20 characters or less"))
        }
        HandleError::EmailTooLong => {
          ApiError::PayloadInvalid(String::from("Email must be of 40 characters or less"))
        }
        HandleError::PasswordTooLong => {
          ApiError::PayloadInvalid(String::from("Password must be of 80 characters or less"))
        }
        HandleError::DeviceIdInvalid => {
          ApiError::PayloadInvalid(String::from("device_id is invalid"))
        }
        HandleError::PlanNotFound(id) => {
          ApiError::PayloadInvalid(format!("Plan with id {id} not found"))
        }
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/auth/user/register/POST/")]
  // #[serde(rename_all = "camelCase")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    plan_id: String,
    email: String,
    password: String,
    phone: Option<String>,
    first_name: String,
    last_name: String,
    account_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_user_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_system_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    account_user_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    account_system_metadata: Option<Metadata>,

    device_id: String,
  }

  // #[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
  // #[ts(export, export_to = "../../../defs/api/auth/user/register/POST/")]
  // // #[serde(rename_all = "camelCase")]
  // #[serde(deny_unknown_fields)]
  // pub struct PayloadLimits {
  //   #[serde(skip_serializing_if = "Option::is_none")]
  //   listeners: Option<u64>,

  //   #[serde(skip_serializing_if = "Option::is_none")]
  //   transfer: Option<u64>,

  //   #[serde(skip_serializing_if = "Option::is_none")]
  //   storage: Option<u64>,
  // }

  #[derive(Debug, Clone)]
  pub struct Input {
    pub ip: IpAddr,
    pub user_agent: UserAgent,
    pub access_token_scope: AccessTokenScope,
    pub payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/auth/user/register/POST/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    pub user: PublicUser,
    pub account: PublicAccount,
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
        plan_id,
        email,
        password,
        phone,
        first_name,
        last_name,
        account_name,
        account_user_metadata,
        account_system_metadata,
        user_user_metadata,
        user_system_metadata,
        device_id,
      } = payload;

      if !AccessToken::is_device_id_valid(&device_id) {
        return Err(HandleError::DeviceIdInvalid);
      }

      let email = email.trim().to_lowercase();
      let first_name = first_name.trim().to_string();
      let last_name = last_name.trim().to_string();
      let account_name = account_name.trim().to_string();

      let phone = match phone {
        None => None,
        Some(phone) => match phone.trim() {
          "" => None,
          phone => Some(phone.to_string()),
        },
      };

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

      if password.len() > 80 {
        return Err(HandleError::PasswordTooLong);
      }

      if first_name.len() > 50 {
        return Err(HandleError::FirstNameTooLong);
      }

      if last_name.len() > 50 {
        return Err(HandleError::FirstNameTooLong);
      }

      if email.len() > 40 {
        return Err(HandleError::EmailTooLong);
      }

      if account_name.len() > 40 {
        return Err(HandleError::AccountNameTooLong);
      }

      if let Some(ref phone) = phone {
        if phone.len() > 20 {
          return Err(HandleError::PhoneTooLong);
        }
      }

      let plan = match Plan::get_by_id(&plan_id).await? {
        Some(plan) => plan,
        None => return Err(HandleError::PlanNotFound(plan_id)),
      };

      let password = crypt::hash(password);

      let now = DateTime::now();

      let user = User {
        id: User::uid(),
        email,
        phone,
        first_name,
        last_name,
        password: Some(password),
        user_metadata: user_user_metadata,
        system_metadata: user_system_metadata,
        created_at: now,
        updated_at: now,
      };

      let limits = Limits {
        stations: Limit {
          total: plan.limits.stations,
          used: 0,
        },
        listeners: Limit {
          total: plan.limits.listeners,
          used: 0,
        },
        transfer: Limit {
          total: plan.limits.transfer,
          used: 0,
        },
        storage: Limit {
          total: plan.limits.storage,
          used: 0,
        },
      };

      let account = Account {
        id: Account::uid(),
        plan_id,
        limits,
        name: account_name,
        user_metadata: account_user_metadata,
        system_metadata: account_system_metadata,
        created_at: now,
        updated_at: now,
      };

      let relation = UserAccountRelation {
        id: UserAccountRelation::uid(),
        user_id: user.id.clone(),
        account_id: account.id.clone(),
        kind: UserAccountRelationKind::Owner,
        created_at: now,
      };

      let key = AccessToken::random_key();
      let media_key = AccessToken::random_media_key();

      let token = AccessToken {
        id: AccessToken::uid(),
        hash: crypt::sha256(&key),
        media_hash: crypt::sha256(&media_key),
        scope: Scope::User {
          user_id: user.id.clone(),
        },
        generated_by: GeneratedBy::Register {
          ip,
          user_agent,
          device_id,
        },
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
        tx_try!(Account::insert_with_session(&account, &mut session).await);
        tx_try!(UserAccountRelation::insert_with_session(&relation, &mut session).await);
        tx_try!(AccessToken::insert_with_session(&token, &mut session).await);
      });

      let out = Output {
        user: user.into_public(access_token_scope.as_public_scope()),
        account: account.into_public(access_token_scope.as_public_scope()),
        token: format!("{}-{}", token.id, key),
        media_key: format!("{}-{}", token.id, media_key),
      };

      Ok(out)
    }
  }
}

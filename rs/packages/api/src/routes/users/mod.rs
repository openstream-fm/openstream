pub mod change_password;
pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use crate::request_ext::get_access_token_scope;
use async_trait::async_trait;
use db::metadata::Metadata;
use db::user::{PublicUser, User};
use db::{Model, Paged, PublicScope};
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use validate::email::is_valid_email;

pub mod get {

  use ts_rs::TS;

  use crate::qs::{PaginationQs, VisibilityQs};

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/users/GET/")]
  pub struct Output(pub Paged<PublicUser>);

  #[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/users/GET/")]
  pub struct Query {
    #[serde(flatten)]
    pub page: PaginationQs,

    #[serde(flatten)]
    pub show: VisibilityQs,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_qs::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::QueryString(e) => e.into(),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = mongodb::error::Error;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let query = req.qs()?;

      Ok(Self::Input {
        access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope,
        query:
          Query {
            page: PaginationQs { skip, limit },
            show: VisibilityQs { show },
          },
      } = input;

      let sort = doc! { User::KEY_CREATED_AT: 1 };

      let mut filters = vec![show.to_filter_doc()];

      match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {}
        AccessTokenScope::User(user) => filters.push(doc! { User::KEY_ID: &user.id }),
      };

      let filter = doc! { "$and": filters };
      let public_scope = access_token_scope.as_public_scope();
      let page = User::paged(filter, sort, skip, limit)
        .await?
        .map(|item| item.into_public(public_scope));

      Ok(Output(page))
    }
  }
}

pub mod post {

  use db::run_transaction;
  use ts_rs::TS;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/users/POST/")]
  #[serde(rename_all = "snake_case")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    email: String,
    phone: Option<String>,
    password: String,
    first_name: String,
    last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
    #[allow(unused)]
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/users/POST/")]
  pub struct Output {
    user: PublicUser,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("user email exists")]
    UserEmailExists,
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
    #[error("password too long")]
    PasswordTooLong,
    #[error("language too long")]
    LanguageTooLong,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::UserEmailExists => ApiError::UserEmailExists,
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
        HandleError::PhoneTooLong => {
          ApiError::PayloadInvalid(String::from("Phone must be of 20 characters or less"))
        }
        HandleError::LanguageTooLong => {
          ApiError::PayloadInvalid(String::from("Language must be of 10 characters or less"))
        }
        HandleError::EmailTooLong => {
          ApiError::PayloadInvalid(String::from("Email must be of 40 characters or less"))
        }
        HandleError::PasswordTooLong => {
          ApiError::PayloadInvalid(String::from("Password must be of 80 characters or less"))
        }
      }
    }
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Self::Input, Self::ParseError> {
      let access_token_scope = get_access_token_scope(&req).await?;
      if !access_token_scope.is_admin_or_global() {
        return Err(GetAccessTokenScopeError::OutOfScope.into());
      }

      let payload: Payload = req.read_body_json(1000 * 40).await?;

      Ok(Self::Input {
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let payload = input.payload;

      let Payload {
        email,
        phone,
        password,
        first_name,
        last_name,
        language,
        user_metadata,
        system_metadata,
      } = payload;

      let email = email.trim().to_lowercase();
      let first_name = first_name.trim().to_string();
      let last_name = last_name.trim().to_string();

      let user_metadata = user_metadata.unwrap_or_default();
      let system_metadata = system_metadata.unwrap_or_default();

      let phone = match phone {
        None => None,
        Some(phone) => match phone.trim() {
          "" => None,
          phone => Some(phone.to_string()),
        },
      };

      let language = match language {
        None => None,
        Some(lang) => match lang.trim() {
          "" => None,
          lang => Some(lang.to_string()),
        },
      };

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

      if let Some(ref phone) = phone {
        if phone.len() > 20 {
          return Err(HandleError::PhoneTooLong);
        }
      }

      if let Some(ref lang) = language {
        if lang.len() > 10 {
          return Err(HandleError::LanguageTooLong);
        }
      }

      let password = crypt::hash(&password);

      let user = run_transaction!(session => {

        let email_exists = tx_try!(User::exists_with_session(doc! { User::KEY_EMAIL: &email }, &mut session).await);
        if email_exists {
          return Err(HandleError::UserEmailExists);
        }

        let now = DateTime::now();

        let user = User {
          id: User::uid(),
          email: email.clone(),
          phone: phone.clone(),
          password: Some(password.clone()),
          first_name: first_name.clone(),
          last_name: last_name.clone(),
          language: language.clone(),
          user_metadata: user_metadata.clone(),
          system_metadata: system_metadata.clone(),
          created_at: now,
          updated_at: now,
          deleted_at: None,
        };

        tx_try!(User::insert_with_session(&user, &mut session).await);

        user
      });

      Ok(Self::Output {
        user: user.into_public(PublicScope::Admin),
      })
    }
  }
}

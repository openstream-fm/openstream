pub mod change_password;
pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::{Model, Paged};
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

  use db::admin::{Admin, PublicAdmin};
  use schemars::JsonSchema;
  use ts_rs::TS;

  use crate::qs::{PaginationQs, VisibilityQs};

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Default, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/admins/GET/")]
  #[macros::schema_ts_export]
  pub struct Query {
    #[serde(flatten)]
    pub page: PaginationQs,
    #[serde(flatten)]
    pub show: VisibilityQs,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/admins/GET/")]
  #[macros::schema_ts_export]
  pub struct Output(Paged<PublicAdmin>);

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

      if !access_token_scope.is_admin_or_global() {
        return Err(GetAccessTokenScopeError::OutOfScope.into());
      }

      let query = match req.uri().query() {
        None => Default::default(),
        Some(_) => req.qs()?,
      };

      Ok(Self::Input { query })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { query } = input;

      let Query {
        page: PaginationQs { skip, limit },
        show: VisibilityQs { show },
      } = query;

      let sort = doc! { Admin::KEY_CREATED_AT: 1 };

      let page = Admin::paged(show.to_filter_doc(), sort, skip, limit)
        .await?
        .map(Admin::into_public);

      Ok(Output(page))
    }
  }
}

pub mod post {

  use constants::validate::*;
  use db::admin::{Admin, PublicAdmin};
  use db::metadata::Metadata;
  use db::run_transaction;
  use modify::Modify;
  use prex::request::ReadBodyJsonError;
  use schemars::JsonSchema;
  use serde_util::DateTime;
  use ts_rs::TS;
  use validate::email::is_valid_email;
  use validator::Validate;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(export, export_to = "../../../defs/api/admins/POST/")]
  #[macros::schema_ts_export]
  #[serde(rename_all = "snake_case", deny_unknown_fields)]
  pub struct Payload {
    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_ADMIN_FIRST_NAME_MAX_LEN",
        message = "First name is either too short or too long",
      ),
      non_control_character(message = "First name contains invalid characters")
    )]
    pub first_name: String,

    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_ADMIN_LAST_NAME_MAX_LEN",
        message = "Last name is either too short or too long",
      ),
      non_control_character(message = "Last name contains invalid characters")
    )]
    pub last_name: String,

    #[modify(trim)]
    #[validate(
      email(message = "Email is invalid"),
      length(
        min = 1,
        max = "VALIDATE_ADMIN_EMAIL_MAX_LEN",
        message = "Email is either too short or too long",
      ),
      non_control_character(message = "Email contains invalid characters")
    )]
    pub email: String,

    #[validate(length(
      min = "VALIDATE_ADMIN_PASSWORD_MIN_LEN",
      max = "VALIDATE_ADMIN_PASSWORD_MAX_LEN",
      message = "Password is either too short or too long",
    ))]
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/admins/POST/")]
  #[macros::schema_ts_export]
  pub struct Output {
    admin: PublicAdmin,
  }

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
        ParseError::Token(e) => ApiError::from(e),
        ParseError::Payload(e) => ApiError::from(e),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
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

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
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
        HandleError::EmailExists => ApiError::AdminEmailExists,
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

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      if !access_token_scope.is_global() {
        return Err(GetAccessTokenScopeError::OutOfScope.into());
      }

      let payload: Payload = req.read_body_json(10_000).await?;
      Ok(Input { payload })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input { payload } = input;

      let Payload {
        first_name,
        last_name,
        email,
        password,
        system_metadata,
      } = payload;

      let email = email.trim().to_lowercase();
      let first_name = first_name.trim().to_string();
      let last_name = last_name.trim().to_string();
      let system_metadata = system_metadata.unwrap_or_default();

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

      let password = crypt::hash(password);

      let now = DateTime::now();

      let admin = Admin {
        id: Admin::uid(),
        first_name,
        last_name,
        email,
        password,
        language: None,
        system_metadata,
        created_at: now,
        updated_at: now,
        deleted_at: None,
      };

      run_transaction!(session => {
        let admin_exists = tx_try!(Admin::exists_with_session(doc!{ Admin::KEY_EMAIL: &admin.email }, &mut session).await);
        if admin_exists {
          return Err(HandleError::EmailExists);
        }

        tx_try!(Admin::insert_with_session(&admin, &mut session).await);
      });

      let out = Output {
        admin: admin.into_public(),
      };

      Ok(out)
    }
  }
}

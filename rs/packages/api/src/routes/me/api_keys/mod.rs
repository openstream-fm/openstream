pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::access_token::{AccessToken, GeneratedBy, Scope};
use db::{Model, Paged};
use mongodb::bson::doc;
use prex::Request;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/api/")]
pub struct ApiKey {
  #[serde(rename = "_id")]
  id: String,
  is_current: bool,
  title: String,
  user_id: Option<String>,
  admin_id: Option<String>,
  created_at: DateTime,
  last_used_at: Option<DateTime>,
}

pub mod get {

  use crate::qs::{PaginationQs, VisibilityQs};

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Default, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/me/api-keys/GET/")]
  #[macros::schema_ts_export]
  pub struct Query {
    #[serde(flatten)]
    page: PaginationQs,

    #[serde(flatten)]
    show: VisibilityQs,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    admin_id: Option<String>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    current_token_id: String,
    access_token_scope: AccessTokenScope,
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/me/api-keys/GET/")]
  #[macros::schema_ts_export]
  pub struct Output(Paged<ApiKey>);

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
      let access_token = request_ext::get_access_token(&req).await?;
      let access_token_scope = request_ext::get_scope_from_token(&req, &access_token).await?;

      let query = match req.uri().query() {
        None => Default::default(),
        Some(_) => req.qs()?,
      };

      Ok(Self::Input {
        current_token_id: access_token.id,
        access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        current_token_id,
        access_token_scope,
        query:
          Query {
            page: PaginationQs { skip, limit },
            show: VisibilityQs { show },
            admin_id,
            user_id,
          },
      } = input;

      let mut filters = vec![
        show.to_filter_doc(),
        doc! {
          GeneratedBy::KEY_ENUM_TAG: { "$in": [ GeneratedBy::KEY_ENUM_VARIANT_API, GeneratedBy::KEY_ENUM_VARIANT_CLI ] }
        },
      ];

      match access_token_scope {
        AccessTokenScope::Global => match &admin_id {
          Some(admin_id) => {
            filters.push(doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_ADMIN , Scope::KEY_ADMIN_ID: admin_id });
          }

          None => match &user_id {
            Some(user_id) => {
              filters.push(doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_USER, Scope::KEY_USER_ID: user_id });
            }

            None => {}
          },
        },

        AccessTokenScope::Admin(admin) => match &user_id {
          None => {
            filters.push(doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_ADMIN, Scope::KEY_ADMIN_ID: admin.id });
          }
          Some(user_id) => {
            filters.push(doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_USER, Scope::KEY_USER_ID: user_id });
          }
        },

        AccessTokenScope::User(user) => {
          filters.push(
            doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_USER, Scope::KEY_USER_ID: user.id },
          );
        }
      };

      let filter = doc! { "$and": filters };

      // null are the smallest in mongodb so this is ok
      let sort = doc! { AccessToken::KEY_CREATED_AT: 1 };

      let page = AccessToken::paged(filter, sort, skip, limit).await?;

      let out = page.map(|token| {
        let is_current = token.id == current_token_id;

        let title = match token.generated_by {
          GeneratedBy::Api { title } | GeneratedBy::Cli { title } => title,
          // unreachable: the mongodb filter ensures this invariants
          _ => unreachable!(),
        };

        let (admin_id, user_id) = match token.scope {
          Scope::Admin { admin_id } => (Some(admin_id), None),
          Scope::User { user_id } => (None, Some(user_id)),
          Scope::AdminAsUser { .. } => (None, None),
          Scope::Global => (None, None),
        };

        ApiKey {
          id: token.id,
          is_current,
          user_id,
          admin_id,
          title,
          created_at: token.created_at,
          last_used_at: token.last_used_at,
        }
      });

      Ok(Output(out))
    }
  }
}

pub mod post {

  use crate::ip_limit;
  use constants::validate::*;
  use modify::Modify;
  use prex::request::ReadBodyJsonError;
  use schemars::JsonSchema;
  use std::net::IpAddr;
  use validator::Validate;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    ip: IpAddr,
    payload: Payload,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(export, export_to = "../../../defs/api/me/api-keys/POST/")]
  #[macros::schema_ts_export]
  pub struct Payload {
    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_ACCESS_TOKEN_TITLE_MAX_LEN",
        message = "API key title is either too short or too long",
      ),
      non_control_character(message = "API key title contains invalid characters")
    )]
    title: String,
    password: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/me/api-keys/POST/")]
  #[macros::schema_ts_export]
  pub struct Output {
    pub api_key: ApiKey,
    pub token: String,
    pub media_key: String,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("password mismatch")]
    PasswordMismatch,
    #[error("title empty")]
    TitleEmpty,
    #[error("title too long")]
    TitleTooLong,
    #[error("invalid global access scope ")]
    InvalidScopeGlobal,
    #[error("ip limit")]
    IpLimit,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::PasswordMismatch => ApiError::PayloadInvalid("Password does not match".into()),
        HandleError::InvalidScopeGlobal => {
          ApiError::BadRequestCustom("Invalid global access scope".into())
        }
        HandleError::TitleEmpty => ApiError::PayloadInvalid("API key title cannot be empty".into()),
        HandleError::TitleTooLong => {
          ApiError::PayloadInvalid("API key title cannot exceed 100 characters".into())
        }
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

    async fn parse(&self, mut req: Request) -> Result<Self::Input, Self::ParseError> {
      let ip = req.isomorphic_ip();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let payload = req.read_body_json::<Payload>(100_000).await?;

      Ok(Self::Input {
        ip,
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        ip,
        access_token_scope,
        payload,
      } = input;

      if ip_limit::should_reject(ip) {
        return Err(HandleError::IpLimit);
      }

      ip_limit::hit(ip);

      let Payload { title, password } = payload;

      let title = title.trim().to_string();

      if title.is_empty() {
        return Err(HandleError::TitleEmpty);
      }

      if title.len() > 100 {
        return Err(HandleError::TitleTooLong);
      }

      let id: String;
      let token: AccessToken;
      let key: String;
      let media_key: String;

      let now = DateTime::now();

      match access_token_scope {
        AccessTokenScope::Global => return Err(HandleError::InvalidScopeGlobal),

        AccessTokenScope::Admin(admin) => {
          if !crypt::compare(&password, &admin.password) {
            return Err(HandleError::PasswordMismatch);
          }

          id = AccessToken::uid();
          key = AccessToken::random_key();
          media_key = AccessToken::random_media_key();

          token = AccessToken {
            id: id.clone(),
            hash: crypt::sha256(&key),
            media_hash: crypt::sha256(&media_key),
            scope: db::access_token::Scope::Admin {
              admin_id: admin.id.clone(),
            },
            generated_by: GeneratedBy::Api { title },
            hits: 0,
            created_at: now,
            last_used_at: None,
            deleted_at: None,
          };
        }

        AccessTokenScope::User(user) => {
          match &user.password {
            None => return Err(HandleError::PasswordMismatch),
            Some(hashed) => {
              if !crypt::compare(&password, hashed) {
                return Err(HandleError::PasswordMismatch);
              }
            }
          }

          id = AccessToken::uid();
          key = AccessToken::random_key();
          media_key = AccessToken::random_media_key();

          token = AccessToken {
            id: id.clone(),
            hash: crypt::sha256(&key),
            media_hash: crypt::sha256(&media_key),
            scope: db::access_token::Scope::User {
              user_id: user.id.clone(),
            },
            generated_by: GeneratedBy::Api { title },
            hits: 0,
            created_at: now,
            last_used_at: None,
            deleted_at: None,
          };
        }
      };

      AccessToken::insert(&token).await?;

      let api_key = {
        let is_current = false;

        let title = match token.generated_by {
          GeneratedBy::Api { title } | GeneratedBy::Cli { title } => title,
          // unreachable: the mongodb filter ensures this invariants
          _ => unreachable!(),
        };

        let (admin_id, user_id) = match token.scope {
          Scope::Admin { admin_id } => (Some(admin_id), None),
          Scope::User { user_id } => (None, Some(user_id)),
          Scope::AdminAsUser { .. } => (None, None),
          Scope::Global => (None, None),
        };

        ApiKey {
          id: token.id,
          is_current,
          user_id,
          admin_id,
          title,
          created_at: token.created_at,
          last_used_at: token.last_used_at,
        }
      };

      let tok = format!("{}-{}", id, key);
      let media_tok = format!("{}-{}", id, media_key);

      Ok(Output {
        api_key,
        token: tok,
        media_key: media_tok,
      })
    }
  }
}

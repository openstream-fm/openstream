pub mod post {

  use async_trait::async_trait;
  use constants::validate::*;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::user::{AdminPublicUser, User};
  use db::Model;
  use modify::Modify;
  use mongodb::bson::doc;
  use prex::request::ReadBodyJsonError;
  use prex::Request;
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};
  use serde_util::DateTime;
  use ts_rs::TS;
  use validator::Validate;

  use crate::error::ApiError;
  use crate::json::JsonHandler;
  use crate::request_ext::{self, GetAccessTokenScopeError};

  #[derive(Debug, Clone)]
  pub struct Input {
    title: String,
    user_id: String,
    access_token: AccessToken,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(
    export,
    export_to = "../../../defs/api/auth/admin/delegate/[user]/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Payload {
    #[modify(trim)]
    #[validate(length(
      min = 1,
      max = "VALIDATE_ACCESS_TOKEN_TITLE_MAX_LEN",
      message = "Title is either too short or too long"
    ))]
    title: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/auth/admin/delegate/[user]/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Output {
    user: AdminPublicUser,
    token: String,
    media_key: String,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("access token scope: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("user not found: {0}")]
    UserNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::Token(e) => e.into(),
        HandleError::UserNotFound(id) => ApiError::UserNotFound(id),
      }
    }
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
        ParseError::Token(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type ParseError = ParseError;
    type HandleError = HandleError;
    type Input = Input;
    type Output = Output;

    async fn parse(&self, mut req: Request) -> Result<Input, Self::ParseError> {
      let user_id = req.param("user").unwrap().to_string();
      let access_token = request_ext::get_access_token(&req).await?;
      let Payload { title } = req.read_body_json(2_000).await?;
      Ok(Input {
        user_id,
        title,
        access_token,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Input {
        user_id,
        title,
        access_token,
      } = input;
      match access_token.scope {
        Scope::Global | Scope::User { .. } | Scope::AdminAsUser { .. } => {
          return Err(GetAccessTokenScopeError::OutOfScope.into());
        }

        Scope::Admin { admin_id } => {
          let user = match User::get_by_id(&user_id).await? {
            None => return Err(HandleError::UserNotFound(user_id)),
            Some(user) => user,
          };

          let id = AccessToken::uid();
          let key = AccessToken::random_key();
          let media_key = AccessToken::random_media_key();

          let now = DateTime::now();
          let token = AccessToken {
            id: id.clone(),
            hash: crypt::sha256(&key),
            media_hash: crypt::sha256(&media_key),
            scope: Scope::AdminAsUser { admin_id, user_id },
            generated_by: GeneratedBy::Api { title },
            hits: 0,
            created_at: now,
            last_used_at: None,
            deleted_at: None,
          };

          AccessToken::insert(&token).await?;

          let out = Output {
            user: user.into(),
            token: format!("{}-{}", id, key),
            media_key: format!("{}-{}", id, media_key),
          };

          Ok(out)
        }
      }
    }
  }
}

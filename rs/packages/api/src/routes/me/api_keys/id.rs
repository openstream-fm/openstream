use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::access_token::{AccessToken, GeneratedBy, Scope};
use db::Model;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod delete {

  use serde_util::empty_struct::EmptyStruct;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    key_id: String,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/me/api-keys/[id]/DELETE/")]
  pub struct Output(EmptyStruct);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("api key with id {0} not found")]
    ApiKeyNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::ApiKeyNotFound(id) => ApiError::ApiKeyNotFound(id),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let key_id = req.param("id").unwrap().to_string();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      Ok(Self::Input {
        access_token_scope,
        key_id,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope,
        key_id,
      } = input;

      let token = match AccessToken::get_by_id(&key_id).await? {
        Some(token) => token,
        None => return Err(HandleError::ApiKeyNotFound(key_id)),
      };

      if token.deleted_at.is_some() {
        return Err(HandleError::ApiKeyNotFound(key_id));
      }

      match &token.generated_by {
        GeneratedBy::Api { .. } | GeneratedBy::Cli { .. } => {}
        GeneratedBy::Login { .. } | GeneratedBy::Register { .. } => {
          return Err(HandleError::ApiKeyNotFound(key_id))
        }
      };

      match &access_token_scope {
        AccessTokenScope::Global => {}
        AccessTokenScope::Admin(admin) => match &token.scope {
          Scope::Global | Scope::AdminAsUser { .. } => {
            return Err(HandleError::ApiKeyNotFound(key_id))
          }
          Scope::Admin { admin_id } => {
            if admin_id != &admin.id {
              return Err(HandleError::ApiKeyNotFound(key_id));
            }
          }
          Scope::User { .. } => {}
        },

        AccessTokenScope::User(user) => match &token.scope {
          Scope::Global | Scope::Admin { .. } | Scope::AdminAsUser { .. } => {
            return Err(HandleError::ApiKeyNotFound(key_id))
          }
          Scope::User { user_id } => {
            if user_id != &user.id {
              return Err(HandleError::ApiKeyNotFound(key_id));
            }
          }
        },
      };

      AccessToken::set_deleted_by_id(&key_id).await?;

      Ok(Output(EmptyStruct(())))
    }
  }
}

pub mod patch {

  use prex::request::ReadBodyJsonError;
  use serde_util::empty_struct::EmptyStruct;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/me/api-keys/[id]/PATCH/")]
  pub struct Payload {
    title: Option<String>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    key_id: String,
    payload: Payload,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/me/api-keys/[id]/PATCH/")]
  pub struct Output(EmptyStruct);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    ReadBodyJsonError(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::ReadBodyJsonError(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("payload invalid")]
    PayloadInvalid(String),
    #[error("api key with id {0} not found")]
    ApiKeyNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::PayloadInvalid(message) => ApiError::PayloadInvalid(message),
        HandleError::ApiKeyNotFound(id) => ApiError::ApiKeyNotFound(id),
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
      let key_id = req.param("id").unwrap().to_string();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let payload = req.read_body_json::<Payload>(100_000).await?;
      Ok(Self::Input {
        key_id,
        payload,
        access_token_scope,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        key_id,
        payload,
        access_token_scope,
      } = input;

      let Payload { title } = payload;

      let token = match AccessToken::get_by_id(&key_id).await? {
        Some(token) => token,
        None => return Err(HandleError::ApiKeyNotFound(key_id)),
      };

      if token.deleted_at.is_some() {
        return Err(HandleError::ApiKeyNotFound(key_id));
      }

      match &token.generated_by {
        GeneratedBy::Api { .. } | GeneratedBy::Cli { .. } => {}
        GeneratedBy::Login { .. } | GeneratedBy::Register { .. } => {
          return Err(HandleError::ApiKeyNotFound(key_id))
        }
      };

      match &access_token_scope {
        AccessTokenScope::Global => {}
        AccessTokenScope::Admin(admin) => match &token.scope {
          Scope::Global | Scope::AdminAsUser { .. } => {
            return Err(HandleError::ApiKeyNotFound(key_id))
          }
          Scope::Admin { admin_id } => {
            if admin_id != &admin.id {
              return Err(HandleError::ApiKeyNotFound(key_id));
            }
          }
          Scope::User { .. } => {}
        },

        AccessTokenScope::User(user) => match &token.scope {
          Scope::Global | Scope::Admin { .. } | Scope::AdminAsUser { .. } => {
            return Err(HandleError::ApiKeyNotFound(key_id))
          }
          Scope::User { user_id } => {
            if user_id != &user.id {
              return Err(HandleError::ApiKeyNotFound(key_id));
            }
          }
        },
      };

      if let Some(title) = title {
        let title = title.trim().to_string();

        if title.is_empty() {
          return Err(HandleError::PayloadInvalid(String::from(
            "The title cannot be empty",
          )));
        }

        let update = doc! {
          "$set": {
            GeneratedBy::KEY_TITLE: title,
          }
        };

        AccessToken::update_by_id(&key_id, update).await?;
      }

      Ok(Output(EmptyStruct(())))
    }
  }
}

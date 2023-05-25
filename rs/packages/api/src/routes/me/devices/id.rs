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
    device_id: String,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/me/devices/[device]/DELETE/")]
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
    #[error("device with id {0} not found")]
    DeviceNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::DeviceNotFound(id) => ApiError::DeviceNotFound(id),
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
      let device_id = req.param("device").unwrap().to_string();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      Ok(Self::Input {
        access_token_scope,
        device_id,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope,
        device_id,
      } = input;

      let token = match AccessToken::get_by_id(&device_id).await? {
        Some(token) => token,
        None => return Err(HandleError::DeviceNotFound(device_id)),
      };

      if token.deleted_at.is_some() {
        return Err(HandleError::DeviceNotFound(device_id));
      }

      match &token.generated_by {
        GeneratedBy::Login { .. } | GeneratedBy::Register { .. } => {}
        GeneratedBy::Api { .. } | GeneratedBy::Cli { .. } => {
          return Err(HandleError::DeviceNotFound(device_id))
        }
      };

      match &access_token_scope {
        AccessTokenScope::Global => {}
        AccessTokenScope::Admin(admin) => match &token.scope {
          Scope::Global | Scope::AdminAsUser { .. } => {
            return Err(HandleError::DeviceNotFound(device_id))
          }
          Scope::Admin { admin_id } => {
            if admin_id != &admin.id {
              return Err(HandleError::DeviceNotFound(device_id));
            }
          }
          Scope::User { .. } => {}
        },

        AccessTokenScope::User(user) => match &token.scope {
          Scope::Global | Scope::Admin { .. } | Scope::AdminAsUser { .. } => {
            return Err(HandleError::DeviceNotFound(device_id))
          }
          Scope::User { user_id } => {
            if user_id != &user.id {
              return Err(HandleError::DeviceNotFound(device_id));
            }
          }
        },
      };

      AccessToken::set_deleted_by_id(&device_id).await?;

      Ok(Output(EmptyStruct(())))
    }
  }
}

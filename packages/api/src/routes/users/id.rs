use crate::json::JsonHandler;
use crate::request_ext::AccessTokenScope;
use crate::request_ext::{self, GetAccessTokenScopeError};

use crate::error::{ApiError, Kind};
use async_trait::async_trait;
use db::user::{PublicUser, User};
use db::Model;
use db::PublicScope;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

  use ts_rs::TS;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    user_id: String,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/users/[user]/GET/")]
  #[serde(rename_all = "camelCase")]
  pub struct Output {
    user: PublicUser,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("token out of scope")]
    TokenOutOfScope,
    #[error("user not found: {0}")]
    UserNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::TokenOutOfScope => ApiError::from(Kind::TokenOutOfScope),
        HandleError::UserNotFound(id) => ApiError::from(Kind::UserNotFound(id)),
        HandleError::Db(e) => ApiError::from(Kind::Db(e)),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let user_id = req.param("user").unwrap().to_string();

      Ok(Self::Input {
        user_id,
        access_token_scope,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Self::Input {
        user_id,
        access_token_scope,
      } = input;

      match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          let user = match User::get_by_id(&user_id).await? {
            None => return Err(HandleError::UserNotFound(user_id)),
            Some(user) => user,
          };

          let out = Output {
            user: user.into_public(PublicScope::Admin),
          };

          Ok(out)
        }

        AccessTokenScope::User(user) => {
          if user.id != user_id {
            return Err(HandleError::TokenOutOfScope);
          }

          let out = Output {
            user: user.into_public(PublicScope::User),
          };

          Ok(out)
        }
      }
    }
  }
}

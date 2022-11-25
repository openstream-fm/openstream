use crate::json::JsonHandler;
use crate::request_ext::AccessTokenScope;
use crate::request_ext::{self, GetAccessTokenScopeError};

use crate::error::{ApiError, Kind};
use async_trait::async_trait;
use db::user::{PublicUser, User};
use db::IntoPublicScope;
use db::Model;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    user_id: String,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct Output {
    user: PublicUser,
  }

  #[derive(Debug)]
  pub enum HandleError {
    UserNotFound(String),
    Db(mongodb::error::Error),
  }

  impl From<mongodb::error::Error> for HandleError {
    fn from(e: mongodb::error::Error) -> Self {
      Self::Db(e)
    }
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
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

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        user_id,
        access_token_scope,
      } = input;

      match access_token_scope {
        AccessTokenScope::Admin | AccessTokenScope::Global => {
          let user = match User::get_by_id(&user_id).await? {
            None => return Err(HandleError::UserNotFound(user_id)),
            Some(user) => user,
          };

          Ok(Self::Output {
            user: user.into_public(IntoPublicScope::Admin),
          })
        }
        AccessTokenScope::User(user) => Ok(Self::Output {
          user: user.into_public(IntoPublicScope::User),
        }),
      }
    }
  }
}

pub mod post {

  use async_trait::async_trait;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::Model;
  use log::warn;
  use mongodb::bson::doc;
  use prex::Request;
  use serde::{Deserialize, Serialize};
  use serde_util::empty_struct::EmptyStruct;
  use ts_rs::TS;

  use crate::error::ApiError;
  use crate::json::JsonHandler;
  use crate::request_ext::{self, GetAccessTokenScopeError};

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token: AccessToken,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/auth/user/logout/POST/")]
  pub struct Output(EmptyStruct);

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("access token scope: {0}")]
    Token(#[from] GetAccessTokenScopeError),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::Token(e) => e.into(),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type ParseError = GetAccessTokenScopeError;
    type HandleError = HandleError;
    type Input = Input;
    type Output = Output;

    async fn parse(&self, req: Request) -> Result<Input, Self::ParseError> {
      let access_token = request_ext::get_access_token(&req).await?;
      Ok(Input { access_token })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Input { access_token } = input;
      match &access_token.scope {
        Scope::Global | Scope::Admin { .. } => {
          return Err(GetAccessTokenScopeError::OutOfScope.into());
        }
        Scope::User { .. } | Scope::AdminAsUser { .. } => match &access_token.generated_by {
          GeneratedBy::Api { .. } | GeneratedBy::Cli { .. } => {
            return Err(GetAccessTokenScopeError::OutOfScope.into());
          }

          GeneratedBy::Login { .. } | GeneratedBy::Register { .. } => {
            let r = AccessToken::set_deleted_by_id(&access_token.id).await?;
            if r.matched_count != 1 {
              warn!(
                "AccessToken::set_deleted_by_id {} matched_count={} modified_count={}",
                access_token.id, r.matched_count, r.modified_count
              )
            }
            Ok(Output(EmptyStruct(())))
          }
        },
      }
    }
  }
}

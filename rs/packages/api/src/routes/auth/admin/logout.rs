pub mod post {

  use async_trait::async_trait;
  use db::access_token::{AccessToken, GeneratedBy, Scope};
  use db::Model;
  use mongodb::bson::doc;
  use prex::Request;
  use schemars::JsonSchema;
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

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/auth/admin/logout/POST/")]
  #[macros::schema_ts_export]
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
        Scope::Global | Scope::User { .. } | Scope::AdminAsUser { .. } => {
          return Err(GetAccessTokenScopeError::OutOfScope.into());
        }
        Scope::Admin { .. } => match &access_token.generated_by {
          GeneratedBy::Api { .. } | GeneratedBy::Cli { .. } => {
            return Err(GetAccessTokenScopeError::OutOfScope.into());
          }

          GeneratedBy::Login { .. } | GeneratedBy::Register { .. } => {
            AccessToken::set_deleted_by_id(&access_token.id).await?;
            Ok(Output(EmptyStruct(())))
          }
        },
      }
    }
  }
}

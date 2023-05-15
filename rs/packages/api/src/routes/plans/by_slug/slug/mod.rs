use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::Model;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use mongodb::bson::doc;

pub mod get {

  use crate::{error::ApiError, request_ext::AccessTokenScope};
  use db::plan::Plan;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    slug: String,
    optional_access_token_scope: Option<AccessTokenScope>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/plans/by-slug/[slug]/GET/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    plan: Plan,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("plan not found: {0}")]
    PlanNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::PlanNotFound(id) => ApiError::PlanNotFound(id),
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
      let slug = req.param("slug").unwrap().to_string();
      let optional_access_token_scope = request_ext::get_optional_access_token_scope(&req).await?;

      Ok(Self::Input {
        slug,
        optional_access_token_scope,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        slug,
        optional_access_token_scope,
      } = input;

      let plan = match Plan::get(doc! { Plan::KEY_SLUG: &slug }).await? {
        Some(plan) => plan,
        None => return Err(HandleError::PlanNotFound(slug)),
      };

      if optional_access_token_scope.is_none()
        && (!plan.is_user_selectable || plan.deleted_at.is_some())
      {
        return Err(HandleError::PlanNotFound(slug));
      }

      if matches!(optional_access_token_scope, Some(AccessTokenScope::User(_)))
        && !plan.is_user_selectable
      {
        return Err(HandleError::PlanNotFound(slug));
      }

      Ok(Output { plan })
    }
  }
}

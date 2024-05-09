use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::Model;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use crate::{error::ApiError, request_ext::AccessTokenScope};
  use db::plan::Plan;
  use schemars::JsonSchema;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    plan_id: String,
    optional_access_token_scope: Option<AccessTokenScope>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/plans/[plan]/GET/")]
  #[macros::schema_ts_export]
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
      let plan_id = req.param("plan").unwrap().to_string();

      let optional_access_token_scope = request_ext::get_optional_access_token_scope(&req).await?;

      Ok(Self::Input {
        plan_id,
        optional_access_token_scope,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        plan_id,
        optional_access_token_scope,
      } = input;

      let plan = match Plan::get_by_id(&plan_id).await? {
        Some(plan) => plan,
        None => return Err(HandleError::PlanNotFound(plan_id)),
      };

      if optional_access_token_scope.is_none()
        && (!plan.is_user_selectable || plan.deleted_at.is_some())
      {
        return Err(HandleError::PlanNotFound(plan_id));
      }

      if matches!(optional_access_token_scope, Some(AccessTokenScope::User(_)))
        && !plan.is_user_selectable
      {
        return Err(HandleError::PlanNotFound(plan_id));
      }

      Ok(Output { plan })
    }
  }
}

pub mod delete {

  use crate::error::ApiError;
  use db::plan::Plan;
  use schemars::JsonSchema;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    plan_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/plans/[plan]/DELETE/")]
  #[macros::schema_ts_export]
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
      let plan_id = req.param("plan").unwrap().to_string();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      if !access_token_scope.is_admin_or_global() {
        return Err(GetAccessTokenScopeError::OutOfScope);
      };

      Ok(Self::Input { plan_id })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { plan_id } = input;

      let plan = match Plan::get_by_id(&plan_id).await? {
        Some(plan) => plan,
        None => return Err(HandleError::PlanNotFound(plan_id)),
      };

      Plan::set_deleted_by_id(&plan.id).await?;

      Ok(Output { plan })
    }
  }
}

pub mod patch {

  use crate::error::ApiError;

  use super::*;
  use constants::validate::*;
  use db::{
    account::{Account, Limit, Limits},
    plan::Plan,
    run_transaction, Model,
  };
  use modify::Modify;
  use prex::request::ReadBodyJsonError;
  use schemars::JsonSchema;
  use serde_util::DateTime;
  use validator::Validate;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(export, export_to = "../../../defs/api/plans/[plan]/PATCH/")]
  #[macros::schema_ts_export]
  pub struct Payload {
    #[ts(optional)]
    #[validate(range(min = 0.0))]
    price: Option<f64>,

    #[ts(optional)]
    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_PLAN_IDENTIFIER_MAX_LEN",
        message = "Identifier is either too short or too long"
      ),
      non_control_character(message = "Identifier contains invalid characters")
    )]
    identifier: Option<String>,

    #[ts(optional)]
    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_PLAN_SLUG_MAX_LEN",
        message = "Slug is either too short or too long"
      ),
      non_control_character(message = "Slug contains invalid characters")
    )]
    slug: Option<String>,

    #[ts(optional)]
    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_PLAN_NAME_MAX_LEN",
        message = "Display name is either too short or too long"
      ),
      non_control_character(message = "Display name contains invalid characters")
    )]
    display_name: Option<String>,

    #[ts(optional)]
    #[modify(trim)]
    #[validate(length(min = 1, message = "Color cannot be empty"))]
    color: Option<String>,

    #[ts(optional)]
    stations: Option<u64>,

    #[ts(optional)]
    listeners: Option<u64>,

    #[ts(optional)]
    transfer: Option<u64>,

    #[ts(optional)]
    storage: Option<u64>,

    #[ts(optional)]
    is_user_selectable: Option<bool>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    plan_id: String,
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/plans/[plan]/PATCH/")]
  #[macros::schema_ts_export]
  pub struct Output(Plan);

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
        ParseError::Token(e) => Self::from(e),
        ParseError::Payload(e) => Self::from(e),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("admin not found: {0}")]
    PlanNotFound(String),
    #[error("slug exists")]
    SlugExists,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::PlanNotFound(id) => ApiError::PlanNotFound(id),
        HandleError::SlugExists => ApiError::BadRequestCustom("The slug already exists".into()),
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
      let plan_id = req.param("plan").unwrap().to_string();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      if !access_token_scope.is_admin_or_global() {
        return Err(GetAccessTokenScopeError::OutOfScope.into());
      }

      let payload: Payload = req.read_body_json(100_000).await?;

      Ok(Self::Input { plan_id, payload })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { plan_id, payload } = input;

      let Payload {
        display_name,
        identifier,
        slug,
        color,
        price,
        stations,
        listeners,
        transfer,
        storage,
        is_user_selectable,
      } = payload;

      let slug = slug.map(|s| s.trim().to_lowercase());

      let plan = run_transaction!(session => {

        if let Some(ref slug) = slug {
          let exists_filter = doc!{ Plan::KEY_ID: { "$ne": &plan_id }, Plan::KEY_SLUG: &slug };
          if tx_try!(Plan::exists_with_session(exists_filter, &mut session).await) {
            return Err(HandleError::SlugExists);
          }
        }

        let mut plan = match tx_try!(Plan::get_by_id_with_session(&plan_id, &mut session).await) {
          Some(plan) => plan,
          None => return Err(HandleError::PlanNotFound(plan_id)),
        };

        if let Some(ref identifier) = identifier {
          plan.identifier.clone_from(identifier);
        }

        if let Some(ref slug) = slug {
          plan.slug.clone_from(slug);
        }

        if let Some(ref display_name) = display_name {
          plan.display_name.clone_from(display_name);
        }

        if let Some(ref color) = color {
          plan.color.clone_from(color);
        }

        if let Some(price) = price {
          plan.price = price;
        }

        if let Some(stations) = stations {
          plan.limits.stations = stations;
        }

        if let Some(listeners) = listeners {
          plan.limits.listeners = listeners;
        }

        if let Some(transfer) = transfer {
          plan.limits.transfer = transfer;
        }

        if let Some(storage) = storage {
          plan.limits.storage = storage;
        }

        if let Some(is_user_selectable) = is_user_selectable {
          plan.is_user_selectable = is_user_selectable;
        }

        plan.updated_at = DateTime::now();

        tx_try!(Plan::replace_with_session(&plan.id, &plan, &mut session).await);

        {
          static STATIONS: &str = db::key!(Account::KEY_LIMITS, Limits::KEY_TRANSFER, Limit::KEY_TOTAL);
          static LISTENERS: &str = db::key!(Account::KEY_LIMITS, Limits::KEY_LISTENERS, Limit::KEY_TOTAL);
          static STORAGE: &str = db::key!(Account::KEY_LIMITS, Limits::KEY_STORAGE, Limit::KEY_TOTAL);
          static TRANSFER: &str = db::key!(Account::KEY_LIMITS, Limits::KEY_TRANSFER, Limit::KEY_TOTAL);

          let accounts_filter = doc!{ Account::KEY_PLAN_ID: &plan.id };

          let accounts_update = doc!{
            "$set": {
              STATIONS: plan.limits.stations as f64,
              LISTENERS: plan.limits.listeners as f64,
              STORAGE: plan.limits.storage as f64,
              TRANSFER: plan.limits.transfer as f64,
            }
          };

          tx_try!(Account::cl().update_many_with_session(accounts_filter, accounts_update, None, &mut session).await);
        }

        plan
      });

      Ok(Output(plan))
    }
  }
}

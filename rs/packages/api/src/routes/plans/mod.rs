pub mod by_slug;
pub mod id;

pub mod get {

  use async_trait::async_trait;
  use db::plan::Plan;
  use db::Model;
  use db::Paged;
  use mongodb::bson::doc;
  use prex::Request;
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};
  use ts_rs::TS;

  use crate::qs::PaginationQs;
  use crate::qs::VisibilityQs;
  use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};
  use crate::{error::ApiError, json::JsonHandler};

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Default, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/plans/GET/")]
  #[macros::schema_ts_export]
  pub struct Query {
    #[serde(flatten)]
    pub page: PaginationQs,
    #[serde(flatten)]
    pub show: VisibilityQs,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    optional_access_token_scope: Option<AccessTokenScope>,
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/plans/GET/")]
  #[macros::schema_ts_export]
  pub struct Output(Paged<Plan>);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_qs::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Token(e) => e.into(),
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
      let query = match req.uri().query() {
        None => Default::default(),
        Some(_) => req.qs()?,
      };

      let optional_access_token_scope = request_ext::get_optional_access_token_scope(&req).await?;

      Ok(Self::Input {
        optional_access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Self::Input {
        optional_access_token_scope,
        query,
      } = input;

      let Query {
        show: VisibilityQs { show },
        page: PaginationQs { skip, limit },
      } = query;

      let mut filters = vec![show.to_filter_doc()];

      match optional_access_token_scope {
        None | Some(AccessTokenScope::User(_)) => {
          filters.push(doc! { Plan::KEY_IS_USER_SELECTABLE: true });
        }
        Some(AccessTokenScope::Global | AccessTokenScope::Admin(_)) => {}
      };

      let sort = doc! { Plan::KEY_CREATED_AT: 1 };

      let filter = doc! { "$and": filters };

      let page = Plan::paged(filter, sort, skip, limit).await?;

      Ok(Output(page))
    }
  }
}

pub mod post {

  use async_trait::async_trait;
  use constants::validate::*;
  use db::Model;
  use db::{
    current_filter_doc,
    plan::{Plan, PlanLimits},
    run_transaction,
  };
  use modify::Modify;
  use mongodb::bson::doc;
  use mongodb::options::FindOneOptions;
  use prex::{request::ReadBodyJsonError, Request};
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};
  use serde_util::DateTime;
  use ts_rs::TS;
  use validator::Validate;

  use crate::{
    error::ApiError,
    json::JsonHandler,
    request_ext::{self, GetAccessTokenScopeError},
  };

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(export, export_to = "../../../defs/api/plans/POST/")]
  #[macros::schema_ts_export]
  #[serde(rename_all = "snake_case", deny_unknown_fields)]
  pub struct Payload {
    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_PLAN_IDENTIFIER_MAX_LEN",
        message = "Identifier is either too short or too long"
      ),
      non_control_character(message = "Identifier contains invalid characters")
    )]
    pub identifier: String,

    #[modify(trim, lowercase)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_PLAN_SLUG_MAX_LEN",
        message = "Slug is either too short or too long"
      ),
      non_control_character(message = "Slug contains invalid characters")
    )]
    pub slug: String,

    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_PLAN_NAME_MAX_LEN",
        message = "Display name is either too short or too long"
      ),
      non_control_character(message = "Display name contains invalid characters")
    )]
    pub display_name: String,

    pub is_user_selectable: bool,

    #[validate(range(min = 0.0))]
    pub price: f64,

    #[modify(trim)]
    #[validate(length(min = 1, message = "Color cannot be empty"))]
    pub color: String,

    pub stations: u64,
    pub listeners: u64,
    pub transfer: u64,
    pub storage: u64,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/plans/POST/")]
  #[macros::schema_ts_export]
  pub struct Output(pub Plan);

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
        ParseError::Token(e) => ApiError::from(e),
        ParseError::Payload(e) => ApiError::from(e),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("slug exists")]
    SlugExists,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::SlugExists => ApiError::BadRequestCustom("The slug is already taken".into()),
      }
    }
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      if !access_token_scope.is_admin_or_global() {
        return Err(GetAccessTokenScopeError::OutOfScope.into());
      }

      let payload: Payload = req.read_body_json(10_000).await?;
      Ok(Input { payload })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input { payload } = input;

      let Payload {
        ref identifier,
        ref slug,
        ref display_name,
        ref color,
        is_user_selectable,
        price,
        stations,
        listeners,
        transfer,
        storage,
      } = payload;

      let now = DateTime::now();

      let plan = run_transaction!(session => {

        let order = {
          let filter = current_filter_doc!{};
          let sort = doc!{ Plan::KEY_ORDER: -1 };
          let options = FindOneOptions::builder().sort(sort).build();
          let document = tx_try!(Plan::cl().find_one_with_session(filter, options, &mut session).await);
          match document {
            Some(doc) => doc.order + 1.0,
            None => 1.0,
          }
        };

        if tx_try!(Plan::exists_with_session(doc! { Plan::KEY_SLUG: &slug }, &mut session).await) {
          return Err(HandleError::SlugExists);
        }

        let plan = Plan {
          id: Plan::uid(),
          identifier: identifier.clone(),
          slug: slug.clone(),
          display_name: display_name.clone(),
          price,
          limits: PlanLimits {
            stations,
            listeners,
            transfer,
            storage,
          },
          color: color.clone(),
          order,
          is_user_selectable,
          created_at: now,
          updated_at: now,
          deleted_at: None,
        };

        tx_try!(Plan::insert_with_session(&plan, &mut session).await);

        plan
      });

      let out = Output(plan);

      Ok(out)
    }
  }
}

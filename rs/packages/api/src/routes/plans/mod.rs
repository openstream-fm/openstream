pub mod by_slug;
pub mod id;

pub mod get {

  use async_trait::async_trait;
  use db::Model;
  use db::{current_filter_doc, plan::Plan};
  use futures_util::TryStreamExt;
  use mongodb::bson::doc;
  use prex::Request;
  use serde::{Deserialize, Serialize};
  use ts_rs::TS;

  use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};
  use crate::{error::ApiError, json::JsonHandler};

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/plans/GET/")]
  #[serde(rename_all = "kebab-case")]
  pub enum Show {
    All,
    Active,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
  #[ts(export, export_to = "../../../defs/api/plans/GET/")]
  struct Query {
    #[serde(default)]
    show: Option<Show>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    optional_access_token_scope: Option<AccessTokenScope>,
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/plans/GET/")]
  pub struct Output {
    pub items: Vec<Plan>,
  }

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

      let Query { show } = query;

      let filter = match (optional_access_token_scope, show) {
        (None | Some(AccessTokenScope::User(_)), _) => {
          current_filter_doc! {
            Plan::KEY_IS_USER_SELECTABLE: true,
          }
        }
        (_, None | Some(Show::Active)) => current_filter_doc! {},
        (_, Some(Show::All)) => doc! {},
      };

      let sort = doc! { Plan::KEY_CREATED_AT: 1 };
      let options = mongodb::options::FindOptions::builder().sort(sort).build();

      let items: Vec<Plan> = Plan::cl()
        .find(filter, options)
        .await?
        .try_collect()
        .await?;

      Ok(Output { items })
    }
  }
}

pub mod post {

  use async_trait::async_trait;
  use db::Model;
  use db::{
    current_filter_doc,
    plan::{Plan, PlanLimits},
    run_transaction,
  };
  use mongodb::bson::doc;
  use mongodb::options::FindOneOptions;
  use prex::{request::ReadBodyJsonError, Request};
  use serde::{Deserialize, Serialize};
  use serde_util::DateTime;
  use ts_rs::TS;
  use validify::{validify, ValidationErrors, Validify};

  use crate::{
    error::ApiError,
    json::JsonHandler,
    request_ext::{self, GetAccessTokenScopeError},
  };

  #[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
  #[ts(export, export_to = "../../../defs/api/plans/POST/")]
  #[validify]
  #[serde(rename_all = "snake_case")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    #[modify(trim)]
    #[validate(length(min = 1))]
    pub identifier: String,

    #[modify(trim, lowercase)]
    #[validate(length(min = 1))]
    pub slug: String,

    #[modify(trim)]
    #[validate(length(min = 1))]
    pub display_name: String,

    pub is_user_selectable: bool,

    #[validate(range(min = 0.0))]
    pub price: f64,

    #[modify(trim)]
    #[validate(length(min = 1))]
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

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/plans/POST/")]
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
    #[error("validify: {0}")]
    Validify(#[from] ValidationErrors),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::SlugExists => ApiError::BadRequestCustom("The slug is already taken".into()),
        HandleError::Validify(errors) => ApiError::PayloadInvalid(format!("{}", errors)),
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

      let payload = Payload::validify(payload.into())?;

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

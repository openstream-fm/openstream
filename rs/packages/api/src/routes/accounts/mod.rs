pub mod id;
pub mod members;
pub mod stream_stats;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::account::Account;
use db::account::PublicAccount;
use db::metadata::Metadata;
use db::models::user_account_relation::UserAccountRelation;
use db::{Model, Paged, PublicScope};
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  pub const DEFAULT_SKIP: u64 = 0;
  pub const DEFAULT_LIMIT: i64 = 60;

  pub fn default_skip() -> u64 {
    DEFAULT_SKIP
  }

  pub fn default_limit() -> i64 {
    DEFAULT_LIMIT
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
  #[ts(export, export_to = "../../../defs/api/accounts/GET/")]
  struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/accounts/GET/")]
  pub struct Output(Paged<PublicAccount>);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_qs::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
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
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let query = match req.uri().query() {
        None => Default::default(),
        Some(_) => req.qs()?,
      };

      Ok(Self::Input {
        access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope,
        query,
      } = input;

      let Query {
        skip,
        limit,
        user_id,
      } = query;

      let skip = skip.unwrap_or_else(default_skip);
      let limit = limit.unwrap_or_else(default_limit);

      let query_user_filter = match user_id {
        None => doc! {},
        Some(user_id) => {
          let filter = doc! { UserAccountRelation::KEY_USER_ID: user_id };
          let account_ids = UserAccountRelation::cl()
            .distinct(UserAccountRelation::KEY_ACCOUNT_ID, filter, None)
            .await?;

          doc! { Account::KEY_ID: { "$in": account_ids } }
        }
      };

      let sort = doc! { Account::KEY_CREATED_AT: 1 };

      let page = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          Account::paged(Some(query_user_filter), Some(sort), skip, limit)
            .await?
            .map(|item| item.into_public(PublicScope::Admin))
        }

        AccessTokenScope::User(user) => {
          let filter = doc! { UserAccountRelation::KEY_USER_ID: &user.id };
          let account_ids = UserAccountRelation::cl()
            .distinct(UserAccountRelation::KEY_ACCOUNT_ID, filter, None)
            .await?;

          if account_ids.is_empty() {
            return Ok(Output(Paged {
              items: vec![],
              limit,
              skip,
              total: 0,
            }));
          }

          let filter =
            doc! { "$and": [ query_user_filter, { Account::KEY_ID: { "$in": account_ids } } ] };

          Account::paged(filter, Some(sort), skip, limit)
            .await?
            .map(|item| item.into_public(PublicScope::User))
        }
      };

      Ok(Output(page))
    }
  }
}

pub mod post {

  use db::account::{Limit, Limits};
  use db::models::user_account_relation::UserAccountRelationKind;
  use db::plan::Plan;
  use db::run_transaction;
  use db::user::User;
  use serde_util::DateTime;
  use ts_rs::TS;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/accounts/POST/")]
  #[serde(rename_all = "snake_case")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    pub name: String,
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/accounts/POST/")]
  pub struct Output {
    account: PublicAccount,
  }

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
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("name missing")]
    PlanNotFound(String),
    #[error("name missing")]
    NameMissing,
    #[error("user id missing")]
    UserIdMissing,
    #[error("account not found ({0})")]
    UserNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => ApiError::from(e),
        HandleError::Token(e) => ApiError::from(e),
        HandleError::PlanNotFound(id) => {
          ApiError::PayloadInvalid(format!("Plan with id {id} not found"))
        }
        HandleError::NameMissing => ApiError::PayloadInvalid(String::from("Name is required")),
        HandleError::UserIdMissing => ApiError::PayloadInvalid(String::from("user_id is required")),
        HandleError::UserNotFound(id) => ApiError::UserNotFound(id),
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
      let payload: Payload = req.read_body_json(10_000).await?;
      Ok(Input {
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        access_token_scope,
        payload,
      } = input;

      let Payload {
        name,
        plan_id,
        user_id,
        user_metadata,
        system_metadata,
      } = payload;

      let name = name.trim().to_string();

      if name.is_empty() {
        return Err(HandleError::NameMissing);
      }

      let plan = match Plan::get_by_id(&plan_id).await? {
        Some(plan) => plan,
        None => return Err(HandleError::PlanNotFound(plan_id)),
      };

      if access_token_scope.is_user() && !plan.is_user_selectable {
        return Err(HandleError::PlanNotFound(plan_id));
      }

      // TODO: validate name length

      let system_metadata = match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          system_metadata.unwrap_or_default()
        }

        AccessTokenScope::User(_) => Default::default(),
      };

      let user_metadata = user_metadata.unwrap_or_default();

      let user_id = match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => match user_id {
          None => return Err(HandleError::UserIdMissing),
          Some(user_id) => user_id,
        },

        AccessTokenScope::User(user) => user.id.clone(),
      };

      let limits = Limits {
        stations: Limit {
          total: plan.limits.stations,
          used: 0,
        },
        listeners: Limit {
          total: plan.limits.listeners,
          used: 0,
        },
        transfer: Limit {
          total: plan.limits.transfer,
          used: 0,
        },
        storage: Limit {
          total: plan.limits.storage,
          used: 0,
        },
      };

      let now = DateTime::now();

      let account = Account {
        id: Account::uid(),
        plan_id,
        name,
        limits,
        system_metadata,
        user_metadata,
        created_at: now,
        updated_at: now,
      };

      let relation = UserAccountRelation {
        id: UserAccountRelation::uid(),
        user_id: user_id.clone(),
        account_id: account.id.clone(),
        kind: UserAccountRelationKind::Owner,
        created_at: now,
      };

      run_transaction!(session => {
        let filter = doc! { User::KEY_ID: &user_id };
        if !tx_try!(User::exists_with_session(filter, &mut session).await) {
          return Err(HandleError::UserNotFound(user_id.clone()));
        }

        tx_try!(Account::insert_with_session(&account, &mut session).await);
        tx_try!(UserAccountRelation::insert_with_session(&relation, &mut session).await);
      });

      let out = Output {
        account: account.into_public(access_token_scope.as_public_scope()),
      };

      Ok(out)
    }
  }
}

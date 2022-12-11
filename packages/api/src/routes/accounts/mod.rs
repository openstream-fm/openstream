pub mod files;
pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use chrono::Utc;
use db::account::Account;
use db::account::PublicAccount;
use db::metadata::Metadata;
use db::run_transaction;
use db::{Model, Paged, PublicScope, Singleton};
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

  use ts_rs::TS;

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
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/GET/")]
  struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/GET/")]
  pub struct Output(Paged<PublicAccount>);

  #[derive(Debug)]
  pub enum ParseError {
    Access(GetAccessTokenScopeError),
    QueryString(serde_querystring::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::QueryString(e) => e.into(),
      }
    }
  }

  impl From<GetAccessTokenScopeError> for ParseError {
    fn from(e: GetAccessTokenScopeError) -> Self {
      Self::Access(e)
    }
  }

  impl From<serde_querystring::Error> for ParseError {
    fn from(e: serde_querystring::Error) -> Self {
      Self::QueryString(e)
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
        Some(qs) => serde_querystring::from_str(qs)?,
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

      let Query { skip, limit } = query;

      let skip = skip.unwrap_or_else(default_skip);
      let limit = limit.unwrap_or_else(default_limit);

      let page = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => Account::paged(None, skip, limit)
          .await?
          .map(|item| item.into_public(PublicScope::Admin)),

        AccessTokenScope::User(user) => {
          let filter = mongodb::bson::doc! { "_id": { "$in": user.account_ids } };
          Account::paged(filter, skip, limit)
            .await?
            .map(|item| item.into_public(PublicScope::User))
        }
      };

      Ok(Output(page))
    }
  }
}

pub mod post {

  use db::account::{Account, Limit, Limits};
  use db::{config::Config, user::User};
  use ts_rs::TS;

  use crate::error::Kind;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/POST/")]
  #[serde(rename_all = "camelCase")]
  pub struct Payload {
    pub name: String,
    /// User.id who created this account
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<PayloadLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/POST/")]
  #[serde(rename_all = "camelCase")]
  pub struct PayloadLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    listeners: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transfer: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<u64>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/POST/")]
  pub struct Output {
    account: PublicAccount,
  }

  #[derive(Debug)]
  pub enum ParseError {
    Token(GetAccessTokenScopeError),
    Payload(ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => ApiError::from(e),
        ParseError::Payload(e) => ApiError::from(e),
      }
    }
  }

  impl From<ReadBodyJsonError> for ParseError {
    fn from(e: ReadBodyJsonError) -> Self {
      ParseError::Payload(e)
    }
  }

  impl From<GetAccessTokenScopeError> for ParseError {
    fn from(e: GetAccessTokenScopeError) -> Self {
      Self::Token(e)
    }
  }

  #[derive(Debug)]
  pub enum HandleError {
    Db(mongodb::error::Error),
    NameMissing,
    OwnerIdMissing,
    OwnerIdOutOfScope,
    UserNotFound(String),
  }

  impl From<mongodb::error::Error> for HandleError {
    fn from(e: mongodb::error::Error) -> Self {
      Self::Db(e)
    }
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => ApiError::from(e),
        HandleError::NameMissing => {
          ApiError::from(Kind::PayloadInvalid(String::from("Name is required")))
        }
        HandleError::OwnerIdMissing => {
          ApiError::from(Kind::PayloadInvalid(String::from("ownerId is required")))
        }
        HandleError::OwnerIdOutOfScope => ApiError::from(Kind::PayloadInvalid(String::from(
          "Specifying ownerId field requires greater access scope",
        ))),
        HandleError::UserNotFound(id) => ApiError::from(Kind::UserNotFound(id)),
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
        owner_id,
        limits: payload_limits,
        user_metadata,
        system_metadata,
      } = payload;

      let name = name.trim().to_string();

      if name.is_empty() {
        return Err(HandleError::NameMissing);
      }

      let (owner_id, system_metadata) = match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          let owner_id = match owner_id {
            None => return Err(HandleError::OwnerIdMissing),
            Some(v) => v,
          };

          (owner_id, system_metadata.unwrap_or_default())
        }

        AccessTokenScope::User(user) => {
          let owner_id = match owner_id {
            Some(_) => return Err(HandleError::OwnerIdOutOfScope),
            None => user.id.clone(),
          };

          (owner_id, Metadata::default())
        }
      };

      let config = Config::get().await?;

      let limits = match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          let payload_limits = payload_limits.unwrap_or_default();
          Limits {
            listeners: Limit {
              used: 0,
              avail: payload_limits.listeners.unwrap_or(config.limits.listeners),
            },
            transfer: Limit {
              used: 0,
              avail: payload_limits.transfer.unwrap_or(config.limits.transfer),
            },
            storage: Limit {
              used: 0,
              avail: payload_limits.storage.unwrap_or(config.limits.storage),
            },
          }
        }
        AccessTokenScope::User(_) => Limits {
          listeners: Limit {
            used: 0,
            avail: config.limits.listeners,
          },
          transfer: Limit {
            used: 0,
            avail: config.limits.transfer,
          },
          storage: Limit {
            used: 0,
            avail: config.limits.storage,
          },
        },
      };

      let now = Utc::now();

      let user_metadata = user_metadata.unwrap_or_default();

      let account = Account {
        id: Account::uid(),
        owner_id,
        name,
        limits,
        system_metadata,
        user_metadata,
        created_at: now,
        updated_at: now,
      };

      run_transaction!(session => {
        let user_exists = User::exists_with_session(account.owner_id.as_ref(), &mut session).await?;
        if !user_exists {
          return Err(HandleError::UserNotFound(account.owner_id));
        }

        Account::insert_with_session(&account, &mut session).await?;
      });

      let out = Output {
        account: account.into_public(access_token_scope.as_public_scope()),
      };

      Ok(out)
    }
  }
}

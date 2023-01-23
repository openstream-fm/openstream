pub mod dashboard_stats;
pub mod files;
pub mod id;
pub mod now_playing;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::metadata::Metadata;
use db::run_transaction;
use db::station::PublicStation;
use db::station::Station;
use db::{Model, Paged, PublicScope, Singleton};
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

  use db::models::user_station_relation::UserStationRelation;
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
  #[ts(export, export_to = "../../defs/api/stations/GET/")]
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
  #[ts(export, export_to = "../../defs/api/stations/GET/")]
  pub struct Output(Paged<PublicStation>);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_querystring::de::Error),
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
        Some(qs) => serde_querystring::from_str(qs, serde_querystring::de::ParseMode::UrlEncoded)?,
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
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          Station::paged(None, None, skip, limit)
            .await?
            .map(|item| item.into_public(PublicScope::Admin))
        }

        AccessTokenScope::User(user) => {
          let filter = doc! { UserStationRelation::KEY_USER_ID: &user.id };
          let station_ids = UserStationRelation::cl()
            .distinct(UserStationRelation::KEY_STATION_ID, filter, None)
            .await?;

          if station_ids.is_empty() {
            return Ok(Output(Paged {
              items: vec![],
              limit,
              skip,
              total: 0,
            }));
          }

          let filter = mongodb::bson::doc! { Station::KEY_ID: { "$in": station_ids } };

          Station::paged(filter, None, skip, limit)
            .await?
            .map(|item| item.into_public(PublicScope::User))
        }
      };

      Ok(Output(page))
    }
  }
}

pub mod post {

  use db::models::user_station_relation::{UserStationRelation, UserStationRelationKind};
  use db::station::{Limit, Limits, Station};
  use db::{config::Config, user::User};
  use serde_util::DateTime;
  use ts_rs::TS;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/stations/POST/")]
  #[serde(rename_all = "snake_case")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    pub name: String,
    /// user.id who created this station
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<PayloadLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/stations/POST/")]
  #[serde(rename_all = "snake_case")]
  #[serde(deny_unknown_fields)]
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
  #[ts(export, export_to = "../../defs/api/stations/POST/")]
  pub struct Output {
    station: PublicStation,
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
    #[error("name missing")]
    NameMissing,
    #[error("owner id missing")]
    OwnerIdMissing,
    #[error("owner id out of scope")]
    OwnerIdOutOfScope,
    #[error("user not found ({0})")]
    UserNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => ApiError::from(e),
        HandleError::NameMissing => ApiError::PayloadInvalid(String::from("Name is required")),
        HandleError::OwnerIdMissing => {
          ApiError::PayloadInvalid(String::from("ownerId is required"))
        }
        HandleError::OwnerIdOutOfScope => ApiError::PayloadInvalid(String::from(
          "Specifying ownerId field requires greater access scope",
        )),
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

      let config = <Config as Singleton>::get().await?;

      let limits = match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          let payload_limits = payload_limits.unwrap_or_default();
          Limits {
            listeners: Limit {
              used: 0,
              total: payload_limits.listeners.unwrap_or(config.limits.listeners),
            },
            transfer: Limit {
              used: 0,
              total: payload_limits.transfer.unwrap_or(config.limits.transfer),
            },
            storage: Limit {
              used: 0,
              total: payload_limits.storage.unwrap_or(config.limits.storage),
            },
          }
        }
        AccessTokenScope::User(_) => Limits {
          listeners: Limit {
            used: 0,
            total: config.limits.listeners,
          },
          transfer: Limit {
            used: 0,
            total: config.limits.transfer,
          },
          storage: Limit {
            used: 0,
            total: config.limits.storage,
          },
        },
      };

      let now = DateTime::now();

      let user_metadata = user_metadata.unwrap_or_default();

      let station = Station {
        id: Station::uid(),
        name,
        limits,
        source_password: Station::random_source_password(),
        system_metadata,
        user_metadata,
        created_at: now,
        updated_at: now,
      };

      let relation = UserStationRelation {
        id: UserStationRelation::uid(),
        station_id: station.id.clone(),
        user_id: owner_id.clone(),
        kind: UserStationRelationKind::Owner,
        created_at: now,
      };

      run_transaction!(session => {
        let user_exists = tx_try!(User::exists_with_session(owner_id.as_ref(), &mut session).await);
        if !user_exists {
          return Err(HandleError::UserNotFound(owner_id));
        }

        tx_try!(Station::insert_with_session(&station, &mut session).await);
        tx_try!(UserStationRelation::insert_with_session(&relation, &mut session).await);
      });

      let out = Output {
        station: station.into_public(access_token_scope.as_public_scope()),
      };

      Ok(out)
    }
  }
}

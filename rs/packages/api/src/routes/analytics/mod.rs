use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use crate::{error::ApiError, request_ext::AccessTokenScope};
  use db::stream_connection::analytics;
  use db::Model;
  use db::{station::Station, stream_connection::analytics::Analytics};
  use mongodb::bson::Bson;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    pub access_token_scope: AccessTokenScope,
    pub query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/analytics/GET/")]
  pub struct Output {
    pub analytics: Analytics,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/analytics/GET/")]
  pub struct Query {
    #[ts(type = "/** time::DateTime */ string")]
    #[serde(with = "time::serde::iso8601")]
    pub since: time::OffsetDateTime,

    #[ts(type = "/** time::DateTime */ string")]
    #[serde(with = "time::serde::iso8601")]
    pub until: time::OffsetDateTime,

    #[serde(default)]
    #[ts(type = "string[] | undefined")]
    /// ommiting this value means all available stations
    /// for the current access scope (this is valid only for admin and global access token scopes)
    pub stations: Vec<String>,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("query: {0}")]
    Query(#[from] serde_qs::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
        ParseError::Query(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("query: {0}")]
    Db(#[from] mongodb::error::Error),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Token(e) => e.into(),
        HandleError::Db(e) => e.into(),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Input, ParseError> {
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let query: Query = req.qs()?;

      Ok(Input {
        access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        access_token_scope,
        query,
      } = input;

      let Query {
        since,
        until,
        stations: station_ids,
      } = query;

      let station_ids = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          if station_ids.is_empty() {
            let values = Station::cl().distinct(Station::KEY_ID, None, None).await?;
            values
              .into_iter()
              .filter_map(|v| {
                match v {
                  Bson::String(s) => Some(s),
                  // this will never happen
                  _ => None,
                }
              })
              .collect::<Vec<String>>()
          } else {
            station_ids
          }
        }

        AccessTokenScope::User(_) => {
          if station_ids.is_empty() {
            station_ids
          } else {
            for id in station_ids.iter() {
              access_token_scope.grant_station_scope(id).await?;
            }
            station_ids
          }
        }
      };

      let query = analytics::AnalyticsQuery {
        station_ids,
        start_date: since,
        end_date: until,
      };

      let analytics = analytics::get_analytics(query).await?;

      let out = Output { analytics };

      Ok(out)
    }
  }
}

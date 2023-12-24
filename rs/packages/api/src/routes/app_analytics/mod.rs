use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use crate::{error::ApiError, request_ext::AccessTokenScope};
  use db::stream_connection::app_analytics;
  use db::Model;
  use db::{station::Station, stream_connection::app_analytics::Analytics};
  use geoip::CountryCode;
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
  #[ts(export_to = "../../../defs/api/app-analytics/GET/")]
  pub struct Output {
    pub analytics: Analytics,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/app-analytics/GET/")]
  #[serde(untagged)]
  pub enum CountryCodeOrZZ {
    ZZ(ZZ),
    CC(CountryCode),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/app-analytics/GET/")]
  pub enum ZZ {
    ZZ,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/app-analytics/GET/")]
  pub struct Query {
    pub kind: app_analytics::AnalyticsQueryKind,

    #[serde(default)]
    #[ts(type = "string[] | undefined")]
    /// ommiting this value means all available stations
    /// for the current access scope (this is valid only for admin and global access token scopes)
    pub stations: Vec<String>,

    // #[serde(default)]
    // pub browser: Option<String>,

    // #[serde(default)]
    // pub os: Option<String>,

    // #[serde(default)]
    // pub domain: Option<String>,
    #[serde(default)]
    pub app_kind: Option<String>,

    // map to u32
    #[serde(default)]
    pub app_version: Option<String>,

    #[serde(default)]
    pub country_code: Option<CountryCodeOrZZ>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration_ms: Option<u64>,
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
        kind,
        stations: station_ids,
        country_code,
        // browser,
        // os,
        // domain,
        app_kind,
        app_version,
        min_duration_ms,
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

      // let os = match os {
      //   None => None,
      //   Some(null) if null == "null" => Some(None),
      //   Some(os) => Some(Some(os)),
      // };

      // let browser = match browser {
      //   None => None,
      //   Some(null) if null == "null" => Some(None),
      //   Some(browser) => Some(Some(browser)),
      // };

      // let domain = match domain {
      //   None => None,
      //   Some(null) if null == "null" => Some(None),
      //   Some(domain) => Some(Some(domain)),
      // };

      let app_kind = match app_kind {
        None => None,
        Some(null) if null == "null" => Some(None),
        Some(app_kind) => Some(Some(app_kind)),
      };

      let app_version = match app_version {
        None => None,
        Some(null) if null == "null" => Some(None),
        Some(s) => match s.parse::<u32>() {
          Ok(v) => Some(Some(v)),
          Err(_) => None,
        },
      };

      let country_code = match country_code {
        None => None,
        Some(CountryCodeOrZZ::ZZ(_)) => Some(None),
        Some(CountryCodeOrZZ::CC(cc)) => Some(Some(cc)),
      };

      let query = app_analytics::AnalyticsQuery {
        kind,
        station_ids,
        country_code,
        // os,
        // browser,
        // domain,
        app_kind,
        app_version,
        min_duration_ms: Some(min_duration_ms.unwrap_or(5_000)),
      };

      let analytics = app_analytics::get_analytics(query).await?;

      let out = Output { analytics };

      Ok(out)
    }
  }
}

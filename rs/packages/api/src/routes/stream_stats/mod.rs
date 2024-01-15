use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::stream_connection::index::{AllFilter, MemIndex};
use db::stream_connection::stats::Stats;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use db::stream_connection::index::StationQuery;
  use schemars::JsonSchema;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub index: MemIndex,
  }

  #[derive(Debug, Clone)]
  pub struct Input {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/stream-stats/GET/")]
  #[macros::schema_ts_export]
  pub struct Output {
    pub stats: Stats,
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = mongodb::error::Error;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      if !access_token_scope.is_admin_or_global() {
        return Err(GetAccessTokenScopeError::OutOfScope);
      };
      Ok(Input {})
    }

    async fn perform(&self, _input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      // let filter = doc! {};
      // let stats = Stats::get_for_filter(filter).await?;
      let stats = self.index.get_stats(StationQuery::all(), AllFilter).await;
      Ok(Output { stats })
    }
  }
}

pub mod now {
  use super::*;
  pub mod get {
    use db::stream_connection::{
      index::{IsOpenFilter, StationQuery},
      stats::StatsItem,
    };
    use schemars::JsonSchema;

    use super::*;

    #[derive(Debug, Clone)]
    pub struct Endpoint {
      pub index: MemIndex,
    }

    #[derive(Debug, Clone)]
    pub struct Input {}

    #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
    #[ts(export, export_to = "../../../defs/api/stream-stats/now/GET/")]
    #[macros::schema_ts_export]
    pub struct Output {
      pub stats: StatsItem,
    }

    #[async_trait]
    impl JsonHandler for Endpoint {
      type Input = Input;
      type Output = Output;
      type ParseError = GetAccessTokenScopeError;
      type HandleError = mongodb::error::Error;

      async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
        let access_token_scope = request_ext::get_access_token_scope(&req).await?;
        if !access_token_scope.is_admin_or_global() {
          return Err(GetAccessTokenScopeError::OutOfScope);
        };
        Ok(Input {})
      }

      async fn perform(&self, _input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        // let filter = doc! {};
        // let stats = Stats::get_for_filter(filter).await?;
        let filter = IsOpenFilter(true);
        let stats = self.index.get_stats_item(StationQuery::all(), filter).await;
        Ok(Output { stats })
      }
    }
  }

  pub mod count {
    use super::*;
    pub mod get {

      use db::stream_connection::index::{IsOpenFilter, StationQuery};
      use schemars::JsonSchema;

      use super::*;
      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {}

      #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
      #[ts(export, export_to = "../../../defs/api/stream-stats/now/count/GET/")]
      #[macros::schema_ts_export]
      pub struct Output {
        pub total: usize,
      }

      #[async_trait]
      impl JsonHandler for Endpoint {
        type Input = Input;
        type Output = Output;
        type ParseError = GetAccessTokenScopeError;
        type HandleError = mongodb::error::Error;

        async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
          let access_token_scope = request_ext::get_access_token_scope(&req).await?;
          if !access_token_scope.is_admin_or_global() {
            return Err(GetAccessTokenScopeError::OutOfScope);
          };
          Ok(Input {})
        }

        async fn perform(&self, _input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          let total = self
            .index
            .count(StationQuery::All, IsOpenFilter(true))
            .await;
          Ok(Output { total })
        }
      }
    }
  }

  pub mod count_by_station {
    use super::*;
    pub mod get {

      use std::collections::{HashMap, HashSet};

      use db::{station::Station, stream_connection::index::IsOpenFilter, Model};
      use schemars::JsonSchema;

      use super::*;
      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {}

      #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
      #[ts(
        export,
        export_to = "../../../defs/api/stream-stats/now/count-by-station/GET/"
      )]
      #[macros::schema_ts_export]
      pub struct Output {
        pub by_station: HashMap<String, u32>,
      }

      #[async_trait]
      impl JsonHandler for Endpoint {
        type Input = Input;
        type Output = Output;
        type ParseError = GetAccessTokenScopeError;
        type HandleError = mongodb::error::Error;

        async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
          let access_token_scope = request_ext::get_access_token_scope(&req).await?;
          if !access_token_scope.is_admin_or_global() {
            return Err(GetAccessTokenScopeError::OutOfScope);
          };
          Ok(Input {})
        }

        async fn perform(&self, _input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          let bson_ids = Station::cl().distinct(Station::KEY_ID, None, None).await?;
          let mut station_ids = HashSet::with_capacity(bson_ids.len());
          for bson_id in bson_ids.into_iter() {
            let id: String = mongodb::bson::from_bson(bson_id).unwrap();
            station_ids.insert(id);
          }

          let by_station = self
            .index
            .count_by_station(station_ids, IsOpenFilter(true))
            .await;

          Ok(Output { by_station })
        }
      }
    }
  }
}

pub mod since {
  use super::*;
  pub mod get {
    use db::stream_connection::{
      index::{SinceFilter, StationQuery},
      stats::StatsItem,
    };
    use schemars::JsonSchema;

    use crate::error::ApiError;

    use super::*;

    #[derive(Debug, Clone)]
    pub struct Endpoint {
      pub index: MemIndex,
    }

    #[derive(Debug, Clone)]
    pub struct Input {
      duration: time::Duration,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
    #[ts(
      export,
      export_to = "../../../defs/api/stream-stats/[last-unitvalue]/GET/"
    )]
    #[macros::schema_ts_export]
    pub struct Output {
      pub stats: StatsItem,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum ParseError {
      #[error("invalid num: {0}")]
      InvalidNum(String),
      #[error("invalid unit: {0}")]
      InvalidUnit(String),
      #[error("token: {0}")]
      Token(#[from] GetAccessTokenScopeError),
    }

    impl From<ParseError> for ApiError {
      fn from(e: ParseError) -> Self {
        match e {
          ParseError::InvalidNum(_) => ApiError::ResourceNotFound,
          ParseError::InvalidUnit(_) => ApiError::ResourceNotFound,
          ParseError::Token(e) => e.into(),
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
        let num_param = req.param("num").unwrap();
        let num: u32 = match num_param.parse() {
          Err(_) => return Err(ParseError::InvalidNum(num_param.to_string())),
          Ok(v) => v,
        };

        let unit = match req.param("unit").unwrap() {
          "ms" => time::Duration::MILLISECOND,
          "s" => time::Duration::SECOND,
          "min" => time::Duration::MINUTE,
          "h" => time::Duration::HOUR,
          "d" => time::Duration::DAY,
          "w" => time::Duration::WEEK,
          invalid => return Err(ParseError::InvalidUnit(invalid.to_string())),
        };

        let duration: time::Duration = num * unit;

        let access_token_scope = request_ext::get_access_token_scope(&req).await?;
        if !access_token_scope.is_admin_or_global() {
          return Err(ParseError::Token(GetAccessTokenScopeError::OutOfScope));
        };
        Ok(Input { duration })
      }

      async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        let Input { duration } = input;
        let stats = self
          .index
          .get_stats_item(StationQuery::all(), SinceFilter::new(duration))
          .await;
        Ok(Output { stats })
      }
    }
  }

  pub mod count {
    use super::*;
    pub mod get {
      use db::stream_connection::index::{SinceFilter, StationQuery};
      use schemars::JsonSchema;

      use crate::error::ApiError;

      use super::*;

      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {
        duration: time::Duration,
      }

      #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
      #[ts(
        export,
        export_to = "../../../defs/api/stream-stats/[last-unitvalue]/count/GET/"
      )]
      #[macros::schema_ts_export]
      pub struct Output {
        pub total: usize,
      }

      #[derive(Debug, thiserror::Error)]
      pub enum ParseError {
        #[error("invalid num: {0}")]
        InvalidNum(String),
        #[error("invalid unit: {0}")]
        InvalidUnit(String),
        #[error("token: {0}")]
        Token(#[from] GetAccessTokenScopeError),
      }

      impl From<ParseError> for ApiError {
        fn from(e: ParseError) -> Self {
          match e {
            ParseError::InvalidNum(_) => ApiError::ResourceNotFound,
            ParseError::InvalidUnit(_) => ApiError::ResourceNotFound,
            ParseError::Token(e) => e.into(),
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
          let num_param = req.param("num").unwrap();
          let num: u32 = match num_param.parse() {
            Err(_) => return Err(ParseError::InvalidNum(num_param.to_string())),
            Ok(v) => v,
          };

          let unit = match req.param("unit").unwrap() {
            "ms" => time::Duration::MILLISECOND,
            "s" => time::Duration::SECOND,
            "min" => time::Duration::MINUTE,
            "h" => time::Duration::HOUR,
            "d" => time::Duration::DAY,
            "w" => time::Duration::WEEK,
            invalid => return Err(ParseError::InvalidUnit(invalid.to_string())),
          };

          let duration: time::Duration = num * unit;

          let access_token_scope = request_ext::get_access_token_scope(&req).await?;
          if !access_token_scope.is_admin_or_global() {
            return Err(ParseError::Token(GetAccessTokenScopeError::OutOfScope));
          };
          Ok(Input { duration })
        }

        async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          let Input { duration } = input;
          let filter = SinceFilter::new(duration);
          let total = self.index.count(StationQuery::all(), filter).await;
          Ok(Output { total })
        }
      }
    }
  }
}

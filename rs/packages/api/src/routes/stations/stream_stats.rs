use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::stream_connection::stats::Stats;
// use db::stream_connection::StreamConnection;
use db::stream_connection::index::MemIndex;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use db::stream_connection::index::{AllFilter, StationQuery};
  use schemars::JsonSchema;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub index: MemIndex,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    station_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/stream-stats/GET/"
  )]
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
      let station_id = req.param("station").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let station = access_token_scope.grant_station_scope(station_id).await?;
      Ok(Input {
        station_id: station.id,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Input { station_id } = input;
      //let filter = doc! { StreamConnection::KEY_STATION_ID: station_id };
      //let stats = Stats::get_for_filter(filter).await?;

      let stats = self
        .index
        .get_stats(StationQuery::one(station_id), AllFilter)
        .await;

      Ok(Output { stats })
    }
  }
}

pub mod now {
  use super::*;
  pub mod get {
    use super::*;

    use db::stream_connection::{
      index::{IsOpenFilter, StationQuery},
      stats::StatsItem,
    };
    use schemars::JsonSchema;

    #[derive(Debug, Clone)]
    pub struct Endpoint {
      pub index: MemIndex,
    }

    #[derive(Debug, Clone)]
    pub struct Input {
      station_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
    #[ts(
      export,
      export_to = "../../../defs/api/stations/[station]/stream-stats/now/GET/"
    )]
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
        let station_id = req.param("station").unwrap();
        let access_token_scope = request_ext::get_access_token_scope(&req).await?;
        let station = access_token_scope.grant_station_scope(station_id).await?;
        Ok(Input {
          station_id: station.id,
        })
      }

      async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        let Input { station_id } = input;
        let stats = self
          .index
          .get_stats_item(StationQuery::one(station_id), IsOpenFilter(true))
          .await;

        Ok(Output { stats })
      }
    }
  }

  pub mod count {
    use super::*;
    pub mod get {
      use super::*;

      use db::stream_connection::index::{IsOpenFilter, StationQuery};
      use schemars::JsonSchema;

      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {
        station_id: String,
      }

      #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
      #[ts(
        export,
        export_to = "../../../defs/api/stations/[station]/stream-stats/now/count/GET/"
      )]
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
          let station_id = req.param("station").unwrap();
          let access_token_scope = request_ext::get_access_token_scope(&req).await?;
          let station = access_token_scope.grant_station_scope(station_id).await?;
          Ok(Input {
            station_id: station.id,
          })
        }

        async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          let Input { station_id } = input;
          let total = self
            .index
            .count(StationQuery::one(station_id), IsOpenFilter(true))
            .await;

          Ok(Output { total })
        }
      }
    }
  }
}

pub mod since {
  use super::*;
  pub mod get {
    use crate::error::ApiError;

    use super::*;

    use db::stream_connection::{
      index::{SinceFilter, StationQuery},
      stats::StatsItem,
    };
    use schemars::JsonSchema;

    #[derive(Debug, Clone)]
    pub struct Endpoint {
      pub index: MemIndex,
    }

    #[derive(Debug, Clone)]
    pub struct Input {
      duration: time::Duration,
      station_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
    #[ts(
      export,
      export_to = "../../../defs/api/stations/[station]/stream-stats/last-[num][unit]/GET/"
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
        let station_id = req.param("station").unwrap();

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
        let station = access_token_scope.grant_station_scope(station_id).await?;

        Ok(Input {
          duration,
          station_id: station.id,
        })
      }

      async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        let Input {
          duration,
          station_id,
        } = input;
        let stats = self
          .index
          .get_stats_item(StationQuery::one(station_id), SinceFilter::new(duration))
          .await;

        Ok(Output { stats })
      }
    }
  }

  pub mod count {
    use super::*;
    pub mod get {
      use super::*;
      use crate::error::ApiError;
      use db::stream_connection::index::{SinceFilter, StationQuery};
      use schemars::JsonSchema;

      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {
        duration: time::Duration,
        station_id: String,
      }

      #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
      #[ts(
        export,
        export_to = "../../../defs/api/stations/[station]/stream-stats/last-[num][unit]/count/GET/"
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
          let station_id = req.param("station").unwrap();

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
          let station = access_token_scope.grant_station_scope(station_id).await?;

          Ok(Input {
            duration,
            station_id: station.id,
          })
        }

        async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          let Input {
            duration,
            station_id,
          } = input;
          let total = self
            .index
            .count(StationQuery::one(station_id), SinceFilter::new(duration))
            .await;

          Ok(Output { total })
        }
      }
    }
  }
}

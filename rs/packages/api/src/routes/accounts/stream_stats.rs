use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::Station;
use db::stream_connection::stats::Stats;
// use db::stream_connection::StreamConnection;
use db::Model;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use db::stream_connection::index::{AllFilter, MemIndex, StationQuery};
  use schemars::JsonSchema;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub index: MemIndex,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    pub account_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/accounts/[account]/stream-stats/GET/"
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
      let account_id = req.param("account").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let account = access_token_scope.grant_account_scope(account_id).await?;
      Ok(Input {
        account_id: account.id,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Input { account_id } = input;

      let station_ids = Station::cl()
        .distinct(
          Station::KEY_ID,
          doc! { Station::KEY_ACCOUNT_ID: account_id },
          None,
        )
        .await?;

      if station_ids.is_empty() {
        return Ok(Output {
          stats: Default::default(),
        });
      }

      let mut set = std::collections::HashSet::with_capacity(station_ids.len());
      for id in station_ids {
        let id: String = mongodb::bson::from_bson(id).unwrap();
        set.insert(id);
      }

      let stats = match set.len() {
        1 => {
          let station_id = set.into_iter().next().unwrap();
          self
            .index
            .get_stats(StationQuery::one(station_id), AllFilter)
            .await
        }

        _ => {
          self
            .index
            .get_stats(StationQuery::some(set), AllFilter)
            .await
        }
      };

      Ok(Output { stats })
    }
  }
}

pub mod now {
  use super::*;
  pub mod get {
    use super::*;

    use db::stream_connection::{
      index::{IsOpenFilter, MemIndex, StationQuery},
      stats::StatsItem,
    };
    use schemars::JsonSchema;

    #[derive(Debug, Clone)]
    pub struct Endpoint {
      pub index: MemIndex,
    }

    #[derive(Debug, Clone)]
    pub struct Input {
      account_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
    #[ts(
      export,
      export_to = "../../../defs/api/accounts/[account]/stream-stats/now/GET/"
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
        let account_id = req.param("account").unwrap();
        let access_token_scope = request_ext::get_access_token_scope(&req).await?;
        let account = access_token_scope.grant_account_scope(account_id).await?;
        Ok(Input {
          account_id: account.id,
        })
      }

      async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        let Input { account_id } = input;

        let station_ids = Station::cl()
          .distinct(
            Station::KEY_ID,
            doc! { Station::KEY_ACCOUNT_ID: account_id },
            None,
          )
          .await?;

        if station_ids.is_empty() {
          return Ok(Output {
            stats: Default::default(),
          });
        }

        let mut set = std::collections::HashSet::with_capacity(station_ids.len());
        for id in station_ids {
          let id: String = mongodb::bson::from_bson(id).unwrap();
          set.insert(id);
        }

        let stats = match set.len() {
          1 => {
            let station_id = set.into_iter().next().unwrap();
            self
              .index
              .get_stats_item(StationQuery::one(station_id), IsOpenFilter(true))
              .await
          }

          _ => {
            self
              .index
              .get_stats_item(StationQuery::some(set), IsOpenFilter(true))
              .await
          }
        };

        Ok(Output { stats })
      }
    }
  }

  pub mod count {

    use super::*;
    pub mod get {
      use super::*;

      use db::stream_connection::index::{IsOpenFilter, MemIndex, StationQuery};
      use schemars::JsonSchema;

      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {
        account_id: String,
      }

      #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
      #[ts(
        export,
        export_to = "../../../defs/api/accounts/[account]/stream-stats/now/count/GET/"
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
          let account_id = req.param("account").unwrap();
          let access_token_scope = request_ext::get_access_token_scope(&req).await?;
          let account = access_token_scope.grant_account_scope(account_id).await?;
          Ok(Input {
            account_id: account.id,
          })
        }

        async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          let Input { account_id } = input;

          let station_ids = Station::cl()
            .distinct(
              Station::KEY_ID,
              doc! { Station::KEY_ACCOUNT_ID: account_id },
              None,
            )
            .await?;

          if station_ids.is_empty() {
            return Ok(Output { total: 0 });
          }

          let mut set = std::collections::HashSet::with_capacity(station_ids.len());
          for id in station_ids {
            let id: String = mongodb::bson::from_bson(id).unwrap();
            set.insert(id);
          }

          let total = match set.len() {
            1 => {
              let station_id = set.into_iter().next().unwrap();
              self
                .index
                .count(StationQuery::one(station_id), IsOpenFilter(true))
                .await
            }

            _ => {
              self
                .index
                .count(StationQuery::some(set), IsOpenFilter(true))
                .await
            }
          };

          Ok(Output { total })
        }
      }
    }
  }

  pub mod count_by_station {

    use super::*;
    pub mod get {
      use std::collections::{HashMap, HashSet};

      use super::*;

      use db::stream_connection::index::{IsOpenFilter, MemIndex};
      use schemars::JsonSchema;

      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {
        account_id: String,
      }

      #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
      #[ts(
        export,
        export_to = "../../../defs/api/accounts/[account]/stream-stats/now/count-by-station/GET/"
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
          let account_id = req.param("account").unwrap();
          let access_token_scope = request_ext::get_access_token_scope(&req).await?;
          let account = access_token_scope.grant_account_scope(account_id).await?;
          Ok(Input {
            account_id: account.id,
          })
        }

        async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          let Input { account_id } = input;

          let station_ids = Station::cl()
            .distinct(
              Station::KEY_ID,
              doc! { Station::KEY_ACCOUNT_ID: account_id },
              None,
            )
            .await?;

          if station_ids.is_empty() {
            return Ok(Output {
              by_station: HashMap::default(),
            });
          }

          let mut set = HashSet::with_capacity(station_ids.len());
          for id in station_ids {
            let id: String = mongodb::bson::from_bson(id).unwrap();
            set.insert(id);
          }

          let by_station = self.index.count_by_station(set, IsOpenFilter(true)).await;

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
      index::{MemIndex, SinceFilter, StationQuery},
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
      account_id: String,
      duration: time::Duration,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
    #[ts(
      export,
      export_to = "../../../defs/api/accounts/[account]/stream-stats/[last-unitvalue]/GET/"
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
        let account_id = req.param("account").unwrap().to_string();

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
        access_token_scope.grant_account_scope(&account_id).await?;

        Ok(Input {
          account_id,
          duration,
        })
      }

      async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        let Input {
          duration,
          account_id,
        } = input;

        let station_ids = Station::cl()
          .distinct(
            Station::KEY_ID,
            doc! { Station::KEY_ACCOUNT_ID: account_id },
            None,
          )
          .await?;

        if station_ids.is_empty() {
          return Ok(Output {
            stats: Default::default(),
          });
        }

        let mut set = std::collections::HashSet::with_capacity(station_ids.len());
        for id in station_ids {
          let id: String = mongodb::bson::from_bson(id).unwrap();
          set.insert(id);
        }

        let stats = match set.len() {
          1 => {
            let station_id = set.into_iter().next().unwrap();
            self
              .index
              .get_stats_item(StationQuery::one(station_id), SinceFilter::new(duration))
              .await
          }

          _ => {
            self
              .index
              .get_stats_item(StationQuery::some(set), SinceFilter::new(duration))
              .await
          }
        };

        Ok(Output { stats })
      }
    }
  }

  pub mod count {
    use super::*;
    pub mod get {
      use db::stream_connection::index::{MemIndex, SinceFilter, StationQuery};
      use schemars::JsonSchema;

      use crate::error::ApiError;

      use super::*;

      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {
        account_id: String,
        duration: time::Duration,
      }

      #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
      #[ts(
        export,
        export_to = "../../../defs/api/accounts/[account]/stream-stats/[last-unitvalue]/count/GET/"
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
          let account_id = req.param("account").unwrap().to_string();

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
          access_token_scope.grant_account_scope(&account_id).await?;

          Ok(Input {
            account_id,
            duration,
          })
        }

        async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          let Input {
            duration,
            account_id,
          } = input;

          let station_ids = Station::cl()
            .distinct(
              Station::KEY_ID,
              doc! { Station::KEY_ACCOUNT_ID: account_id },
              None,
            )
            .await?;

          if station_ids.is_empty() {
            return Ok(Output { total: 0 });
          }

          let mut set = std::collections::HashSet::with_capacity(station_ids.len());
          for id in station_ids {
            let id: String = mongodb::bson::from_bson(id).unwrap();
            set.insert(id);
          }

          let total = match set.len() {
            1 => {
              let station_id = set.into_iter().next().unwrap();
              self
                .index
                .count(StationQuery::one(station_id), SinceFilter::new(duration))
                .await
            }

            _ => {
              self
                .index
                .count(StationQuery::some(set), SinceFilter::new(duration))
                .await
            }
          };

          Ok(Output { total })
        }
      }
    }
  }
}

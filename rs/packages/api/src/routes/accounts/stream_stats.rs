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

  use db::stream_connection::index::{MemIndex, StationIdFilter, StationIdSetFilter};

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub index: MemIndex,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    pub account_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/accounts/[account]/stream-stats/GET/")]
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
          self.index.get_stats(StationIdFilter::new(station_id)).await
        }

        _ => self.index.get_stats(StationIdSetFilter::new(set)).await,
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
      index::{IsOpenFilter, MemIndex, StationIdFilter, StationIdSetFilter},
      stats::StatsItem,
    };

    #[derive(Debug, Clone)]
    pub struct Endpoint {
      pub index: MemIndex,
    }

    #[derive(Debug, Clone)]
    pub struct Input {
      account_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(export)]
    #[ts(export_to = "../../../defs/api/accounts/[account]/stream-stats/now/GET/")]
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
            let filter = (IsOpenFilter(true), StationIdFilter::new(station_id));
            self.index.get_stats_item(filter).await
          }

          _ => {
            let filter = (IsOpenFilter(true), StationIdSetFilter::new(set));

            self.index.get_stats_item(filter).await
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

      use db::stream_connection::index::{
        IsOpenFilter, MemIndex, StationIdFilter, StationIdSetFilter,
      };

      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {
        account_id: String,
      }

      #[derive(Debug, Clone, Serialize, Deserialize, TS)]
      #[ts(export)]
      #[ts(export_to = "../../../defs/api/accounts/[account]/stream-stats/now/count/GET/")]
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
              let filter = (IsOpenFilter(true), StationIdFilter::new(station_id));
              self.index.count(filter).await
            }

            _ => {
              let filter = (IsOpenFilter(true), StationIdSetFilter::new(set));

              self.index.count(filter).await
            }
          };

          Ok(Output { total })
        }
      }
    }
  }
}

pub mod since {
  use super::*;
  pub mod get {
    use db::stream_connection::{
      index::{MemIndex, SinceFilter, StationIdFilter, StationIdSetFilter},
      stats::StatsItem,
    };

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

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(export)]
    #[ts(export_to = "../../../defs/api/accounts/[account]/stream-stats/last-[num][unit]/GET/")]
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
            let filter = (SinceFilter::new(duration), StationIdFilter::new(station_id));
            self.index.get_stats_item(filter).await
          }

          _ => {
            let filter = (SinceFilter::new(duration), StationIdSetFilter::new(set));

            self.index.get_stats_item(filter).await
          }
        };

        Ok(Output { stats })
      }
    }
  }

  pub mod count {
    use super::*;
    pub mod get {
      use db::stream_connection::index::{
        MemIndex, SinceFilter, StationIdFilter, StationIdSetFilter,
      };

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

      #[derive(Debug, Clone, Serialize, Deserialize, TS)]
      #[ts(export)]
      #[ts(
        export_to = "../../../defs/api/accounts/[account]/stream-stats/last-[num][unit]/count/GET/"
      )]
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
              let filter = (SinceFilter::new(duration), StationIdFilter::new(station_id));
              self.index.count(filter).await
            }

            _ => {
              let filter = (SinceFilter::new(duration), StationIdSetFilter::new(set));
              self.index.count(filter).await
            }
          };

          Ok(Output { total })
        }
      }
    }
  }
}

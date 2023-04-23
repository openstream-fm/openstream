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

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub index: MemIndex,
  }

  #[derive(Debug, Clone)]
  pub struct Input {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/stream-stats/GET/")]
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
      if !access_token_scope.is_global() {
        return Err(GetAccessTokenScopeError::OutOfScope);
      };
      Ok(Input {})
    }

    async fn perform(&self, _input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      // let filter = doc! {};
      // let stats = Stats::get_for_filter(filter).await?;
      let stats = self.index.get_stats(AllFilter).await;
      Ok(Output { stats })
    }
  }
}

pub mod now {
  use super::*;
  pub mod get {
    use db::stream_connection::{index::IsOpenFilter, stats::StatsItem};

    use super::*;

    #[derive(Debug, Clone)]
    pub struct Endpoint {
      pub index: MemIndex,
    }

    #[derive(Debug, Clone)]
    pub struct Input {}

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(export)]
    #[ts(export_to = "../../../defs/api/stream-stats/now/GET/")]
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
        if !access_token_scope.is_global() {
          return Err(GetAccessTokenScopeError::OutOfScope);
        };
        Ok(Input {})
      }

      async fn perform(&self, _input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        // let filter = doc! {};
        // let stats = Stats::get_for_filter(filter).await?;
        let filter = IsOpenFilter(true);
        let stats = self.index.get_stats_item(filter).await;
        Ok(Output { stats })
      }
    }
  }

  pub mod count {
    use super::*;
    pub mod get {

      use db::stream_connection::index::IsOpenFilter;

      use super::*;
      #[derive(Debug, Clone)]
      pub struct Endpoint {
        pub index: MemIndex,
      }

      #[derive(Debug, Clone)]
      pub struct Input {}

      #[derive(Debug, Clone, Serialize, Deserialize, TS)]
      #[ts(export)]
      #[ts(export_to = "../../../defs/api/stream-stats/now/count/GET/")]
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
          if !access_token_scope.is_global() {
            return Err(GetAccessTokenScopeError::OutOfScope);
          };
          Ok(Input {})
        }

        async fn perform(&self, _input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          // let filter = doc! {};
          // let stats = Stats::get_for_filter(filter).await?;
          let filter = IsOpenFilter(true);
          let total = self.index.count(filter).await;
          Ok(Output { total })
        }
      }
    }
  }
}

pub mod since {
  use super::*;
  pub mod get {
    use db::stream_connection::{index::SinceFilter, stats::StatsItem};

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

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(export)]
    #[ts(export_to = "../../../defs/api/stream-stats/last-[num][unit]/GET/")]
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
        let num: u16 = match num_param.parse() {
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
        if !access_token_scope.is_global() {
          return Err(ParseError::Token(GetAccessTokenScopeError::OutOfScope));
        };
        Ok(Input { duration })
      }

      async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        let Input { duration } = input;
        let filter = SinceFilter::new(duration);
        let stats = self.index.get_stats_item(filter).await;
        Ok(Output { stats })
      }
    }
  }

  pub mod count {
    use super::*;
    pub mod get {
      use db::stream_connection::index::SinceFilter;

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

      #[derive(Debug, Clone, Serialize, Deserialize, TS)]
      #[ts(export)]
      #[ts(export_to = "../../../defs/api/stream-stats/last-[num][unit]/count/GET/")]
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
          let num: u16 = match num_param.parse() {
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
          if !access_token_scope.is_global() {
            return Err(ParseError::Token(GetAccessTokenScopeError::OutOfScope));
          };
          Ok(Input { duration })
        }

        async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
          let Input { duration } = input;
          let filter = SinceFilter::new(duration);
          let total = self.index.count(filter).await;
          Ok(Output { total })
        }
      }
    }
  }
}

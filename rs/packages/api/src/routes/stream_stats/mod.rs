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
}

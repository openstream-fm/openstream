use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::Station;
use db::stream_connection::stats::Stats;
use db::stream_connection::StreamConnection;
use db::Model;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    account_id: String,
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

      let filter = doc! { StreamConnection::KEY_STATION_ID: { "$in": station_ids } };
      let stats = Stats::get_for_filter(filter).await?;

      Ok(Output { stats })
    }
  }
}

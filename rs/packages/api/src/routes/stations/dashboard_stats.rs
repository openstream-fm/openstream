use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::Station;
use db::stream_connection::StreamConnection;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(
    export,
    export_to = "../../defs/api/stations/[station]/dashboard-stats/GET/"
  )]
  pub struct Output {
    pub sessions_24h: u64,
    pub sessions_7d: u64,
    pub sessions_30d: u64,
    pub listeners_24h: u64,
    pub listeners_7d: u64,
    pub listeners_30d: u64,
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
      Ok(Self::Input { station })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { station } = input;

      let (s24h, s7d, s30d, l24h, l7d, l30d) = tokio::try_join!(
        StreamConnection::count_for_station_in_last(&station.id, time::Duration::HOUR * 24),
        StreamConnection::count_for_station_in_last(&station.id, time::Duration::DAY * 7),
        StreamConnection::count_for_station_in_last(&station.id, time::Duration::DAY * 30),
        StreamConnection::count_unique_ips_for_station_in_last(
          &station.id,
          time::Duration::HOUR * 24
        ),
        StreamConnection::count_unique_ips_for_station_in_last(
          &station.id,
          time::Duration::DAY * 7
        ),
        StreamConnection::count_unique_ips_for_station_in_last(
          &station.id,
          time::Duration::DAY * 30
        ),
      )?;

      let out = Output {
        sessions_24h: s24h,
        sessions_7d: s7d,
        sessions_30d: s30d,
        listeners_24h: l24h,
        listeners_7d: l7d,
        listeners_30d: l30d,
      };

      Ok(out)
    }
  }
}

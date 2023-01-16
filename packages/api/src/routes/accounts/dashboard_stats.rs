use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::account::Account;
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
    account: Account,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(
    export,
    export_to = "../../defs/api/accounts/[account]/dashboard-stats/GET/"
  )]
  pub struct Output {
    pub listeners_last_24h: u64,
    pub listeners_last_7d: u64,
    pub listeners_last_30d: u64,
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
      Ok(Self::Input { account })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { account } = input;

      let (l24h, l7d, l30d) = tokio::try_join!(
        StreamConnection::count_for_account_in_last(&account.id, time::Duration::HOUR * 24),
        StreamConnection::count_for_account_in_last(&account.id, time::Duration::DAY * 7),
        StreamConnection::count_for_account_in_last(&account.id, time::Duration::DAY * 30),
      )?;

      let out = Output {
        listeners_last_24h: l24h,
        listeners_last_7d: l7d,
        listeners_last_30d: l30d,
      };

      Ok(out)
    }
  }
}

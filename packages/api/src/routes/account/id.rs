use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::account::Account;
use db::account::PublicAccount;
use prex::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Endpoint {}

#[derive(Debug, Clone)]
pub struct Input {
  account: Account,
  access_token_scope: AccessTokenScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
  account: PublicAccount,
}

#[async_trait]
impl JsonHandler for Endpoint {
  type Input = Input;
  type Output = Output;
  type ParseError = GetAccessTokenScopeError;
  type HandleError = !;

  async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
    let account_id = req.param("account").unwrap();

    let access_token_scope = request_ext::get_access_token_scope(&req).await?;

    let account = access_token_scope.grant_scope(account_id).await?;

    Ok(Self::Input {
      access_token_scope,
      account,
    })
  }

  async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
    let Self::Input {
      access_token_scope,
      account,
    } = input;

    let account = account.into_public(access_token_scope.is_admin());

    Ok(Output { account })
  }
}

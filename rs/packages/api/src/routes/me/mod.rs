pub mod devices;

use crate::{
  json::JsonHandler,
  me::Me,
  request_ext::{get_access_token_scope, AccessTokenScope, GetAccessTokenScopeError},
};
use async_trait::async_trait;
use prex::Request;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use ts_rs::TS;

pub mod get {
  use super::*;

  #[derive(Debug)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/me/GET/")]
  pub struct Output(pub Me);

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = Infallible;

    async fn parse(&self, request: Request) -> Result<Input, GetAccessTokenScopeError> {
      let access_token_scope = get_access_token_scope(&request).await?;
      Ok(Input { access_token_scope })
    }

    async fn perform(&self, input: Input) -> Result<Output, Infallible> {
      let Input { access_token_scope } = input;

      let me = match access_token_scope {
        AccessTokenScope::Global => Me::Global,
        AccessTokenScope::Admin(admin) => Me::Admin {
          admin: admin.into_public(),
        },
        AccessTokenScope::User(user) => Me::User {
          user: user.into_public(db::PublicScope::User),
        },
      };

      Ok(Output(me))
    }
  }
}

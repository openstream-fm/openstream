use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::user::{PublicUser, User};
use db::PublicScope;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod get {

  use super::*;
  use std::convert::Infallible;
  use ts_rs::TS;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    user: User,
    public_scope: PublicScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/users/[user]/GET/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    user: PublicUser,
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = Infallible;

    async fn parse(&self, req: Request) -> Result<Input, GetAccessTokenScopeError> {
      let user_id = req.param("user").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let user = access_token_scope.grant_user_scope(user_id).await?;
      let public_scope = access_token_scope.as_public_scope();
      Ok(Self::Input { user, public_scope })
    }

    async fn perform(&self, input: Input) -> Result<Output, Infallible> {
      let Self::Input { user, public_scope } = input;
      Ok(Output {
        user: user.into_public(public_scope),
      })
    }
  }
}

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::account::Account;
use db::account::PublicAccount;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use std::convert::Infallible;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    account: Account,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/[account]/GET/")]
  #[serde(rename_all = "camelCase")]
  pub struct Output {
    account: PublicAccount,
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = Infallible;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let account_id = req.param("account").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let account = access_token_scope.grant_account_scope(account_id).await?;

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

      let account = account.into_public(access_token_scope.as_public_scope());

      Ok(Output { account })
    }
  }
}

pub mod patch {

  use crate::error::{ApiError, Kind};

  use super::*;
  use db::{
    account::AccountPatch, error::ApplyPatchError, fetch_and_patch, run_transaction, Model,
  };
  use prex::request::ReadBodyJsonError;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/accounts/[account]/PATCH/")]
  pub struct Payload(pub AccountPatch);

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
    access_token_scope: AccessTokenScope,
    account: Account,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/accounts/[account]/PATCH/")]
  pub struct Output(pub PublicAccount);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => Self::from(e),
        ParseError::Payload(e) => Self::from(e),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("apply patch: {0}")]
    Patch(#[from] ApplyPatchError),
    #[error("account not found: {0}")]
    AccountNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => Self::from(e),
        HandleError::Patch(e) => Self::from(e),
        HandleError::AccountNotFound(id) => Self::from(Kind::AccountNotFound(id)),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Self::Input, Self::ParseError> {
      let account_id = req.param("account").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let account = access_token_scope.grant_account_scope(account_id).await?;

      let payload: Payload = req.read_body_json(100_000).await?;

      Ok(Self::Input {
        payload,
        access_token_scope,
        account,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        payload: Payload(payload),
        access_token_scope,
        account,
      } = input;

      let id = account.id;

      let account = run_transaction!(session => {
        fetch_and_patch!(Account, account, &id, Err(HandleError::AccountNotFound(id)), session, {
          account.apply_patch(payload, access_token_scope.as_public_scope())?;
        })
      });
      /*
      let account = run_transaction!(session => {

        let mut account = match Account::get_by_id_with_session(&account.id, &mut session).await? {
          Some(account) => account,
          None => return Err(HandleError::AccountNotFound(account.id)),
        };

        match access_token_scope {
          AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
            account.apply_admin_patch(payload)?;
          }

          AccessTokenScope::User(_) => {
            account.apply_user_patch(payload)?;
          }
        }

        Account::replace_with_session(&account.id, &account, &mut session).await?;

        account
      });
      */

      let out = account.into_public(access_token_scope.as_public_scope());

      Ok(Output(out))
    }
  }
}

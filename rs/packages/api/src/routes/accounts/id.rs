use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::account::{Account, PublicAccount};
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use db::{
    current_filter_doc,
    user_account_relation::{UserAccountRelation, UserAccountRelationKind},
    Model,
  };

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
  #[ts(export_to = "../../../defs/api/accounts/[account]/GET/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    pub is_owner: bool,
    pub account: PublicAccount,
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

      let is_owner = match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => true,
        AccessTokenScope::User(user) => {
          let filter = current_filter_doc! {
            UserAccountRelation::KEY_USER_ID: &user.id,
            UserAccountRelation::KEY_ACCOUNT_ID: &account.id,
            UserAccountRelation::KEY_KIND: UserAccountRelationKind::KEY_ENUM_VARIANT_OWNER,
          };

          UserAccountRelation::exists(filter).await?
        }
      };

      let account = account.into_public(access_token_scope.as_public_scope());

      Ok(Output { account, is_owner })
    }
  }
}

pub mod patch {

  use crate::error::ApiError;

  use super::*;
  use db::{
    account::{Account, AccountPatch, PublicAccount},
    plan::Plan,
    run_transaction, Model,
  };
  use prex::request::ReadBodyJsonError;
  use validify::{ValidationErrors, Validify};

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/accounts/[account]/PATCH/")]
  pub struct Payload(pub AccountPatch);

  #[derive(Debug, Clone)]
  pub struct Input {
    account_id: String,
    payload: Payload,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/accounts/[account]/PATCH/")]
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
    #[error("mongodb: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("mongodb: {0}")]
    Validate(#[from] ValidationErrors),
    #[error("plan not found: {0}")]
    PlanNotFound(String),
    #[error("station not found: {0}")]
    AccountNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => Self::from(e),
        HandleError::Token(e) => Self::from(e),
        HandleError::AccountNotFound(id) => Self::StationNotFound(id),
        HandleError::PlanNotFound(id) => {
          Self::BadRequestCustom(format!("Plan with {} not found", id))
        }
        HandleError::Validate(e) => ApiError::BadRequestCustom(format!("{e}")),
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
      let account_id = req.param("account").unwrap().to_string();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let payload: Payload = req.read_body_json(100_000).await?;

      Ok(Self::Input {
        account_id,
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        payload: Payload(payload),
        access_token_scope,
        account_id,
      } = input;

      let payload: AccountPatch = AccountPatch::validify(payload.into())?;

      let account = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          run_transaction!(session => {
            let mut account = match tx_try!(Account::get_by_id_with_session(&account_id, &mut session).await) {
              None => return Err(HandleError::AccountNotFound(account_id)),
              Some(account) => account,
            };

            if let Some(ref name) = payload.name {
              account.name = name.clone();
            }

            if let Some(ref user_metadata) = payload.user_metadata {
              account.user_metadata.merge(user_metadata.clone());
            }

            if let Some(ref system_metadata) = payload.system_metadata {
              account.system_metadata.merge(system_metadata.clone());
            }

            if let Some(ref plan_id) = payload.plan_id {
              let plan = match tx_try!(Plan::get_by_id(plan_id).await) {
                None => return Err(HandleError::PlanNotFound(plan_id.clone())),
                Some(plan_id) => plan_id,
              };

              if plan.deleted_at.is_some() {
                return Err(HandleError::PlanNotFound(plan_id.clone()));
              }

              account.plan_id = plan.id.clone();

              account.limits.stations.total = plan.limits.stations;
              account.limits.listeners.total = plan.limits.listeners;
              account.limits.storage.total = plan.limits.storage;
              account.limits.transfer.total = plan.limits.transfer;
            }

            tx_try!(Account::replace_with_session(&account.id, &account, &mut session).await);

            account
          })
        }

        AccessTokenScope::User(_) => {
          access_token_scope.grant_account_scope(&account_id).await?;
          run_transaction!(session => {
            let mut account = match tx_try!(Account::get_by_id_with_session(&account_id, &mut session).await) {
              None => return Err(HandleError::AccountNotFound(account_id)),
              Some(account) => account,
            };

            if let Some(ref name) = payload.name {
              account.name = name.clone();
            }

            if let Some(ref user_metadata) = payload.user_metadata {
              account.user_metadata.merge(user_metadata.clone());
            }

            if let Some(ref plan_id) = payload.plan_id {
              let plan = match tx_try!(Plan::get_by_id(plan_id).await) {
                None => return Err(HandleError::PlanNotFound(plan_id.clone())),
                Some(plan_id) => plan_id,
              };

              if plan.deleted_at.is_some() || !plan.is_user_selectable {
                return Err(HandleError::PlanNotFound(plan_id.clone()));
              }

              account.plan_id = plan.id.clone();
              account.limits.stations.total = plan.limits.stations;
              account.limits.listeners.total = plan.limits.listeners;
              account.limits.storage.total = plan.limits.storage;
              account.limits.transfer.total = plan.limits.transfer;
            }

            tx_try!(Account::replace_with_session(&account.id, &account, &mut session).await);

            account
          })
        }
      };

      let out = account.into_public(access_token_scope.as_public_scope());

      Ok(Output(out))
    }
  }
}

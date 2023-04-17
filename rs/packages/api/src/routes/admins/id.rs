use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use db::admin::{Admin, PublicAdmin};
  use std::convert::Infallible;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    admin: Admin,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/admins/[admin]/GET/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    admin: PublicAdmin,
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = Infallible;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let admin_id = req.param("admin").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let admin = access_token_scope.grant_admin_read_scope(admin_id).await?;

      Ok(Self::Input { admin })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { admin } = input;

      Ok(Output {
        admin: admin.into_public(),
      })
    }
  }
}

pub mod patch {

  use crate::error::ApiError;

  use super::*;
  use db::{
    admin::{Admin, AdminPatch, PublicAdmin},
    error::ApplyPatchError,
    fetch_and_patch, run_transaction, Model,
  };
  use prex::request::ReadBodyJsonError;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/admins/[admin]/PATCH/")]
  pub struct Payload(pub AdminPatch);

  #[derive(Debug, Clone)]
  pub struct Input {
    payload: Payload,
    admin: Admin,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/admins/[admin]/PATCH/")]
  pub struct Output(pub PublicAdmin);

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
    #[error("admin not found: {0}")]
    AdminNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => Self::from(e),
        HandleError::Patch(e) => Self::from(e),
        HandleError::AdminNotFound(id) => Self::StationNotFound(id),
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
      let admin_id = req.param("admin").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let admin = access_token_scope.grant_admin_write_scope(admin_id).await?;

      let payload: Payload = req.read_body_json(100_000).await?;

      Ok(Self::Input { payload, admin })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        payload: Payload(payload),
        admin,
      } = input;

      let id = admin.id;

      let admin = run_transaction!(session => {
        fetch_and_patch!(Admin, admin, &id, Err(HandleError::AdminNotFound(id)), session, {
          admin.apply_patch(payload.clone())?
        })
      });

      let out = admin.into_public();

      Ok(Output(out))
    }
  }
}

use crate::error::ApiError;
use crate::request_ext::get_access_token_scope;
use crate::request_ext::AccessTokenScope;
use crate::{json::JsonHandler, request_ext::GetAccessTokenScopeError};
use async_trait::async_trait;
use db::payment_method::{PaymentMethod, PublicPaymentMethod};
use db::Model;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use super::*;

  pub struct Input {
    pub payment_method_id: String,
    pub access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("payment method not found: {0}")]
    NotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::NotFound(id) => ApiError::PaymentMethodNotFound(id),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(
    export,
    export_to = "../../../defs/api/payment-methods/[payment-method]/GET/"
  )]
  pub struct Output {
    pub payment_method: PublicPaymentMethod,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Input, ParseError> {
      let payment_method_id = req.param("payment_method").unwrap().to_string();
      let access_token_scope = get_access_token_scope(&req).await?;
      Ok(Input {
        payment_method_id,
        access_token_scope,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        payment_method_id,
        access_token_scope,
      } = input;

      let filter = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          doc! {PaymentMethod::KEY_USER_ID: &payment_method_id }
        }

        AccessTokenScope::User(user) => doc! {
          PaymentMethod::KEY_ID: &payment_method_id,
          PaymentMethod::KEY_USER_ID: user.id
        },
      };

      let payment_method = match PaymentMethod::get(filter).await? {
        None => return Err(HandleError::NotFound(payment_method_id)),
        Some(doc) => doc,
      };

      Ok(Output {
        payment_method: PublicPaymentMethod::from(payment_method),
      })
    }
  }
}

use crate::error::ApiError;
use crate::request_ext::get_access_token_scope;
use crate::request_ext::AccessTokenScope;
use crate::{json::JsonHandler, request_ext::GetAccessTokenScopeError};
use async_trait::async_trait;
use db::payment_method::PaymentMethodKind;
use db::payment_method::{PaymentMethod, PublicPaymentMethod};
use db::user::User;
use db::Model;
use mongodb::bson::doc;
use payments::PaymentsClient;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

pub mod id;

pub mod get {
  use db::Paged;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
  #[ts(export, export_to = "../../../defs/api/payment-methods/GET/")]
  pub struct Query {
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u64>,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
  }

  pub struct Input {
    pub access_token_scope: AccessTokenScope,
    pub query: Query,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("query: {0}")]
    Query(#[from] serde_qs::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
        ParseError::Query(e) => e.into(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/payment-methods/GET/")]
  pub struct Output(Paged<PublicPaymentMethod>);

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Input, ParseError> {
      let access_token_scope = get_access_token_scope(&req).await?;
      let query = req.qs()?;
      Ok(Input {
        access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        access_token_scope,
        query,
      } = input;

      let Query {
        user_id: query_user_id,
        skip,
        limit,
      } = query;

      let skip = skip.unwrap_or(0);
      let limit = limit.unwrap_or(60);

      let filter = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => match query_user_id {
          None => doc! {},
          Some(id) => doc! { PaymentMethod::KEY_USER_ID: id },
        },

        AccessTokenScope::User(user) => doc! { PaymentMethod::KEY_USER_ID: user.id },
      };

      let sort = doc! { PaymentMethod::KEY_CREATED_AT: 1 };

      let paged = PaymentMethod::paged(filter, sort, skip, limit as i64)
        .await?
        .map(PublicPaymentMethod::from);

      Ok(Output(paged))
    }
  }
}

pub mod post {
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
  #[ts(export, export_to = "../../../defs/api/payment-methods/POST/")]
  pub struct Payload {
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub nonce: String,
    pub device_data: String,
  }

  pub struct Input {
    pub access_token_scope: AccessTokenScope,
    pub payload: Payload,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("payments_ensure_method: {0}")]
    PaymentsEnsureCustomer(#[source] payments::error::PerformError),
    #[error("payments_save_payment_method: {0}")]
    PaymentSavePaymentMethod(#[source] payments::error::PerformError),
    #[error("user not found: {0}")]
    PayloadUserNotFound(String),
    #[error("payload user_id required")]
    PayloadUserRequired,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::PaymentSavePaymentMethod(e) => e.into(),
        HandleError::PaymentsEnsureCustomer(e) => e.into(),
        HandleError::PayloadUserNotFound(id) => {
          ApiError::PayloadInvalid(format!("user with id {} not found", id))
        }
        HandleError::PayloadUserRequired => {
          ApiError::PayloadInvalid("`user_id` is required for admin and global scope".into())
        }
      }
    }
  }

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
        ParseError::Payload(e) => e.into(),
        ParseError::Token(e) => e.into(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/payment-methods/POST/")]
  pub struct Output {
    pub payment_method: PublicPaymentMethod,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub payments_client: PaymentsClient,
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let access_token_scope = get_access_token_scope(&req).await?;
      let payload = req.read_body_json(50_000).await?;
      Ok(Input {
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        access_token_scope,
        payload,
      } = input;

      let Payload {
        user_id: admin_access_user_id,
        nonce,
        device_data,
      } = payload;

      let user = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => match admin_access_user_id {
          None => return Err(HandleError::PayloadUserRequired),
          Some(id) => match User::get_by_id(&id).await? {
            None => return Err(HandleError::PayloadUserNotFound(id)),
            Some(user) => user,
          },
        },

        AccessTokenScope::User(user) => user,
      };

      let customer_id = {
        let query = payments::query::ensure_customer::EnsureCustomer {
          customer_id: user.id.clone(),
          first_name: user.first_name.clone(),
          last_name: user.last_name.clone(),
          email: user.email.clone(),
        };

        let res = self
          .payments_client
          .perform(query)
          .await
          .map_err(HandleError::PaymentsEnsureCustomer)?;

        res.customer_id
      };

      let payment_method_response = {
        let query = payments::query::save_payment_method::SavePaymentMethod {
          customer_id,
          payment_method_nonce: nonce,
          device_data: Some(device_data),
        };

        let payment_method_response = self
          .payments_client
          .perform(query)
          .await
          .map_err(HandleError::PaymentSavePaymentMethod)?;

        payment_method_response
      };

      let now = DateTime::now();

      let payment_method = PaymentMethod {
        id: PaymentMethod::uid(),
        user_id: user.id.clone(),
        kind: PaymentMethodKind::Card {
          token: payment_method_response.payment_method_token,
          card_type: payment_method_response.card_type,
          last_4: payment_method_response.last_4,
          expiration_month: payment_method_response.expiration_month,
          expiration_year: payment_method_response.expiration_year,
        },
        created_at: now,
        updated_at: now,
        deleted_at: None,
      };

      PaymentMethod::insert(&payment_method).await?;

      let out = Output {
        payment_method: payment_method.into(),
      };

      Ok(out)
    }
  }
}

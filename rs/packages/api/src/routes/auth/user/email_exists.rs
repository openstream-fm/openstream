pub mod get {
  use async_trait::async_trait;
  use db::user::User;
  use mongodb::bson::doc;
  use prex::Request;
  use serde::{Deserialize, Serialize};
  use std::convert::Infallible;
  use std::net::IpAddr;
  use ts_rs::TS;

  use crate::error::ApiError;
  use crate::ip_limit::{hit, should_reject};
  use crate::json::JsonHandler;

  #[derive(Debug, Clone)]
  pub struct Input {
    email: String,
    ip: IpAddr,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(
    export,
    export_to = "../../../defs/api/auth/user/email-exists/[email]/GET/"
  )]
  pub struct Output {
    exists: bool,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("too many requests")]
    TooManyRequests,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::TooManyRequests => ApiError::TooManyRequests,
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type ParseError = Infallible;
    type HandleError = HandleError;
    type Input = Input;
    type Output = Output;

    async fn parse(&self, req: Request) -> Result<Input, Infallible> {
      let email = req.param("email").unwrap().to_string();
      let ip = req.isomorphic_ip();
      Ok(Input { email, ip })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input { email, ip } = input;

      let email = email.trim().to_lowercase();

      if should_reject(ip) {
        return Err(HandleError::TooManyRequests);
      }

      hit(ip);

      let exists = User::email_exists(&email).await?;

      Ok(Output { exists })
    }
  }
}

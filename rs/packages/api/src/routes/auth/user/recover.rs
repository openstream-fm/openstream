pub mod post {
  use async_trait::async_trait;
  use db::user::User;
  use log::warn;
  use mongodb::bson::doc;
  use prex::{request::ReadBodyJsonError, Request};
  use serde::{Deserialize, Serialize};
  use serde_util::empty_struct::EmptyStruct;
  use std::net::IpAddr;
  use std::time::Duration;
  use ts_rs::TS;

  use crate::error::ApiError;
  use crate::ip_limit::{hit, should_reject};
  use crate::json::JsonHandler;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/auth/user/recover/POST/")]
  // #[serde(rename_all = "camelCase")]
  #[serde(deny_unknown_fields)]
  pub struct Payload {
    email: String,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    ip: IpAddr,
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/auth/user/recover/POST/")]
  pub struct Output(EmptyStruct);

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("too many requests")]
    TooManyRequests,
    #[error("device id invalid")]
    DeviceIdInvalid,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::TooManyRequests => ApiError::TooManyRequests,
        HandleError::DeviceIdInvalid => ApiError::PayloadInvalid("device_id is invalid".into()),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type ParseError = ReadBodyJsonError;
    type HandleError = HandleError;
    type Input = Input;
    type Output = Output;

    async fn parse(&self, mut req: Request) -> Result<Input, Self::ParseError> {
      let payload: Payload = req.read_body_json(1_000).await?;
      let ip = req.isomorphic_ip();
      Ok(Input { ip, payload })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Input { ip, payload } = input;

      let Payload { email } = payload;

      if should_reject(ip) {
        warn!(
          target: "api",
          "user recover intent from ip {ip} to address {email} (ip limited)"
        );
        return Err(HandleError::TooManyRequests);
      }

      hit(ip);

      let email = email.trim().to_lowercase();

      let _user = match User::find_by_email(&email).await? {
        // if we return an error here, we will be showing the
        // users addresses we have in the database to a possible attacker
        None => {
          warn!(
            target: "api",
            "user recover intent from ip {ip} to address {email} (address not found)"
          );
          // approximate time to correctly handle this request
          let ms = 1.5 + (rand::random::<f64>() * 1.5);
          tokio::time::sleep(Duration::from_secs_f64(ms)).await;
          return Ok(Output(EmptyStruct(())));
        }
        Some(user) => user,
      };

      let out = Output(EmptyStruct(()));

      Ok(out)
    }
  }
}

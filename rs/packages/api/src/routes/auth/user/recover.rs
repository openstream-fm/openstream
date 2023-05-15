pub mod post {
  use async_trait::async_trait;
  use db::sent_email::{SentEmail, SentEmailAddress, SentEmailKind};
  use db::token_user_recovery::TokenUserRecovery;
  use db::user::User;
  use db::Model;
  use log::warn;
  use mailer::error::RenderError;
  use mailer::send::{Address, Email, Mailer, SendError};
  use mongodb::bson::doc;
  use prex::{request::ReadBodyJsonError, Request};
  use serde::{Deserialize, Serialize};
  use serde_util::empty_struct::EmptyStruct;
  use serde_util::DateTime;
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
  pub struct Endpoint {
    pub mailer: Mailer,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("too many requests")]
    TooManyRequests,
    #[error("mailer render: {0}")]
    MailerRender(#[from] RenderError),
    #[error("mailer send: {0}")]
    MailerSend(#[from] SendError),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::TooManyRequests => ApiError::TooManyRequests,
        HandleError::MailerRender(e) => e.into(),
        HandleError::MailerSend(e) => e.into(),
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

      let user = match User::find_by_email(&email).await? {
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

      let recovery_token_id = TokenUserRecovery::uid();
      let recovery_token_key = TokenUserRecovery::random_key();
      let user_recovery_token = TokenUserRecovery {
        id: recovery_token_id.clone(),
        hash: crypt::sha256(&recovery_token_key),
        user_id: user.id.clone(),
        created_at: DateTime::now(),
        used_at: None,
      };

      TokenUserRecovery::insert(user_recovery_token).await?;

      let recovery_url = format!(
        "https://studio.openstream.fm/user-recovery/{}-{}",
        recovery_token_id, recovery_token_key
      );

      let template = mailer::templates::UserRecovery {
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        recovery_url: recovery_url.clone(),
      };

      let render = mailer::render::render(template)?;

      let from_name = String::from("Openstrema");
      let to_name = format!("{} {}", user.first_name, user.last_name);
      let subject = String::from("Recover your account at Openstream");

      let sent_email = SentEmail {
        id: SentEmail::uid(),

        from: SentEmailAddress {
          name: Some(from_name.clone()),
          email: self.mailer.username.clone(),
        },

        to: SentEmailAddress {
          name: Some(to_name.clone()),
          email: user.email.clone(),
        },

        reply_to: None,

        subject: subject.clone(),

        text: render.storable.text,
        html: render.storable.html,

        kind: SentEmailKind::UserRecovery {
          user_id: user.id.clone(),
          token_id: recovery_token_id,
        },

        created_at: DateTime::now(),
      };

      SentEmail::insert(sent_email).await?;

      let email = Email {
        from: Address {
          name: Some(from_name),
          email: self.mailer.username.clone(),
        },
        to: Address {
          name: Some(to_name),
          email: user.email,
        },
        html: render.sendable.html,
        text: render.sendable.text,
        subject,
      };

      self.mailer.send(email).await?;

      let out = Output(EmptyStruct(()));

      Ok(out)
    }
  }
}

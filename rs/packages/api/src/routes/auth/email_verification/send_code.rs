pub mod post {
  use async_trait::async_trait;
  use db::email_verification_code::EmailVerificationCode;
  use db::sent_email::{SentEmail, SentEmailAddress, SentEmailKind};
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
  use ts_rs::TS;
  use validate::email::is_valid_email;

  use crate::error::ApiError;
  use crate::ip_limit::{hit, should_reject};
  use crate::json::JsonHandler;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(
    export,
    export_to = "../../../defs/api/auth/email-verification/send-code/POST/"
  )]
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
  #[ts(
    export,
    export_to = "../../../defs/api/auth/email-verification/send-code/POST/"
  )]
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
    #[error("email invalid")]
    EmailInvalid(String),
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
        HandleError::EmailInvalid(_) => ApiError::BadRequestCustom("Email is invalid".into()),
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
          "email verification code send intent from ip {ip} to address {email} (ip limited)"
        );
        return Err(HandleError::TooManyRequests);
      }

      hit(ip);

      let email = email.trim().to_lowercase();

      if !is_valid_email(&email) {
        return Err(HandleError::EmailInvalid(email));
      }

      let code_id = EmailVerificationCode::uid();
      let code = EmailVerificationCode::random_code();
      let hash = crypt::sha256(&code);

      let document = EmailVerificationCode {
        id: code_id.clone(),
        email: email.clone(),
        hash,
        used_at: None,
        created_at: DateTime::now(),
      };

      EmailVerificationCode::insert(document).await?;

      let template = mailer::templates::EmailVerification { code };

      let render = mailer::render::render(template)?;

      let from_name = String::from("Openstream");
      let subject = String::from("Your email verification code");

      let sent_email = SentEmail {
        id: SentEmail::uid(),

        from: SentEmailAddress {
          name: Some(from_name.clone()),
          email: self.mailer.username.clone(),
        },

        to: SentEmailAddress {
          name: None,
          email: email.clone(),
        },

        reply_to: None,

        subject: subject.clone(),

        text: render.storable.text,
        html: render.storable.html,

        kind: SentEmailKind::EmailVerificationCode {
          email: email.clone(),
          code_id,
        },

        created_at: DateTime::now(),
      };

      SentEmail::insert(sent_email).await?;

      let sendable_email = Email {
        from: Address {
          name: Some(from_name),
          email: self.mailer.username.clone(),
        },
        to: Address { name: None, email },
        html: render.sendable.html,
        text: render.sendable.text,
        subject,
      };

      self.mailer.send(sendable_email).await?;

      let out = Output(EmptyStruct(()));

      Ok(out)
    }
  }
}

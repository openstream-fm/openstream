use serde::{Deserialize, Serialize};

use lettre::{
  message::{Mailbox, Message, MultiPart},
  transport::smtp::authentication::Credentials,
  AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};

#[derive(Debug, Clone)]
pub struct Email {
  pub from: Address,
  pub to: Address,
  pub subject: String,
  pub html: String,
  pub text: String,
}

#[derive(Debug, Clone)]
pub struct Address {
  pub name: Option<String>,
  pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mailer {
  pub hostname: String,
  pub port: u16,
  pub username: String,
  pub password: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
  #[error("lettre: {0}")]
  Lettre(#[from] lettre::error::Error),
  #[error("address: {0}")]
  Address(#[from] lettre::address::AddressError),
  #[error("transport: {0}")]
  Transport(#[from] lettre::transport::smtp::Error),
}

impl Mailer {
  pub async fn send(&self, email: Email) -> Result<(), SendError> {
    let from = Mailbox::new(email.from.name, email.from.email.parse()?);
    let to = Mailbox::new(email.to.name, email.to.email.parse()?);

    let message = Message::builder()
      .from(from)
      .to(to)
      .subject(&email.subject)
      .multipart(MultiPart::alternative_plain_html(email.text, email.html))?;

    let creds = Credentials::new(self.username.clone(), self.password.clone());

    let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.hostname)?
      .port(self.port)
      .credentials(creds)
      .build::<Tokio1Executor>();

    transport.send(message).await?;

    Ok(())
  }
}

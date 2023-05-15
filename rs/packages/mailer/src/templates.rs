use crate::{redactable::Redactable, sample::Sample};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "account-invitation.html")]
pub struct AccountInvitation {
  account_name: String,
  sender_first_name: String,
  sender_last_name: String,
  invitation_url: String,
}

impl Redactable for AccountInvitation {
  fn into_redacted(self) -> Self {
    Self {
      account_name: self.account_name,
      sender_first_name: self.sender_first_name,
      sender_last_name: self.sender_last_name,
      invitation_url: String::from("https://studio.openstream.fm/invitations/:redacted"),
    }
  }
}

impl Sample for AccountInvitation {
  fn sample() -> Self {
    Self {
      account_name: String::from("Account"),
      sender_first_name: String::from("Name"),
      sender_last_name: String::from("Lastname"),
      invitation_url: String::from("https://studio.openstream.fm/invitations/:token"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "user-recovery.html")]
pub struct UserRecovery {
  pub first_name: String,
  pub last_name: String,
  pub recovery_url: String,
}

impl Redactable for UserRecovery {
  fn into_redacted(self) -> Self {
    Self {
      first_name: self.first_name,
      last_name: self.last_name,
      recovery_url: String::from("https://studio.openstream.fm/user-recovery/:redacted"),
    }
  }
}

impl Sample for UserRecovery {
  fn sample() -> Self {
    Self {
      first_name: String::from("Name"),
      last_name: String::from("Lastname"),
      recovery_url: String::from("https://studio.openstream.fm/user-recovery/:token"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "email-validation.html")]
pub struct EmailValidation {
  first_name: String,
  last_name: String,
  validation_url: String,
}

impl Redactable for EmailValidation {
  fn into_redacted(self) -> Self {
    Self {
      first_name: self.first_name,
      last_name: self.last_name,
      validation_url: String::from("https://studio.openstream.fm/email-validation/:redacted"),
    }
  }
}

impl Sample for EmailValidation {
  fn sample() -> Self {
    Self {
      first_name: String::from("Name"),
      last_name: String::from("Lastname"),
      validation_url: String::from("https://studio.openstream.fm/email-validation/:token"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "no-reply-autoreply.html")]
pub struct NoReplyAutoreply {
  contact_email: String,
}

impl Redactable for NoReplyAutoreply {
  fn into_redacted(self) -> Self {
    Self {
      contact_email: self.contact_email,
    }
  }
}

impl Sample for NoReplyAutoreply {
  fn sample() -> Self {
    Self {
      contact_email: String::from("hello@openstream.fm"),
    }
  }
}

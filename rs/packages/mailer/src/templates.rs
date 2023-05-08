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

impl Default for AccountInvitation {
  fn default() -> Self {
    Self {
      account_name: String::from("Account"),
      sender_first_name: String::from("Name"),
      sender_last_name: String::from("Lastname"),
      invitation_url: String::from(
        "https://studio.openstream.fm/invitations/asdjaskdl-asklñdjaksdjkasljdkaskjaklsñdjakd",
      ),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "user-recovery.html")]
pub struct UserRecovery {
  first_name: String,
  last_name: String,
  recovery_url: String,
}

impl Default for UserRecovery {
  fn default() -> Self {
    Self {
      first_name: String::from("Name"),
      last_name: String::from("Lastname"),
      recovery_url: String::from(
        "https://studio.openstream.fm/user-recovery/asdjaskdl-kajsdlkñajsdlkjasdaskdkals",
      ),
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

impl Default for EmailValidation {
  fn default() -> Self {
    Self {
      first_name: String::from("Name"),
      last_name: String::from("Lastname"),
      validation_url: String::from(
        "https://studio.openstream.fm/email-validation/asdjaskdl-kajsdlkñajsdlkjasdaskdkals",
      ),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "no-reply-autoreply.html")]
pub struct NoReplyAutoreply {
  contact_email: String,
}

impl Default for NoReplyAutoreply {
  fn default() -> Self {
    Self {
      contact_email: String::from("hello@openstream.fm"),
    }
  }
}

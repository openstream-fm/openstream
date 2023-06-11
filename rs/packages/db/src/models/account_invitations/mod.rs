use crate::Model;
use mongodb::IndexModel;
use mongodb::{bson::doc, ClientSession};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

crate::register!(AccountInvitation);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[macros::keys]
pub struct AccountInvitation {
  #[serde(rename = "_id")]
  pub id: String,

  /// sha256 of a Self::random_key()
  pub hash: String,

  /// account.id of the invitation
  /// this is the account that the invitation grant access to
  pub account_id: String,

  /// user.id of the creator of this invitation if this was created by a user
  pub user_sender_id: Option<String>,

  /// admin.id of the creator of this invitation if this was created by a user
  pub admin_sender_id: Option<String>,

  /// email of the invitation (maybe)user target (this could be a registered user or not)
  /// that's why we don't use a user.id, because the user may not be registered yet
  pub receiver_email: String,

  #[serde(flatten)]
  pub state: AccountInvitationState,

  pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(tag = "state")]
#[macros::keys]
pub enum AccountInvitationState {
  #[serde(rename = "pending")]
  Pending,
  #[serde(rename = "accepted")]
  Accepted { used_at: DateTime },
  #[serde(rename = "rejected")]
  Rejected { used_at: DateTime },
}

impl AccountInvitation {
  const RANDOM_KEY_LEN: usize = 24;

  pub fn random_key() -> String {
    uid::uid(Self::RANDOM_KEY_LEN)
  }

  pub fn is_expired(&self) -> bool {
    let now = DateTime::now().inner();

    let until = self
      .created_at
      .saturating_add(time::Duration::SECOND * constants::EMAIL_VERIFICATION_VALIDITY_SECS);

    now > until
  }

  pub async fn get_by_token(id_key: &str) -> Result<Option<Self>, mongodb::error::Error> {
    let (id, key) = match id_key.split_once('-') {
      Some((id, key)) => (id, key),
      None => return Ok(None),
    };

    let doc = match Self::get_by_id(id).await? {
      Some(doc) => doc,
      None => return Ok(None),
    };

    let hash = crypt::sha256(key);
    if doc.hash != hash {
      return Ok(None);
    }

    Ok(Some(doc))
  }

  pub async fn get_by_token_with_session(
    id_key: &str,
    session: &mut ClientSession,
  ) -> Result<Option<Self>, mongodb::error::Error> {
    let (id, key) = match id_key.split_once('-') {
      Some((id, key)) => (id, key),
      None => return Ok(None),
    };

    let doc = match Self::get_by_id_with_session(id, session).await? {
      Some(doc) => doc,
      None => return Ok(None),
    };

    let hash = crypt::sha256(key);
    if doc.hash != hash {
      return Ok(None);
    }

    Ok(Some(doc))
  }
}

impl Model for AccountInvitation {
  const UID_LEN: usize = 12;
  const CL_NAME: &'static str = "account_invitations";

  fn indexes() -> Vec<IndexModel> {
    let user_sender_id = IndexModel::builder()
      .keys(doc! {
        AccountInvitation::KEY_USER_SENDER_ID: 1,
      })
      .build();

    let admin_sender_id = IndexModel::builder()
      .keys(doc! {
        AccountInvitation::KEY_ADMIN_SENDER_ID: 1,
      })
      .build();

    let receiver_email = IndexModel::builder()
      .keys(doc! {
        AccountInvitation::KEY_RECEIVER_EMAIL: 1,
      })
      .build();

    let state = IndexModel::builder()
      .keys(doc! {
        AccountInvitationState::KEY_ENUM_TAG: 1,
      })
      .build();

    let created_at = IndexModel::builder()
      .keys(doc! {
        AccountInvitation::KEY_CREATED_AT: 1,
      })
      .build();

    vec![
      user_sender_id,
      admin_sender_id,
      receiver_email,
      state,
      created_at,
    ]
  }
}

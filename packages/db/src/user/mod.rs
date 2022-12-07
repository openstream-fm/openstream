use crate::metadata::Metadata;
use crate::{Model, PublicScope};
use chrono::{DateTime, Utc};
use mongodb::error::Result as MongoResult;
use mongodb::ClientSession;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::datetime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(rename_all = "camelCase")]
pub struct User {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_ids: Vec<String>,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub password: Option<String>,
  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "datetime")]
  pub updated_at: DateTime<Utc>,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(rename_all = "camelCase")]
pub struct UserPublicUser {
  #[serde(rename = "_id")]
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "datetime")]
  pub updated_at: DateTime<Utc>,
  pub user_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(rename_all = "camelCase")]
pub struct AdminPublicUser {
  #[serde(rename = "_id")]
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "datetime")]
  pub updated_at: DateTime<Utc>,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(untagged)]
pub enum PublicUser {
  Admin(AdminPublicUser),
  User(UserPublicUser),
}

impl From<User> for AdminPublicUser {
  fn from(user: User) -> Self {
    Self {
      id: user.id,
      first_name: user.first_name,
      last_name: user.last_name,
      email: user.email,
      created_at: user.created_at,
      updated_at: user.updated_at,
      user_metadata: user.user_metadata,
      system_metadata: user.system_metadata,
    }
  }
}

impl From<User> for UserPublicUser {
  fn from(user: User) -> Self {
    Self {
      id: user.id,
      first_name: user.first_name,
      last_name: user.last_name,
      email: user.email,
      created_at: user.created_at,
      updated_at: user.updated_at,
      user_metadata: user.user_metadata,
    }
  }
}

impl User {
  pub fn into_public(self, scope: PublicScope) -> PublicUser {
    match scope {
      PublicScope::Admin => PublicUser::Admin(self.into()),
      PublicScope::User => PublicUser::User(self.into()),
    }
  }

  pub async fn find_by_email(email: &str) -> MongoResult<Option<Self>> {
    Self::cl().find_one(doc! { "email": email }, None).await
  }

  pub async fn email_exists(email: &str) -> MongoResult<bool> {
    Self::exists(doc! { "email": email }).await
  }

  pub async fn email_exists_with_session(
    email: &str,
    session: &mut ClientSession,
  ) -> MongoResult<bool> {
    Self::exists_with_session(doc! { "email": email }, session).await
  }

  pub async fn find_by_email_with_session(
    email: &str,
    session: &mut ClientSession,
  ) -> MongoResult<Option<Self>> {
    Self::cl()
      .find_one_with_session(doc! { "email": email }, None, session)
      .await
  }
}

impl Model for User {
  const UID_LEN: usize = 6;
  const CL_NAME: &'static str = "users";

  fn indexes() -> Vec<IndexModel> {
    let account_ids = IndexModel::builder().keys(doc! { "accountIds": 1 }).build();
    let email_opts = IndexOptions::builder().unique(true).build();
    let email = IndexModel::builder()
      .keys(doc! { "email": 1 })
      .options(email_opts)
      .build();

    vec![account_ids, email]
  }
}

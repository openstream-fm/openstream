use crate::metadata::Metadata;
use crate::{IntoPublicScope, Model};
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
  pub fn into_public(self, scope: IntoPublicScope) -> PublicUser {
    match scope {
      IntoPublicScope::Admin => PublicUser::Admin(self.into()),
      IntoPublicScope::User => PublicUser::User(self.into()),
    }
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

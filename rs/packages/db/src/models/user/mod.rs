use crate::metadata::Metadata;
use crate::{current_filter_doc, deleted_filter_doc, Model, PublicScope};
use modify::Modify;
use mongodb::error::Result as MongoResult;
use mongodb::ClientSession;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;
use validator::Validate;

crate::register!(User);

#[derive(Debug, Clone, Serialize, Deserialize, TS, Modify, Validate)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct User {
  #[serde(rename = "_id")]
  pub id: String,

  #[modify(trim)]
  #[validate(length(min = 1, max = 50))]
  pub first_name: String,

  #[modify(trim)]
  #[validate(length(min = 1, max = 50))]
  pub last_name: String,

  #[validate(length(min = 1, max = 50))]
  pub email: String,

  #[modify(trim)]
  #[validate(length(min = 1, max = 50))]
  pub phone: Option<String>,

  pub language: Option<String>,

  pub password: Option<String>,

  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
pub struct UserPublicUser {
  #[serde(rename = "_id")]
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub phone: Option<String>,
  pub language: Option<String>,
  pub user_metadata: Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
pub struct AdminPublicUser {
  #[serde(rename = "_id")]
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub phone: Option<String>,
  pub language: Option<String>,
  pub user_metadata: Metadata,
  pub system_metadata: Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/db/")]
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
      phone: user.phone,
      language: user.language,
      user_metadata: user.user_metadata,
      system_metadata: user.system_metadata,
      created_at: user.created_at,
      updated_at: user.updated_at,
      deleted_at: user.deleted_at,
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
      phone: user.phone,
      language: user.language,
      user_metadata: user.user_metadata,
      created_at: user.created_at,
      updated_at: user.updated_at,
      deleted_at: user.deleted_at,
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

  pub async fn find_by_email(email: &str, active: Option<bool>) -> MongoResult<Option<Self>> {
    let filter = match active {
      None => doc! { Self::KEY_EMAIL: email },
      Some(active) => {
        if active {
          current_filter_doc! { Self::KEY_EMAIL: email }
        } else {
          deleted_filter_doc! { Self::KEY_EMAIL: email }
        }
      }
    };

    Self::cl().find_one(filter, None).await
  }

  pub async fn email_exists(email: &str) -> MongoResult<bool> {
    Self::exists(doc! { Self::KEY_EMAIL: email }).await
  }

  pub async fn email_exists_with_session(
    email: &str,
    session: &mut ClientSession,
  ) -> MongoResult<bool> {
    Self::exists_with_session(doc! { Self::KEY_EMAIL: email }, session).await
  }

  pub async fn find_by_email_with_session(
    email: &str,
    active: Option<bool>,
    session: &mut ClientSession,
  ) -> MongoResult<Option<Self>> {
    let email_filter = doc! { Self::KEY_EMAIL: email };

    let filter = match active {
      None => email_filter,
      Some(true) => current_filter_doc!(email_filter),
      Some(false) => deleted_filter_doc!(email_filter),
    };

    Self::cl()
      .find_one_with_session(filter, None, session)
      .await
  }
}

impl Model for User {
  const UID_LEN: usize = 6;
  const CL_NAME: &'static str = "users";

  fn indexes() -> Vec<IndexModel> {
    let email_opts = IndexOptions::builder().unique(true).build();
    let email = IndexModel::builder()
      .keys(doc! { Self::KEY_EMAIL: 1 })
      .options(email_opts)
      .build();

    vec![email]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, User::KEY_ID);
  }
}

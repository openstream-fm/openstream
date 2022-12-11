use std::net::IpAddr;

use crate::Model;
use chrono::{DateTime, Utc};
use mongodb::bson::{self, doc};
use mongodb::options::{FindOneAndUpdateOptions, IndexOptions, ReturnDocument};
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::{as_f64, datetime};
use ts_rs::TS;
use user_agent::UserAgent;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[ts(rename = "AccessTokenScope")]
#[serde(tag = "scope", rename_all = "camelCase")]
pub enum Scope {
  Global,
  Admin { admin_id: String },
  User { user_id: String },
}

impl Scope {
  pub fn is_user(&self) -> bool {
    matches!(self, Scope::User { .. })
  }

  pub fn is_admin(&self) -> bool {
    matches!(self, Scope::Admin { .. })
  }

  pub fn is_global(&self) -> bool {
    matches!(self, Scope::Global)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[ts(rename = "AccessTokenGeneratedBy")]
#[serde(tag = "generatedBy", rename_all = "camelCase")]
pub enum GeneratedBy {
  Login {
    #[serde(with = "serde_util::ip")]
    #[ts(type = "string")]
    ip: IpAddr,
    user_agent: UserAgent,
  },
  Register {
    #[serde(with = "serde_util::ip")]
    #[ts(type = "string")]
    ip: IpAddr,
    user_agent: UserAgent,
  },
  Api {
    title: String,
  },
  Cli {
    title: String,
  },
}

impl GeneratedBy {
  pub fn is_login(&self) -> bool {
    matches!(self, Self::Login { .. })
  }

  pub fn is_generated(&self) -> bool {
    matches!(self, Self::Api { title: _ })
  }

  pub fn title(&self) -> Option<&str> {
    match self {
      Self::Login { .. } => None,
      Self::Register { .. } => None,
      Self::Api { title } => Some(title.as_ref()),
      Self::Cli { title } => Some(title.as_ref()),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[ts(rename = "BaseAccessToken")]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
  #[serde(rename = "_id")]
  pub id: String,

  pub key: String,

  #[serde(flatten)]
  #[ts(skip)]
  pub scope: Scope,

  #[serde(flatten)]
  #[ts(skip)]
  pub generated_by: GeneratedBy,

  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,

  #[serde(with = "datetime::option")]
  pub last_used_at: Option<DateTime<Utc>>,

  #[serde(with = "as_f64")]
  pub hits: u64,
}

impl AccessToken {
  pub async fn touch(key: &str) -> Result<Option<AccessToken>, mongodb::error::Error> {
    let filter = doc! { "key": key };

    let update = doc! {
      "$set": { "lastUsedAt": bson::DateTime::now() },
      "$inc": { "hits": 1 }
    };

    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::After)
      .build();

    Self::cl()
      .find_one_and_update(filter, update, options)
      .await
  }
}

impl AccessToken {
  pub fn random_key() -> String {
    uid::uid(48)
  }

  pub fn is_login(&self) -> bool {
    self.generated_by.is_login()
  }

  pub fn is_generated(&self) -> bool {
    self.generated_by.is_generated()
  }

  pub fn title(&self) -> Option<&str> {
    self.generated_by.title()
  }

  pub fn is_admin(&self) -> bool {
    self.scope.is_admin()
  }

  pub fn is_user(&self) -> bool {
    self.scope.is_user()
  }

  pub fn is_global(&self) -> bool {
    self.scope.is_global()
  }
}

impl Model for AccessToken {
  const UID_LEN: usize = 24;
  const CL_NAME: &'static str = "access_tokens";

  fn indexes() -> Vec<IndexModel> {
    let key = IndexModel::builder()
      .keys(doc! { "key": 1 })
      .options(IndexOptions::builder().unique(true).build())
      .build();
    let user_id = IndexModel::builder().keys(doc! { "userId": 1 }).build();
    let admin_id = IndexModel::builder().keys(doc! { "adminId": 1 }).build();
    let scope = IndexModel::builder().keys(doc! { "scope": 1 }).build();
    let generated_by = IndexModel::builder()
      .keys(doc! { "generatedBy": 1 })
      .build();
    vec![key, user_id, admin_id, scope, generated_by]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn serde_bson_vec() {
    // chrono has nanosecond precision that get lost on serialize and deserialize
    // so we use bson::DateTime::now().into() instead of Utc::now()
    let now = bson::DateTime::now().into();

    let token = AccessToken {
      id: AccessToken::uid(),
      key: AccessToken::random_key(),
      created_at: now,
      last_used_at: Some(now),
      generated_by: GeneratedBy::Api {
        title: String::from("Title"),
      },
      scope: Scope::Global,
      hits: 0,
    };

    let vec = bson::to_vec(&token).expect("bson serialize");

    let out = bson::from_slice(&vec).expect("bson deserialize");

    assert_eq!(token, out);
  }

  #[test]
  fn serde_bson_doc() {
    // chrono has nanosecond precision that get lost on serialize and deserialize
    // so we use bson::DateTime::now().into() instead of Utc::now()
    let now = bson::DateTime::now().into();

    let token = AccessToken {
      id: AccessToken::uid(),
      key: AccessToken::random_key(),
      created_at: now,
      last_used_at: Some(now),
      generated_by: GeneratedBy::Api {
        title: String::from("Title"),
      },
      scope: Scope::Global,
      hits: 0,
    };

    let doc = bson::to_document(&token).expect("bson serialize");

    let out = bson::from_document(doc).expect("bson deserialize");

    assert_eq!(token, out);
  }
}

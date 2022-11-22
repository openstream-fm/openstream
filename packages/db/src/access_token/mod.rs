use crate::Model;
use chrono::{DateTime, Utc};
use mongodb::bson::{self, doc};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::{as_f64, datetime};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "accessType", rename_all = "camelCase")]
pub enum Scope {
  User { user_id: String },
  Admin { admin_id: String },
  Global,
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "generatedBy", rename_all = "camelCase")]
pub enum Kind {
  #[serde(rename=""login)]
  Login { ip: String, user_agent: String },
  #[serde(rename = "generated")]
  Generated { title: String },
  #[serde(rename = "cli")]
  CliGenerated { title: String },
}

impl Kind {
  pub fn is_login(&self) -> bool {
    matches!(self, Self::Login { .. })
  }

  pub fn is_generated(&self) -> bool {
    matches!(self, Self::Generated { title: _ })
  }

  pub fn title(&self) -> Option<&str> {
    match self {
      Self::Login { .. } => None,
      Self::Generated { title } => Some(title.as_ref()),
      Self::CliGenerated { title } => Some(title.as_ref()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
  #[serde(rename = "_id")]
  pub id: String,
  #[serde(flatten)]
  pub scope: Scope,
  #[serde(flatten)]
  pub kind: Kind,
  #[serde(with = "datetime")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "datetime::option")]
  pub last_used_at: Option<DateTime<Utc>>,
  #[serde(with = "as_f64")]
  pub hits: u64,
}

impl AccessToken {
  pub async fn touch(id: &str) -> Result<Option<AccessToken>, mongodb::error::Error> {
    let filter = doc! { "_id": id };

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
  pub fn is_login(&self) -> bool {
    self.kind.is_login()
  }

  pub fn is_generated(&self) -> bool {
    self.kind.is_generated()
  }

  pub fn title(&self) -> Option<&str> {
    self.kind.title()
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
  const UID_LEN: usize = 48;
  const CL_NAME: &'static str = "access_tokens";

  fn indexes() -> Vec<IndexModel> {
    let variant = IndexModel::builder().keys(doc! { "scope": 1 }).build();
    let kind = IndexModel::builder().keys(doc! { "kind": 1 }).build();
    let user_id = IndexModel::builder().keys(doc! { "userId": 1 }).build();
    let admin_id = IndexModel::builder().keys(doc! { "adminId": 1 }).build();
    vec![variant, user_id, admin_id, kind]
  }
}

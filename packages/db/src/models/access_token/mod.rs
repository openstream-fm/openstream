use std::net::IpAddr;

use crate::Model;
// compiler bug (this is indeed used)
#[allow(unused)]
use mongodb::bson::{self, doc};
use mongodb::options::{FindOneAndUpdateOptions, IndexOptions, ReturnDocument};
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::as_f64;
use serde_util::DateTime;
use ts_rs::TS;
use user_agent::UserAgent;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TS)]
#[ts(export, export_to = "../../defs/db/", rename = "AccessTokenScope")]
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
#[ts(
  export,
  export_to = "../../defs/db/",
  rename = "AccessTokenGeneratedBy"
)]
#[serde(tag = "generatedBy", rename_all = "camelCase")]
pub enum GeneratedBy {
  Login {
    #[serde(with = "serde_util::ip")]
    ip: IpAddr,
    user_agent: UserAgent,
  },
  Register {
    #[serde(with = "serde_util::ip")]
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

  pub fn is_register(&self) -> bool {
    matches!(self, Self::Register { .. })
  }

  pub fn is_api(&self) -> bool {
    matches!(self, Self::Api { .. })
  }

  pub fn is_cli(&self) -> bool {
    matches!(self, Self::Cli { .. })
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
#[ts(export, export_to = "../../defs/db/", rename = "BaseAccessToken")]
#[serde(rename_all = "camelCase")]
#[macros::keys]
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

  pub created_at: DateTime,
  pub last_used_at: Option<DateTime>,

  #[serde(with = "as_f64")]
  pub hits: u64,
}

impl AccessToken {
  pub async fn touch(key: &str) -> Result<Option<AccessToken>, mongodb::error::Error> {
    let filter = doc! { Self::KEY_KEY: key };

    let now = serde_util::DateTime::now();

    let update = doc! {
      "$set": { Self::KEY_LAST_USED_AT: now },
      "$inc": { Self::KEY_HITS: 1 }
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

  pub fn is_generatyed_login(&self) -> bool {
    self.generated_by.is_login()
  }

  pub fn is_generated_register(&self) -> bool {
    self.generated_by.is_register()
  }

  pub fn is_generated_api(&self) -> bool {
    self.generated_by.is_api()
  }

  pub fn is_generated_cli(&self) -> bool {
    self.generated_by.is_cli()
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
      .keys(doc! { AccessToken::KEY_KEY: 1 })
      .options(IndexOptions::builder().unique(true).build())
      .build();

    // TODO: implement enum macros::keys()
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
    let now = DateTime::now();

    let key = AccessToken::random_key();

    let token = AccessToken {
      id: AccessToken::uid(),
      key,
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
    let now = DateTime::now();

    let key = AccessToken::random_key();

    let token = AccessToken {
      id: AccessToken::uid(),
      key,
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

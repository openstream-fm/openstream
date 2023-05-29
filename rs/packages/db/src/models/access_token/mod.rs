use std::collections::hash_map::Entry;
use std::net::IpAddr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Instant;

use crate::current_filter_doc;
use crate::Model;
use mongodb::bson::Document;
// compiler bug (this is indeed used)
#[allow(unused)]
use mongodb::bson::{self, doc};
use mongodb::options::IndexOptions;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::as_f64;
use serde_util::DateTime;
use ts_rs::TS;
use user_agent::UserAgent;

use log::*;

use tokio::time::Duration;

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

crate::register!(AccessToken);

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TS)]
#[ts(export, export_to = "../../../defs/db/", rename = "AccessTokenScope")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "scope")]
#[macros::keys]
pub enum Scope {
  Global,
  Admin { admin_id: String },
  AdminAsUser { admin_id: String, user_id: String },
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
  export_to = "../../../defs/db/",
  rename = "AccessTokenGeneratedBy"
)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "generated_by")]
#[macros::keys]
pub enum GeneratedBy {
  Login {
    #[serde(with = "serde_util::ip")]
    ip: IpAddr,
    user_agent: UserAgent,
    device_id: String,
  },
  Register {
    #[serde(with = "serde_util::ip")]
    ip: IpAddr,
    user_agent: UserAgent,
    device_id: String,
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
#[ts(export, export_to = "../../../defs/db/", rename = "BaseAccessToken")]
// #[ts(rename = "BaseAccessToken")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct AccessToken {
  #[serde(rename = "_id")]
  pub id: String,

  pub hash: String,

  /// the media_hash is used to access streams and files with access token scope
  /// directly from the client without exposing a full access token
  pub media_hash: String,

  #[serde(flatten)]
  // #[ts(skip)]
  pub scope: Scope,

  #[serde(flatten)]
  // #[ts(skip)]
  pub generated_by: GeneratedBy,

  pub last_used_at: Option<DateTime>,

  #[serde(with = "as_f64")]
  pub hits: u64,

  pub created_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

struct HitsEntry {
  hits: usize,
  last_used_at: DateTime,
}

static HITS_MAP: Mutex<Option<HashMap<String, HitsEntry>>> = Mutex::new(None);
static HIT_JOB_STARTED: AtomicBool = AtomicBool::new(false);

const HIT_SAVE_INTERVAL: Duration = Duration::from_secs(5);

fn start_access_token_hit_saver_job() {
  info!("access token hit saver background job started");
  tokio::spawn(async move {
    loop {
      tokio::time::sleep(HIT_SAVE_INTERVAL).await;
      let map = {
        let mut lock = HITS_MAP.lock();
        if lock.is_none() {
          trace!("hits saver loop: no changes");
          continue;
        }
        lock.take().unwrap()
      };

      for (id, entry) in map {
        let update = doc! {
          "$inc": { AccessToken::KEY_HITS: entry.hits as f64 },
          "$set": { AccessToken::KEY_LAST_USED_AT: entry.last_used_at }
        };

        match AccessToken::update_by_id(&id, update).await {
          Err(e) => {
            warn!("hits and last date save error for token {id}: {e}");
          }
          Ok(_) => {
            trace!("hits and last date saved for token {id}");
          }
        }
      }
    }
  });
}

const CACHE_CLEAR_INTERVAL: Duration = Duration::from_secs(5);
static CACHE_JOB_STARTED: AtomicBool = AtomicBool::new(false);

#[allow(clippy::type_complexity)]
static ACCESS_TOKEN_CACHE: tokio::sync::Mutex<
  Option<HashMap<String, (Instant, Arc<OnceCell<AccessToken>>)>>,
> = tokio::sync::Mutex::const_new(None);

const ACCESS_TOKEN_CACHE_VALIDITY: Duration = Duration::from_secs(300);

fn start_access_token_cache_job() {
  info!("access token cache clear job started");
  tokio::spawn(async move {
    loop {
      tokio::time::sleep(CACHE_CLEAR_INTERVAL).await;

      let mut lock = ACCESS_TOKEN_CACHE.lock().await;

      match lock.as_mut() {
        None => continue,
        Some(map) => map.retain(|_, (got_at, _)| got_at.elapsed() < CACHE_CLEAR_INTERVAL),
      }
    }
  });
}

impl AccessToken {
  pub async fn touch_cached(id_key: &str) -> Result<Option<AccessToken>, mongodb::error::Error> {
    let v = CACHE_JOB_STARTED.swap(true, Ordering::SeqCst);
    if !v {
      start_access_token_cache_job();
    }

    let cell = {
      let mut lock = ACCESS_TOKEN_CACHE.lock().await;

      let entry = lock
        .get_or_insert_with(HashMap::new)
        .entry(id_key.to_string());

      match entry {
        Entry::Occupied(mut entry) => {
          let (instant, cell) = entry.get();
          if instant.elapsed() > ACCESS_TOKEN_CACHE_VALIDITY {
            let cell = Arc::new(OnceCell::new());
            entry.insert((Instant::now(), cell.clone()));
            cell
          } else {
            cell.clone()
          }
        }

        Entry::Vacant(entry) => {
          let cell = Arc::new(OnceCell::new());
          entry.insert((Instant::now(), cell.clone()));
          cell
        }
      }
    };

    #[derive(Debug, thiserror::Error)]
    enum AccessTokenInitError {
      #[error("mongo: {0}")]
      Mongo(#[from] mongodb::error::Error),
      #[error("access token none")]
      None,
    }

    let result = cell
      .get_or_try_init(|| async {
        let (id, key) = match id_key.split_once('-') {
          None => return Err(AccessTokenInitError::None),
          Some(r) => r,
        };
        let hash = crypt::sha256(key);
        let filter = current_filter_doc! { AccessToken::KEY_ID: id, AccessToken::KEY_HASH: hash };
        let token = AccessToken::internal_touch(filter, false).await?;
        match token {
          Some(token) => Ok(token),
          None => Err(AccessTokenInitError::None),
        }
      })
      .await;

    match result {
      Ok(token) => {
        Self::hit(token.id.clone());
        Ok(Some(token.clone()))
      }

      Err(AccessTokenInitError::Mongo(e)) => Err(e),
      Err(AccessTokenInitError::None) => Ok(None),
    }
  }

  fn hit(id: String) {
    let v = HIT_JOB_STARTED.swap(true, Ordering::SeqCst);

    if !v {
      start_access_token_hit_saver_job();
    }

    let mut lock = HITS_MAP.lock();

    match lock.get_or_insert_with(HashMap::new).entry(id) {
      Entry::Vacant(entry) => {
        entry.insert(HitsEntry {
          hits: 1,
          last_used_at: DateTime::now(),
        });
      }

      Entry::Occupied(mut entry) => {
        let v = entry.get_mut();
        v.hits += 1;
        v.last_used_at = DateTime::now();
      }
    }
  }

  async fn internal_touch(
    filter: Document,
    hit: bool,
  ) -> Result<Option<AccessToken>, mongodb::error::Error> {
    let doc = match Self::get(filter).await? {
      None => return Ok(None),
      Some(doc) => doc,
    };

    if hit {
      Self::hit(doc.id.clone());
    }

    Ok(Some(doc))
  }

  pub async fn touch(id_key: &str) -> Result<Option<AccessToken>, mongodb::error::Error> {
    let (id, key) = match id_key.split_once('-') {
      None => return Ok(None),
      Some(r) => r,
    };

    let hash = crypt::sha256(key);

    Self::internal_touch(
      current_filter_doc! { AccessToken::KEY_ID: id, AccessToken::KEY_HASH: hash },
      true,
    )
    .await
  }

  pub async fn touch_by_media_key(
    id_media_key: &str,
  ) -> Result<Option<AccessToken>, mongodb::error::Error> {
    let (id, media_key) = match id_media_key.split_once('-') {
      None => return Ok(None),
      Some(r) => r,
    };

    let media_hash = crypt::sha256(media_key);

    Self::internal_touch(
      current_filter_doc! { AccessToken::KEY_ID: id,  AccessToken::KEY_MEDIA_HASH: media_hash },
      true,
    )
    .await
  }
}

impl AccessToken {
  pub fn random_key() -> String {
    uid::uid(48)
  }

  pub fn random_media_key() -> String {
    uid::uid(24)
  }

  const DEVICE_ID_LEN: usize = 24;
  pub fn random_device_id() -> String {
    uid::uid(Self::DEVICE_ID_LEN)
  }

  pub fn is_device_id_valid(device_id: &str) -> bool {
    if device_id.len() != Self::DEVICE_ID_LEN {
      false
    } else {
      lazy_regex::regex_is_match!("^[a-z0-9]+$", device_id)
    }
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

  pub fn is_deleted(&self) -> bool {
    self.deleted_at.is_some()
  }
}

impl Model for AccessToken {
  const UID_LEN: usize = 24;
  const CL_NAME: &'static str = "access_tokens";

  fn indexes() -> Vec<IndexModel> {
    let hash = IndexModel::builder()
      .keys(doc! { AccessToken::KEY_ID: 1, AccessToken::KEY_HASH: 1 })
      .options(IndexOptions::builder().unique(true).build())
      .build();

    let media_hash = IndexModel::builder()
      .keys(doc! { Self::KEY_ID: 1, Self::KEY_MEDIA_HASH: 1 })
      .options(IndexOptions::builder().unique(true).build())
      .build();

    let user_id = IndexModel::builder()
      .keys(doc! { Scope::KEY_USER_ID: 1 })
      .build();

    let admin_id = IndexModel::builder()
      .keys(doc! { Scope::KEY_ADMIN_ID: 1 })
      .build();

    let scope = IndexModel::builder()
      .keys(doc! { Scope::KEY_ENUM_TAG: 1 })
      .build();

    let generated_by = IndexModel::builder()
      .keys(doc! { GeneratedBy::KEY_ENUM_TAG: 1 })
      .build();

    let deleted_at = IndexModel::builder()
      .keys(doc! { Self::KEY_DELETED_AT: 1 })
      .build();

    vec![
      hash,
      media_hash,
      user_id,
      admin_id,
      scope,
      generated_by,
      deleted_at,
    ]
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_DELETED_AT, AccessToken::KEY_DELETED_AT);
    assert_eq!(crate::KEY_ID, AccessToken::KEY_ID);
  }

  #[test]
  fn serde_bson_vec() {
    let now = DateTime::now();

    let key = AccessToken::random_key();
    let hash = crypt::sha256(key);

    let media_key = AccessToken::random_media_key();
    let media_hash = crypt::sha256(media_key);

    let token = AccessToken {
      id: AccessToken::uid(),
      hash,
      media_hash,
      last_used_at: Some(now),
      generated_by: GeneratedBy::Api {
        title: String::from("Title"),
      },
      scope: Scope::Global,
      hits: 0,
      created_at: now,
      deleted_at: Some(now),
    };

    let vec = bson::to_vec(&token).expect("bson serialize");

    let out = bson::from_slice(&vec).expect("bson deserialize");

    assert_eq!(token, out);
  }

  #[test]
  fn serde_bson_doc() {
    let now = DateTime::now();

    let key = AccessToken::random_key();
    let media_key = AccessToken::random_media_key();

    let token = AccessToken {
      id: AccessToken::uid(),
      hash: crypt::sha256(key),
      media_hash: crypt::sha256(media_key),
      created_at: now,
      last_used_at: Some(now),
      generated_by: GeneratedBy::Api {
        title: String::from("Title"),
      },
      scope: Scope::Global,
      hits: 0,
      deleted_at: Some(now),
    };

    let doc = bson::to_document(&token).expect("bson serialize");

    let out = bson::from_document(doc).expect("bson deserialize");

    assert_eq!(token, out);
  }
}

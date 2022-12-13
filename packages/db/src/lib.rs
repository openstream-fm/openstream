use std::borrow::Borrow;

use async_trait::async_trait;
use futures_util::TryStreamExt;
use log::*;
use mongodb::error::Result as MongoResult;
use mongodb::options::ReplaceOptions;
use mongodb::{
  bson::{doc, Document},
  options::FindOneOptions,
  results::{InsertManyResult, InsertOneResult, UpdateResult},
  Client, ClientSession, Collection, Database, IndexModel,
};
use once_cell::sync::OnceCell;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ts_rs::TS;

pub mod access_token;
pub mod account;
pub mod admin;
pub mod audio_chunk;
pub mod audio_file;
pub mod audio_upload_operation;
pub mod config;
pub mod error;
pub mod event;
pub mod metadata;
pub mod station;
pub mod user;

static CLIENT: OnceCell<Client> = OnceCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExistsDocument {
  #[serde(rename = "_id")]
  pub id: String,
}

pub trait IntoExistFilter: Send + Sync {
  fn into_exists_filter(self) -> Document;
}

impl IntoExistFilter for Document {
  fn into_exists_filter(self) -> Document {
    self
  }
}

impl IntoExistFilter for String {
  fn into_exists_filter(self) -> Document {
    doc! { "_id": self }
  }
}

impl IntoExistFilter for &str {
  fn into_exists_filter(self) -> Document {
    doc! { "_id": self }
  }
}

pub fn init(client: Client) {
  CLIENT
    .set(client)
    .expect("[internal] mongodb client initialized more than once");
}

pub async fn ensure_collections() -> MongoResult<()> {
  config::Config::ensure_collection().await?;
  account::Account::ensure_collection().await?;
  audio_chunk::AudioChunk::ensure_collection().await?;
  audio_file::AudioFile::ensure_collection().await?;
  user::User::ensure_collection().await?;
  admin::Admin::ensure_collection().await?;
  station::Station::ensure_collection().await?;
  audio_upload_operation::AudioUploadOperation::ensure_collection().await?;
  access_token::AccessToken::ensure_collection().await?;
  event::Event::ensure_collection().await?;

  Ok(())
}

pub fn client_ref() -> &'static Client {
  CLIENT
    .get()
    .expect("[internal] mongodb client is not initialized, call db::init(Client) before using it")
}

pub fn client() -> Client {
  client_ref().clone()
}

pub fn db() -> Database {
  client_ref()
    .default_database()
    .expect("[internal] no database specified in mongodb connection string")
}

#[async_trait]
pub trait Model: Sized + Unpin + Send + Sync + Serialize + DeserializeOwned {
  const UID_LEN: usize;
  const CL_NAME: &'static str;

  fn uid() -> String {
    uid::uid(Self::UID_LEN)
  }

  fn cl_as<T: Serialize + DeserializeOwned>() -> Collection<T> {
    db().collection(Self::CL_NAME)
  }

  fn cl() -> Collection<Self> {
    Self::cl_as()
  }

  fn indexes() -> Vec<IndexModel> {
    vec![]
  }

  async fn ensure_collection() -> MongoResult<()> {
    Self::ensure_indexes().await?;
    Ok(())
  }

  async fn ensure_indexes() -> MongoResult<()> {
    let idxs = Self::indexes();
    if idxs.is_empty() {
      debug!(
        "ensuring indexes for collection {} => no indexes, skiping",
        Self::CL_NAME
      );
    } else {
      debug!(
        "ensuring indexes for collection {} => {} indexes",
        Self::CL_NAME,
        idxs.len()
      );

      if log_enabled!(Level::Trace) {
        for idx in idxs.iter() {
          trace!(
            "ensuring index for collection {} => {:?}",
            Self::CL_NAME,
            idx
          );
        }
      }

      Self::cl().create_indexes(idxs, None).await?;
    }

    Ok(())
  }

  async fn exists<F: IntoExistFilter>(filter: F) -> MongoResult<bool> {
    let options = FindOneOptions::builder()
      .projection(doc! { "_id": 1 })
      .build();
    let doc = Self::cl_as::<ExistsDocument>()
      .find_one(filter.into_exists_filter(), options)
      .await?;
    match doc {
      None => Ok(false),
      Some(_) => Ok(true),
    }
  }

  async fn exists_with_session<F: IntoExistFilter>(
    filter: F,
    session: &mut ClientSession,
  ) -> Result<bool, mongodb::error::Error> {
    let options = FindOneOptions::builder()
      .projection(doc! { "_id": 1 })
      .build();
    let doc = Self::cl_as::<ExistsDocument>()
      .find_one_with_session(filter.into_exists_filter(), options, session)
      .await?;
    match doc {
      None => Ok(false),
      Some(_) => Ok(true),
    }
  }

  async fn get(filter: Document) -> MongoResult<Option<Self>> {
    Self::cl().find_one(filter, None).await
  }

  async fn get_with_session(
    filter: Document,
    session: &mut ClientSession,
  ) -> MongoResult<Option<Self>> {
    Self::cl()
      .find_one_with_session(filter, None, session)
      .await
  }

  async fn get_by_id(id: &str) -> MongoResult<Option<Self>> {
    Self::get(doc! { "_id": id }).await
  }

  async fn get_by_id_with_session(
    id: &str,
    session: &mut ClientSession,
  ) -> MongoResult<Option<Self>> {
    Self::get_with_session(doc! { "_id": id }, session).await
  }

  async fn insert(
    doc: impl std::borrow::Borrow<Self> + Send + Sync,
  ) -> MongoResult<InsertOneResult> {
    Self::cl().insert_one(doc, None).await
  }

  async fn insert_with_session(
    doc: impl Borrow<Self> + Send + Sync,
    session: &mut ClientSession,
  ) -> MongoResult<InsertOneResult> {
    Self::cl().insert_one_with_session(doc, None, session).await
  }

  async fn insert_many(docs: &[Self]) -> MongoResult<InsertManyResult> {
    Self::cl().insert_many(docs, None).await
  }

  async fn insert_many_with_session(
    docs: &[Self],
    session: &mut ClientSession,
  ) -> MongoResult<InsertManyResult> {
    Self::cl()
      .insert_many_with_session(docs, None, session)
      .await
  }

  async fn replace(
    id: &str,
    replacement: impl std::borrow::Borrow<Self> + Send + Sync,
  ) -> MongoResult<UpdateResult> {
    Self::cl()
      .replace_one(doc! {"_id": id}, replacement, None)
      .await
  }

  async fn replace_with_session(
    id: &str,
    replacement: impl std::borrow::Borrow<Self> + Send + Sync,
    session: &mut ClientSession,
  ) -> MongoResult<UpdateResult> {
    Self::cl()
      .replace_one_with_session(doc! {"_id": id}, replacement, None, session)
      .await
  }

  async fn paged(
    filter: impl Into<Option<Document>> + Send,
    skip: u64,
    limit: i64,
  ) -> MongoResult<Paged<Self>> {
    let filter = filter.into();
    let total = Self::cl().count_documents(filter.clone(), None).await?;
    let items = Self::cl().find(filter, None).await?.try_collect().await?;

    Ok(Paged {
      total,
      skip,
      limit,
      items,
    })
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/")]
#[serde(rename_all = "camelCase")]
pub struct Paged<T> {
  pub total: u64,
  pub skip: u64,
  pub limit: i64,
  pub items: Vec<T>,
}

impl<T> Paged<T> {
  pub fn map<O>(self, f: impl FnMut(T) -> O) -> Paged<O> {
    let Paged {
      total,
      skip,
      limit,
      items,
    } = self;
    Paged {
      total,
      skip,
      limit,
      items: items.into_iter().map(f).collect(),
    }
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PublicScope {
  Admin,
  User,
}

impl PublicScope {
  pub fn is_admin(self) -> bool {
    matches!(self, Self::Admin)
  }

  pub fn is_user(self) -> bool {
    matches!(self, Self::User)
  }
}

#[macro_export]
macro_rules! run_transaction {
  ($session:ident => $block:block) => {{
    let mut $session = $crate::client().start_session(None).await?;
    $session.start_transaction(None).await?;

    let r = $block;

    $session.commit_transaction().await?;

    r
  }};
}

pub const SINGLETON_UID_LEN: usize = 1;
fn singleton_uid() -> String {
  String::from("0")
}

#[async_trait]
pub trait Singleton: Model + Default + Clone {
  async fn ensure_instance() -> Result<Self, mongodb::error::Error> {
    run_transaction!(session => {
      let cl = Self::cl();
      let instance = cl.find_one_with_session(doc!{}, None, &mut session).await?;
      match instance {
        Some(instance) => Ok(instance),
        None => {
          let instance = Self::default();
          cl.insert_one_with_session(&instance, None, &mut session).await?;
          Ok(instance)
        }
      }
    })
  }

  async fn get() -> Result<Self, mongodb::error::Error> {
    let cl = Self::cl();
    let instance = cl.find_one(None, None).await?;
    Ok(instance.unwrap_or_default())
  }

  async fn get_with_session(session: &mut ClientSession) -> Result<Self, mongodb::error::Error> {
    let cl = Self::cl();
    let instance = cl.find_one_with_session(None, None, session).await?;
    Ok(instance.unwrap_or_default())
  }

  async fn set(doc: impl Borrow<Self> + Send) -> Result<(), mongodb::error::Error> {
    let cl = Self::cl();
    let options = ReplaceOptions::builder().upsert(true).build();
    cl.replace_one(doc! {}, doc, options).await?;
    Ok(())
  }

  async fn set_with_session(
    doc: impl Borrow<Self> + Send,
    session: &mut ClientSession,
  ) -> Result<(), mongodb::error::Error> {
    let cl = Self::cl();
    let options = ReplaceOptions::builder().upsert(true).build();
    cl.replace_one_with_session(doc! {}, doc, options, session)
      .await?;
    Ok(())
  }
}

// #[macro_export]
// macro_rules! fetch_and_update {
//   ($Model:ident, $id:expr, $err:expr, $session:expr, $apply:expr) => {
//     let id = $id;
//     $Model::get_with_session($id)
//   };
// }

#[macro_export]
macro_rules! fetch_and_patch {
  ($Model:ident, $name:ident, $id:expr, $err:expr, $session:ident, $apply:expr) => {{
    let mut $name = match $Model::get_by_id_with_session($id, &mut $session).await? {
      Some(doc) => doc,
      None => return $err,
    };

    // this seems like a clippy bug
    // #[allow(clippy::unnecessary_operation)]
    $apply;

    $Model::replace_with_session($id, &$name, &mut $session).await?;

    $name
  }};
}

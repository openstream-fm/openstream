use std::borrow::Borrow;

use async_trait::async_trait;
use futures_util::TryStreamExt;
use log::*;
use mongodb::error::Result as MongoResult;
use mongodb::{
  bson::{doc, Document},
  options::FindOneOptions,
  results::{InsertManyResult, InsertOneResult, UpdateResult},
  Client, ClientSession, Collection, Database, IndexModel,
};
use once_cell::sync::OnceCell;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod access_token;
pub mod account;
pub mod admin;
pub mod audio_chunk;
pub mod audio_file;
pub mod audio_upload_operation;
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

pub async fn ensure_indexes() -> MongoResult<()> {
  account::Account::ensure_indexes().await?;
  audio_chunk::AudioChunk::ensure_indexes().await?;
  audio_file::AudioFile::ensure_indexes().await?;
  user::User::ensure_indexes().await?;
  admin::Admin::ensure_indexes().await?;
  station::Station::ensure_indexes().await?;
  audio_upload_operation::AudioUploadOperation::ensure_indexes().await?;
  access_token::AccessToken::ensure_indexes().await?;
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

  async fn get_by_id(id: &str) -> MongoResult<Option<Self>> {
    Self::cl().find_one(doc! { "_id": id }, None).await
  }

  async fn get_by_id_with_session(
    id: &str,
    session: &mut ClientSession,
  ) -> MongoResult<Option<Self>> {
    Self::cl()
      .find_one_with_session(doc! { "_id": id }, None, session)
      .await
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

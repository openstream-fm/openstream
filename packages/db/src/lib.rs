use async_trait::async_trait;
use log::*;
use mongodb::{
  bson::doc,
  results::{InsertOneResult, UpdateResult},
  Client, Collection, Database, IndexModel,
};
use once_cell::sync::OnceCell;
use serde::{de::DeserializeOwned, Serialize};

pub mod access_token;
pub mod account;
pub mod audio_chunk;
pub mod audio_file;
pub mod audio_upload_operation;
pub mod metadata;
pub mod station;
pub mod user;

static CLIENT: OnceCell<Client> = OnceCell::new();

pub fn init(client: Client) {
  CLIENT
    .set(client)
    .expect("[internal] mongodb client initialized more than once");
}

pub async fn ensure_indexes() -> Result<(), mongodb::error::Error> {
  account::Account::ensure_indexes().await?;
  audio_chunk::AudioChunk::ensure_indexes().await?;
  audio_file::AudioFile::ensure_indexes().await?;
  user::User::ensure_indexes().await?;
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
  fn uid_len() -> usize;
  fn cl_name() -> &'static str;

  fn uid() -> String {
    uid::uid(Self::uid_len())
  }

  fn cl_as<T: Serialize + DeserializeOwned>() -> Collection<T> {
    db().collection(Self::cl_name())
  }

  fn cl() -> Collection<Self> {
    Self::cl_as()
  }

  fn indexes() -> Vec<IndexModel> {
    vec![]
  }

  async fn ensure_indexes() -> Result<(), mongodb::error::Error> {
    let idxs = Self::indexes();
    if idxs.is_empty() {
      debug!(
        "ensuring indexes for collection {} => no indexes, skiping",
        Self::cl_name()
      );
    } else {
      debug!(
        "ensuring indexes for collection {} => {} indexes",
        Self::cl_name(),
        idxs.len()
      );

      if log_enabled!(Level::Trace) {
        for idx in idxs.iter() {
          trace!(
            "ensuring index for collection {} => {:?}",
            Self::cl_name(),
            idx
          );
        }
      }

      Self::cl().create_indexes(idxs, None).await?;
    }

    Ok(())
  }

  async fn get_by_id(id: &str) -> Result<Option<Self>, mongodb::error::Error> {
    Self::cl().find_one(doc! { "_id": id }, None).await
  }

  async fn insert(
    doc: impl std::borrow::Borrow<Self> + Send + Sync,
  ) -> Result<InsertOneResult, mongodb::error::Error> {
    Self::cl().insert_one(doc, None).await
  }

  async fn replace(
    id: &str,
    replacement: impl std::borrow::Borrow<Self> + Send + Sync,
  ) -> Result<UpdateResult, mongodb::error::Error> {
    Self::cl()
      .replace_one(doc! {"_id": id}, replacement, None)
      .await
  }
}

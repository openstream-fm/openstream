use async_trait::async_trait;
use error::CheckCollectionError;
use futures_util::{Future, TryStreamExt};
use log::*;
use mongodb::error::Result as MongoResult;
use mongodb::options::{
  CountOptions, FindOptions, Hint, ReplaceOptions, SelectionCriteria, SessionOptions,
  TransactionOptions,
};
use mongodb::results::DeleteResult;
use mongodb::{
  bson::{doc, Document},
  options::FindOneOptions,
  results::{InsertManyResult, InsertOneResult, UpdateResult},
  Client, ClientSession, Collection, Database, IndexModel,
};
use once_cell::sync::OnceCell;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_util::DateTime;
use std::borrow::Borrow;
use ts_rs::TS;

pub mod error;
pub mod http;
pub mod metadata;
pub mod models;
pub mod registry;

pub use models::*;

static CLIENT_AND_STORAGE_DB_NAME: OnceCell<(Client, Option<String>)> = OnceCell::new();

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
    doc! { crate::KEY_ID: self }
  }
}

impl IntoExistFilter for &str {
  fn into_exists_filter(self) -> Document {
    doc! { crate::KEY_ID: self }
  }
}

pub fn init(client: Client, storage_db_name: Option<String>) {
  try_init(client, storage_db_name).expect("[internal] mongodb client initialized more than once, this is a bug, please file an issue at https://github.com/ramiroaisen/openstream")
}

pub fn try_init(
  client: Client,
  storage_db_name: Option<String>,
) -> Result<(), (Client, Option<String>)> {
  CLIENT_AND_STORAGE_DB_NAME.set((client, storage_db_name))
}

pub async fn ensure_collections() -> MongoResult<()> {
  let registry = registry::Registry::global();
  registry.ensure_collections().await?;
  Ok(())
}

pub fn client_ref() -> &'static Client {
  let (client, _) = CLIENT_AND_STORAGE_DB_NAME
    .get()
    .expect("[internal] mongodb client is not initialized, call db::init(Client) before using it");
  client
}

pub fn client() -> Client {
  client_ref().clone()
}

pub fn db() -> Database {
  client_ref()
    .default_database()
    .expect("[internal] no database specified in mongodb connection string")
}

pub fn storage_db() -> Database {
  let (client, storage_db_name) = CLIENT_AND_STORAGE_DB_NAME
    .get()
    .expect("[internal] mongodb client is not initialized, call db::init(Client) before using it");
  match storage_db_name {
    Some(name) => client.database(name),
    None => client
      .default_database()
      .expect("[internal] no database specified in mongodb connecton string"),
  }
}

#[async_trait]
pub trait Model: Sized + Unpin + Send + Sync + Serialize + DeserializeOwned {
  const UID_LEN: usize;
  const CL_NAME: &'static str;

  fn uid() -> String {
    uid::uid(Self::UID_LEN)
  }

  fn db() -> Database {
    db()
  }

  fn cl_as<T: Serialize + DeserializeOwned>() -> Collection<T> {
    Self::db().collection(Self::CL_NAME)
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

  async fn set_deleted(filter: Document) -> MongoResult<UpdateResult> {
    let update = doc! { "$set": { crate::KEY_DELETED_AT: DateTime::now() } };
    Self::cl().update_many(filter, update, None).await
  }

  async fn set_deleted_with_session(
    filter: Document,
    session: &mut ClientSession,
  ) -> MongoResult<UpdateResult> {
    let update = doc! { "$set": { crate::KEY_DELETED_AT: DateTime::now() } };
    Self::cl()
      .update_many_with_session(filter, update, None, session)
      .await
  }

  async fn set_deleted_by_id(id: &str) -> MongoResult<UpdateResult> {
    let update = doc! { "$set": { crate::KEY_DELETED_AT: DateTime::now() } };
    Self::update_by_id(id, update).await
  }

  async fn set_deleted_by_id_with_session(
    id: &str,
    session: &mut ClientSession,
  ) -> MongoResult<UpdateResult> {
    let update = doc! { "$set": { crate::KEY_DELETED_AT: DateTime::now() } };
    Self::update_by_id_with_session(id, update, session).await
  }

  async fn delete_by_id(id: &str) -> MongoResult<DeleteResult> {
    Self::cl()
      .delete_one(doc! { crate::KEY_ID: id }, None)
      .await
  }

  async fn delete_by_id_with_session(
    id: &str,
    session: &mut ClientSession,
  ) -> MongoResult<DeleteResult> {
    Self::cl()
      .delete_one_with_session(doc! { crate::KEY_ID: id }, None, session)
      .await
  }

  async fn exists<F: IntoExistFilter>(filter: F) -> MongoResult<bool> {
    let options = FindOneOptions::builder()
      .projection(doc! { crate::KEY_ID: 1 })
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
      .projection(doc! { crate::KEY_ID: 1 })
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
    Self::get(doc! { crate::KEY_ID: id }).await
  }

  async fn get_by_id_with_session(
    id: &str,
    session: &mut ClientSession,
  ) -> MongoResult<Option<Self>> {
    Self::get_with_session(doc! { crate::KEY_ID: id }, session).await
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
      .replace_one(doc! { crate::KEY_ID: id }, replacement, None)
      .await
  }

  async fn replace_with_session(
    id: &str,
    replacement: impl std::borrow::Borrow<Self> + Send + Sync,
    session: &mut ClientSession,
  ) -> MongoResult<UpdateResult> {
    Self::cl()
      .replace_one_with_session(doc! { crate::KEY_ID: id }, replacement, None, session)
      .await
  }

  async fn update_by_id(id: &str, update: Document) -> MongoResult<UpdateResult> {
    Self::cl()
      .update_one(doc! { crate::KEY_ID: id }, update, None)
      .await
  }

  async fn update_by_id_with_session(
    id: &str,
    update: Document,
    session: &mut ClientSession,
  ) -> MongoResult<UpdateResult> {
    Self::cl()
      .update_one_with_session(doc! { crate::KEY_ID: id }, update, None, session)
      .await
  }

  async fn paged(
    filter: impl Into<Option<Document>> + Send,
    sort: impl Into<Option<Document>> + Send,
    skip: u64,
    limit: i64,
  ) -> MongoResult<Paged<Self>> {
    Self::paged_with_optional_hint(filter, sort, skip, limit, None).await
  }

  async fn paged_with_hint(
    filter: impl Into<Option<Document>> + Send,
    sort: impl Into<Option<Document>> + Send,
    skip: u64,
    limit: i64,
    hint: Document,
  ) -> MongoResult<Paged<Self>> {
    Self::paged_with_optional_hint(filter, sort, skip, limit, Some(hint)).await
  }

  async fn paged_with_optional_hint(
    filter: impl Into<Option<Document>> + Send,
    sort: impl Into<Option<Document>> + Send,
    skip: u64,
    limit: i64,
    hint: Option<Document>,
  ) -> MongoResult<Paged<Self>> {
    let sort = sort.into().unwrap_or_else(|| doc! { "$natural": 1 });
    let filter = filter.into();
    let hint = hint.map(Hint::Keys);
    let options = FindOptions::builder()
      .sort(sort)
      .skip(skip)
      .limit(limit)
      .hint(hint.clone())
      .build();

    let count_options = CountOptions::builder().hint(hint).build();

    let total = Self::cl()
      .count_documents(filter.clone(), count_options)
      .await?;

    let items = Self::cl()
      .find(filter, options)
      .await?
      .try_collect()
      .await?;

    Ok(Paged {
      total,
      skip,
      limit,
      items,
    })
  }

  async fn check_collection_documents() -> Result<u64, CheckCollectionError> {
    use CheckCollectionError::*;

    let cl_name = Self::CL_NAME;

    let cl = Self::cl_as::<Document>();

    info!("checking collection {}", cl_name);

    let count = cl.count_documents(None, None).await.map_err(Count)?;

    info!(
      "checking collection {}, counted {} documents",
      cl_name, count
    );

    let mut cursor = cl.find(None, None).await.map_err(Find)?;

    let mut i: u64 = 0;
    let mut bson_errors = Vec::<(u64, Document, mongodb::bson::de::Error)>::new();

    while let Some(document) = cursor.try_next().await.map_err(Cursor)? {
      i += 1;
      if i % 1000 == 0 {
        info!("checking collection {cl_name}, testing document {i} of {count}");
      }
      match mongodb::bson::from_document::<Self>(document.clone()) {
        Ok(_) => {}
        Err(e) => {
          warn!("error deserializing document {i}");
          warn!("document: {:?}", document);
          warn!("error: {} => {:?}", e, e);
          bson_errors.push((i, document, e));
        }
      }
    }

    let error_count = bson_errors.len();
    info!("checking collection {cl_name}, tested {i} documents, found {error_count} errors");

    match bson_errors.len() {
      0 => Ok(i),
      _ => Err(Deserialize(bson_errors)),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export)]
#[ts(export_to = "../../../defs/")]
#[serde(rename_all = "snake_case")]
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

  pub async fn map_async<O, F: FnMut(T) -> Fut, Fut: Future<Output = O>>(
    self,
    buffer: usize,
    f: F,
  ) -> Paged<O> {
    let Paged {
      total,
      skip,
      limit,
      items,
    } = self;

    use futures_util::StreamExt;

    let items = futures_util::stream::iter(items)
      .map(f)
      .buffered(buffer)
      .collect()
      .await;

    Paged {
      total,
      skip,
      limit,
      items,
    }
  }

  pub async fn try_map_async<O, E, F: FnMut(T) -> Fut, Fut: Future<Output = Result<O, E>>>(
    self,
    buffer: usize,
    f: F,
  ) -> Result<Paged<O>, E> {
    let Paged {
      total,
      skip,
      limit,
      items,
    } = self;

    use futures_util::StreamExt;
    //use futures_util::TryStreamExt;

    let items = futures_util::stream::iter(items)
      .map(f)
      .buffered(buffer)
      .try_collect()
      .await?;

    Ok(Paged {
      total,
      skip,
      limit,
      items,
    })
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

pub fn transaction_session_options() -> SessionOptions {
  let selection_criteria =
    SelectionCriteria::ReadPreference(mongodb::options::ReadPreference::Primary);

  let transaction_options = TransactionOptions::builder()
    .selection_criteria(selection_criteria)
    .build();

  #[allow(clippy::let_and_return)]
  let session_options = SessionOptions::builder()
    .default_transaction_options(transaction_options)
    .build();

  session_options
}

#[macro_export]
macro_rules! run_transaction {

  ($session:ident => $block:block) => {{
    $crate::run_transaction!($session, @options=$crate::transaction_session_options() => $block)
  }};

  ($session:ident, @options=$options:expr => $block:block) => {{

    const MAX_TX_RETRIES: usize = 5;
    let mut tx_retries = 0;

    #[deny(unused_labels)]
    let (r, mut $session) = 'tx: loop {

      let mut $session = $crate::client().start_session($options).await?;
      $session.start_transaction(None).await?;

      #[deny(unused_macros)]
      macro_rules! tx_try {
        ($e:expr) => {
          match $e {
            Ok(r) => r,
            Err(e) => {
              if e.contains_label(::mongodb::error::TRANSIENT_TRANSACTION_ERROR) {
                tx_retries += 1;
                if tx_retries <= MAX_TX_RETRIES {
                  continue 'tx;
                } else {
                  return Err(e.into());
                }
              } else {
                return Err(e.into());
              }
            }
          }
        };
      }

      break ($block, $session);
    };

    loop {
      match $session.commit_transaction().await {
        Err(e) => {
          if e.contains_label(::mongodb::error::UNKNOWN_TRANSACTION_COMMIT_RESULT) {
            continue;
          } else {
            return Err(e.into());
          }
        }
        Ok(_) => break r,
      }
    }
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
      let instance = tx_try!(Self::cl().find_one_with_session(None, None, &mut session).await);
      match instance {
        Some(instance) => Ok(instance),
        None => {
          let instance = Self::default();
          tx_try!(Self::cl().insert_one_with_session(&instance, None, &mut session).await);
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

#[macro_export]
macro_rules! fetch_and_patch {
  ($Model:ident, $name:ident, $id:expr, $err:expr, $session:ident, $apply:expr) => {{
    let mut $name = match tx_try!($Model::get_by_id_with_session($id, &mut $session).await) {
      Some(doc) => doc,
      None => return $err,
    };

    // this seems like a clippy bug
    #[allow(clippy::unnecessary_operation)]
    $apply;

    tx_try!($Model::replace_with_session($id, &$name, &mut $session).await);

    $name
  }};
}

pub async fn test_setup() {
  let client =
    mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017/openstream-test?replicaSet=rs1")
      .await
      .expect("failed to create mongodb client");

  if crate::try_init(client, None).is_ok() {
    crate::ensure_collections()
      .await
      .expect("error ensuring db collections");
  }
}

#[macro_export]
macro_rules! key {
  ($first:expr, $($rest:expr),*) => {
    const_str::concat!(
      $first,
      $(".", $rest,)*
    )
  };
}

pub const KEY_ID: &str = "_id";
pub const KEY_DELETED_AT: &str = "deleted_at";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdDocument {
  #[serde(rename = "_id")]
  pub id: String,
}

impl IdDocument {
  pub fn projection() -> Document {
    doc! { KEY_ID: 1 }
  }
}

#[macro_export]
macro_rules! current_filter_doc {
  () => {
    ::mongodb::bson::doc!{ $crate::KEY_DELETED_AT: null }
  };

  ($filter:ident) => {
    ::mongodb::bson::doc!{
      "$and": [
        { $crate::KEY_DELETED_AT: null },
        $filter,
      ]
    }
  };

  ($($tt:tt)*) => {
    ::mongodb::bson::doc! {
      "$and": [
        { $crate::KEY_DELETED_AT: null },
        { $($tt)* },
      ]
    }
  }
}

#[macro_export]
macro_rules! deleted_filter_doc {
  () => {
    ::mongodb::bson::doc!{ $crate::KEY_DELETED_AT: { "$ne": null } }
  };

  ($filter:ident) => {
    ::mongodb::bson::doc!{
      "$and": [
        { $crate::KEY_DELETED_AT: { "$ne": null } },
       $filter,
      ]
    }
  };

  ($($tt:tt)*) => {
    ::mongodb::bson::doc! {
      "$and": [
        { $crate::KEY_DELETED_AT: { "$ne": null } },
        { $($tt)* },
      ]
    }
  }
}

// const KEY_ORDER_BOUNDS_NEXT: &str = "next";
// const KEY_ORDER_BOUNDS_PREV: &str = "prev";

// #[derive(Serialize, Deserialize)]
// pub struct OrderBoundsNext {
//   next: f64,
// }

// impl OrderBoundsNext {
//   pub fn projection() -> Document {
//     doc! { KEY_ID: 0, KEY_ORDER_BOUNDS_NEXT: 1 }
//   }
// }

// #[derive(Serialize, Deserialize)]
// struct OrderBoundsPrev {
//   next: f64,
// }

// impl OrderBoundsPrev {
//   pub fn projection() -> Document {
//     doc! { KEY_ID: 0, KEY_ORDER_BOUNDS_PREV: 1 }
//   }
// }

// #[async_trait]
// pub trait OrderBounds: Model {
//   async fn next(id: &str) -> Result<f64, mongodb::error::Error> {
//     let filter = doc! { KEY_ID: id };
//     let update = doc! { "$setOnInsert": { KEY_ID: id, KEY_ORDER_BOUNDS_PREV: -1.0 }, "$inc": { KEY_ORDER_BOUNDS_NEXT: 1f64 } };
//     let options = FindOneAndUpdateOptions::builder()
//       .upsert(true)
//       .projection(OrderBoundsNext::projection())
//       .return_document(ReturnDocument::Before)
//       .build();

//     match Self::cl_as::<OrderBoundsNext>()
//       .find_one_and_update(filter, update, options)
//       .await?
//     {
//       Some(doc) => Ok(doc.next),
//       None => Ok(0.0),
//     }
//   }

//   async fn next_with_session(
//     id: &str,
//     session: &mut ClientSession,
//   ) -> Result<f64, mongodb::error::Error> {
//     let filter = doc! { KEY_ID: id };
//     let update = doc! { "$setOnInsert": { KEY_ID: id, KEY_ORDER_BOUNDS_PREV: -1.0 }, "$inc": { KEY_ORDER_BOUNDS_NEXT: 1f64 } };
//     let options = FindOneAndUpdateOptions::builder()
//       .upsert(true)
//       .projection(OrderBoundsNext::projection())
//       .return_document(ReturnDocument::Before)
//       .build();

//     match Self::cl_as::<OrderBoundsNext>()
//       .find_one_and_update_with_session(filter, update, options, session)
//       .await?
//     {
//       Some(doc) => Ok(doc.next),
//       None => Ok(0.0),
//     }
//   }

//   async fn prev(id: &str) -> Result<f64, mongodb::error::Error> {
//     let filter = doc! { KEY_ID: id };
//     let update = doc! { "$setOnInsert": { KEY_ID: id, KEY_ORDER_BOUNDS_NEXT: 0.0 }, "$inc": { KEY_ORDER_BOUNDS_PREV: -1f64 } };
//     let options = FindOneAndUpdateOptions::builder()
//       .upsert(true)
//       .projection(OrderBoundsPrev::projection())
//       .return_document(ReturnDocument::Before)
//       .build();

//     match Self::cl_as::<OrderBoundsPrev>()
//       .find_one_and_update(filter, update, options)
//       .await?
//     {
//       Some(doc) => Ok(doc.next),
//       None => Ok(0.0),
//     }
//   }

//   async fn prev_with_session(
//     id: &str,
//     session: &mut ClientSession,
//   ) -> Result<f64, mongodb::error::Error> {
//     let filter = doc! { KEY_ID: id };
//     let update = doc! { "$setOnInsert": { KEY_ID: id, KEY_ORDER_BOUNDS_NEXT: 0.0 }, "$inc": { KEY_ORDER_BOUNDS_PREV: -1f64 } };
//     let options = FindOneAndUpdateOptions::builder()
//       .upsert(true)
//       .projection(OrderBoundsPrev::projection())
//       .return_document(ReturnDocument::Before)
//       .build();

//     match Self::cl_as::<OrderBoundsPrev>()
//       .find_one_and_update_with_session(filter, update, options, session)
//       .await?
//     {
//       Some(doc) => Ok(doc.next),
//       None => Ok(-1.0),
//     }
//   }
// }

#[cfg(test)]
#[test]
fn current_filter_doc() {
  current_filter_doc! {
    "hello": "world",
  };

  current_filter_doc! {
    "$and": [ { KEY_ID: "id" } ],
  };
}

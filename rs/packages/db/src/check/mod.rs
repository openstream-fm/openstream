use crate::Model;
use futures_util::TryStreamExt;
use indexmap::IndexMap;
use log::*;
use mongodb::bson::Document;

use crate::models::*;

pub async fn check_all() -> IndexMap<&'static str, Result<u64, CheckCollectionError>> {
  let mut map = IndexMap::new();
  macro_rules! check {
    ($model:ty) => {
      let r = check_collection::<$model>().await;
      map.insert(<$model>::CL_NAME, r);
    };
  }

  check!(config::Config);
  check!(admin::Admin);
  check!(user::User);
  check!(user_account_relation::UserAccountRelation);
  check!(account::Account);
  check!(station::Station);
  check!(access_token::AccessToken);
  check!(deployment::Deployment);
  check!(transfer_checkpoint::TransferCheckpoint);
  check!(media_session::MediaSession);
  check!(event::Event);
  check!(play_history_item::PlayHistoryItem);
  check!(stream_connection::StreamConnection);
  check!(stream_connection::lite::StreamConnectionLite);
  check!(audio_upload_operation::AudioUploadOperation);
  check!(audio_file::AudioFile);
  check!(audio_chunk::AudioChunk);

  map
}

pub async fn check_collection<M: Model>() -> Result<u64, CheckCollectionError> {
  use CheckCollectionError::*;

  let cl_name = M::CL_NAME;

  let cl = M::cl_as::<Document>();

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
    match mongodb::bson::from_document::<M>(document.clone()) {
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

#[derive(Debug, thiserror::Error)]
pub enum CheckCollectionError {
  #[error("cl::count_documents error: {0}")]
  Count(mongodb::error::Error),
  #[error("cl::find error: {0}")]
  Find(mongodb::error::Error),
  #[error("cursor::try_next error: {0}")]
  Cursor(mongodb::error::Error),
  #[error("bson::from_document encounter one or more errors")]
  Deserialize(Vec<(u64, Document, mongodb::bson::de::Error)>),
}

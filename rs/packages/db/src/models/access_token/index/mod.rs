use super::AccessToken;
use crate::current_filter_doc;
use crate::Model;
use futures_util::TryStreamExt;
use mongodb::bson::Timestamp;
use mongodb::change_stream::event::OperationType;
use mongodb::options::{ChangeStreamOptions, FullDocumentType};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

static GLOBAL: once_cell::sync::Lazy<AccessTokenIndex> =
  once_cell::sync::Lazy::new(AccessTokenIndex::new);

#[derive(Debug, Clone)]
pub struct AccessTokenIndex {
  map: Arc<RwLock<HashMap<String, AccessToken>>>,
}

impl AccessTokenIndex {
  pub async fn get_by_id(&self, id: &str) -> Option<AccessToken> {
    self.map.read().await.get(id).cloned()
  }

  pub fn global() -> &'static Self {
    &GLOBAL
  }
}

impl Default for AccessTokenIndex {
  fn default() -> Self {
    Self::new()
  }
}

impl AccessTokenIndex {
  pub fn new() -> Self {
    let map = Arc::new(RwLock::new(HashMap::new()));

    tokio::spawn({
      let map = map.clone();
      async move {
        let mut loop_time: usize = 0;
        'root: loop {
          macro_rules! loop_try {
            ($e:expr, $message:expr) => {
              match $e {
                Err(e) => {
                  log::error!(
                    target: "access-token-index",
                    "access-token-index error ({}): {} {:?}", $message, e, e
                  );
                  continue 'root;
                }
                Ok(v) => v,
              }
            };
          }

          if Arc::strong_count(&map) == 1 {
            break;
          }

          loop_time += 1;

          log::info!(
            target: "access-token-index",
            "access-token-index start, loop {loop_time}"
          );

          let indexing_start = Instant::now();
          let indexing_start_timestamp = Timestamp {
            time: DateTime::now().unix_timestamp() as u32,
            increment: 0,
          };

          {
            let mut lock = map.write().await;

            // resetting the map
            *lock = HashMap::new();

            let filter = current_filter_doc!();

            let options = mongodb::options::FindOptions::builder().build();

            let mut cursor = loop_try!(AccessToken::cl().find(filter, options).await, "cl::find");

            while let Some(item) = loop_try!(cursor.try_next().await, "cl::find::cursor::try_next")
            {
              lock.insert(item.id.clone(), item);
            }

            log::info!(
              target: "access-token-index",
              "access-token-index indexing end: {} items in {}ms",
              lock.len(),
              indexing_start.elapsed().as_millis()
            );

            // log::info!(
            //   target: "access-token-index",
            //   "access-token-index index size: {} items",
            //   lock.len()
            //   //human_bytes::human_bytes(lock.deep_size_of() as f64),
            // )
          }

          let options = ChangeStreamOptions::builder()
            .full_document(Some(FullDocumentType::UpdateLookup))
            .start_at_operation_time(Some(indexing_start_timestamp))
            .build();

          let mut cursor = loop_try!(AccessToken::cl().watch([], options).await, "cl::watch");

          while let Some(event) = loop_try!(cursor.try_next().await, "cl::watch::cursor::try_next")
          {
            if Arc::strong_count(&map) == 1 {
              return;
            }

            match event.operation_type {
              OperationType::Delete => {
                #[derive(Debug, Serialize, Deserialize)]
                struct DocumentKey {
                  #[serde(rename = "_id")]
                  id: String,
                }

                match event.document_key {
                  None => {
                    log::warn!(
                      target: "access-token-index",
                      "access-token-index: event.document_key is None for {:#?}",
                      event
                    );
                  }
                  Some(doc) => match mongodb::bson::from_document::<DocumentKey>(doc.clone()) {
                    Err(e) => {
                      log::warn!(
                        target: "access-token-index",
                        "access-token-index: failed to deserialize event.document_key: {}, {:?}: {:?}", e, e, doc
                      );
                    }

                    Ok(doc) => {
                      map.write().await.remove(&doc.id);
                    }
                  },
                }
              }

              OperationType::Insert | OperationType::Replace | OperationType::Update => {
                match event.full_document {
                  None => {
                    log::warn!(
                      target: "access-token-index",
                      "access-token-index: event without full document: {:?}",
                      event
                    );
                  }

                  Some(item) => {
                    if item.deleted_at.is_some() {
                      map.write().await.remove(&item.id);
                    } else {
                      map.write().await.insert(item.id.clone(), item);
                    }
                  }
                }
              }

              OperationType::Invalidate | OperationType::Drop | OperationType::DropDatabase => {
                log::warn!(
                  target: "access-token-index",
                  "access-token-index, operation {:?} received, sleeping and reseting index",
                  event.operation_type
                );
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                continue 'root;
              }

              op => {
                log::error!(
                  target: "access-token-index",
                  "access-token-index, unknown operation_type {:?}, resetting cursor",
                  op
                );
                continue 'root;
              }
            }
          }
        }
      }
    });

    Self { map }
  }
}

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use log::{trace, warn};
use mongodb::bson::{self, doc};
use mongodb::options::FindOptions;
use mongodb::{options::CreateCollectionOptions, IndexModel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use ts_rs::TS;

use crate::{db, Model};

static WATCHER: Mutex<Option<Watcher>> = Mutex::const_new(None);

struct Watcher {
  tx: tokio::sync::broadcast::Sender<Arc<Event>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[ts(rename = "BaseEvent")]
#[serde(rename_all = "camelCase")]
pub struct Event {
  #[serde(rename = "_id")]
  id: String,

  #[serde(with = "serde_util::datetime")]
  created_at: DateTime<Utc>,

  // working in adding support for flattened enums in ts-rs
  #[serde(flatten)]
  #[ts(skip)]
  variant: Variant,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[ts(rename = "EventVariant")]
#[serde(tag = "kind", content = "payload")]
pub enum Variant {
  #[serde(rename = "listener.start")]
  AudioListenerStart(AudioListenerStart),
  #[serde(rename = "listener.end")]
  AudioListenerEnd(AudioListenerEnd),
}

impl From<Variant> for Event {
  fn from(variant: Variant) -> Self {
    Event {
      id: Event::uid(),
      created_at: Utc::now(),
      variant,
    }
  }
}

impl From<AudioListenerStart> for Event {
  fn from(value: AudioListenerStart) -> Self {
    Event::from(Variant::AudioListenerStart(value))
  }
}

impl From<AudioListenerEnd> for Event {
  fn from(value: AudioListenerEnd) -> Self {
    Event::from(Variant::AudioListenerEnd(value))
  }
}

impl Event {
  pub async fn dispatch(event: impl Into<Event>) -> Result<Event, mongodb::error::Error> {
    let event = event.into();
    Self::insert(&event).await?;
    Ok(event)
  }

  pub async fn watch() -> Result<tokio::sync::broadcast::Receiver<Arc<Event>>, mongodb::error::Error>
  {
    let mut watcher = WATCHER.lock().await;

    if watcher.is_none() {
      let cl = Self::cl();

      let options = FindOptions::builder()
        .cursor_type(mongodb::options::CursorType::Tailable)
        .build();

      let mut cursor = cl
        .find(
          doc! { "createdAt": { "$gt": bson::DateTime::now() } },
          options,
        )
        .await?;

      let (tx, rx) = tokio::sync::broadcast::channel(1024);

      let _ = watcher.insert(Watcher { tx: tx.clone() });

      tokio::spawn(async move {
        trace!("starting event cursor");

        loop {
          match cursor.next().await {
            None => {
              warn!("event cursor unexpectedly ended");
              break;
            }
            Some(item) => match item {
              Err(e) => {
                warn!("event cursor unexpectedly errored: {e} => {e:?}");
                break;
              }
              Ok(event) => match tx.send(Arc::new(event)) {
                Ok(_) => continue,
                Err(_e) => {
                  trace!("dropping event cursor: no receivers")
                }
              },
            },
          }
        }

        *WATCHER.lock().await = None;
      });

      Ok(rx)
    } else {
      let watcher = watcher.as_ref().unwrap();
      Ok(watcher.tx.subscribe())
    }
  }
}

#[async_trait]
impl Model for Event {
  const CL_NAME: &'static str = "events";
  const UID_LEN: usize = 24;

  async fn ensure_collection() -> Result<(), mongodb::error::Error> {
    let db = db();
    let names = db.list_collection_names(None).await?;
    let exists = names.iter().any(|name| name == Self::CL_NAME);
    if !exists {
      // 100 MB or 100K docs max,
      let options = CreateCollectionOptions::builder()
        .capped(true)
        .size(100_000_000)
        .max(100_000)
        .build();

      db.create_collection(Self::CL_NAME, options).await?;
    }

    Self::ensure_indexes().await?;
    Ok(())
  }

  fn indexes() -> Vec<IndexModel> {
    let kind = IndexModel::builder().keys(doc! { "kind": 1 }).build();
    let created_at = IndexModel::builder().keys(doc! { "created_at": 1 }).build();
    vec![kind, created_at]
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/event-payload/")]
#[serde(rename_all = "camelCase")]
pub struct AudioListenerStart {
  account_id: String,
  connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/event-payload/")]
#[serde(rename_all = "camelCase")]
pub struct AudioListenerEnd {
  account_id: String,
  connection_id: String,
}

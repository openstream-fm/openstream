use super::lite::StreamConnectionLite;
use super::stats::Stats;
use crate::stream_connection::stats::StatsItem;
use crate::Model;
use deepsize::DeepSizeOf;
use futures_util::TryStreamExt;
use geoip::CountryCode;
use mongodb::bson::{doc, Timestamp};
use mongodb::change_stream::event::OperationType;
use mongodb::options::{ChangeStreamOptions, FullDocumentType};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

// use const_str::concat as str;

#[derive(Debug, Clone)]
pub struct MemIndex {
  map: Arc<RwLock<HashMap<String, Item>>>,
}

#[derive(Debug, Clone, deepsize::DeepSizeOf)]
pub struct Item {
  pub station_id: u64,
  pub is_open: bool,
  pub ip: u64,
  pub country_code: Option<CountryCode>,
  pub created_at_secs: u32,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[macros::keys]
// pub struct ProjectionItem {
//   #[serde(rename = "u")]
//   pub id: String,

//   #[serde(rename = "s")]
//   pub station_id: String,

//   #[serde(rename = "x")]
//   pub is_open: bool,

//   #[serde(rename = "i")]
//   #[serde(with = "serde_util::ip")]
//   pub ip: IpAddr,

//   #[serde(rename = "c")]
//   pub country_code: Option<String>,

//   #[serde(rename = "d")]
//   pub created_at: serde_util::DateTime,
// }

// impl ProjectionItem {
//   pub fn into_parts(self) -> (String, Item) {
//     (
//       self.id,
//       Item {
//         station_id: self.station_id,
//         is_open: self.is_open,
//         ip: self.ip,
//         country_code: self.country_code,
//         created_at_secs: self.created_at.unix_timestamp() as u32,
//       },
//     )
//   }
// }

pub fn split_id_item(conn: StreamConnectionLite) -> (String, Item) {
  (
    conn.id,
    Item {
      station_id: hash(&conn.station_id),
      is_open: conn.is_open,
      ip: hash(&conn.ip),
      country_code: conn.country_code,
      created_at_secs: conn.created_at.unix_timestamp() as u32,
    },
  )
}

impl From<StreamConnectionLite> for Item {
  fn from(conn: StreamConnectionLite) -> Self {
    Self {
      station_id: hash(&conn.station_id),
      is_open: conn.is_open,
      ip: hash(&conn.ip),
      country_code: conn.country_code,
      created_at_secs: conn.created_at.unix_timestamp() as u32,
    }
  }
}

// impl std::ops::Deref for MemIndex {
//   type Target = Arc<RwLock<HashMap<String, Item>>>;
//   fn deref(&self) -> &Self::Target {
//     &self.map
//   }
// }

#[derive(Debug, Default)]
struct ProcessItem {
  sessions: u64,
  ips: HashSet<u64>,
  country_sessions: HashMap<CountryCode, u64>,
  country_ips: HashMap<CountryCode, HashSet<u64>>,
}

impl From<ProcessItem> for StatsItem {
  fn from(v: ProcessItem) -> Self {
    Self {
      sessions: v.sessions as f64,
      ips: v.ips.len() as f64,
      country_sessions: v
        .country_sessions
        .into_iter()
        .map(|(k, v)| (k, v as f64))
        .collect(),
      country_ips: v
        .country_ips
        .into_iter()
        .map(|(k, v)| (k, v.len() as f64))
        .collect(),
    }
  }
}

#[inline(always)]
fn add(item: &mut ProcessItem, conn: &Item) {
  item.sessions += 1;
  item.ips.insert(conn.ip);
  if let Some(code) = &conn.country_code {
    *item.country_sessions.entry(*code).or_insert(0) += 1;
    item
      .country_ips
      .entry(*code)
      .or_insert_with(Default::default)
      .insert(conn.ip);
  }
}

impl MemIndex {
  pub async fn new() -> Self {
    let map = Arc::new(RwLock::new(HashMap::<String, Item>::new()));
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn({
      let map = map.clone();
      async move {
        let clear_task = async {
          let mut last = Instant::now();
          let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
          loop {
            interval.tick().await;

            if Arc::strong_count(&map) == 1 {
              return;
            }

            if last.elapsed().as_secs() < 120 {
              continue;
            }

            let remove_since: time::OffsetDateTime =
              time::OffsetDateTime::now_utc() - time::Duration::DAY * 30;
            let remove_since = remove_since.unix_timestamp() as u32;

            let (before, after) = {
              let mut lock = map.write().await;
              let before = lock.len();
              lock.retain(|_, item| item.created_at_secs > remove_since);
              let after = lock.len();
              (before, after)
            };

            log::info!(
              "stream connection stats index clear: removed {} items, left {} items",
              before - after,
              after,
            );

            log::info!(
              "stream connection stats index size: {}",
              human_bytes::human_bytes(map.read().await.deep_size_of() as f64),
            );

            last = Instant::now();
          }
        };

        let populate_task = async {
          let mut loop_time = 0usize;
          'root: loop {
            macro_rules! loop_try {
              ($e:expr, $message:expr) => {
                match $e {
                  Err(e) => {
                    log::error!("stream stats index error ({}): {} {:?}", $message, e, e);
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

            log::info!("stream connection indexing start, loop {loop_time}");

            let indexing_start = Instant::now();
            let indexing_start_timestamp = Timestamp {
              time: DateTime::now().unix_timestamp() as u32,
              increment: 0,
            };

            {
              let mut lock = map.write().await;

              // resetting the map
              *lock = HashMap::new();

              if loop_time == 1 {
                let _ = tx.send(());
              }

              let one_month_ago = time::OffsetDateTime::now_utc() - time::Duration::DAY * 30;

              let filter = doc! {
                "$or": [
                  { StreamConnectionLite::KEY_CREATED_AT: { "$gte": serde_util::DateTime::from(one_month_ago) } },
                  { StreamConnectionLite::KEY_IS_OPEN: true },
                ]
              };

              let options = mongodb::options::FindOptions::builder().build();

              let mut cursor = loop_try!(
                StreamConnectionLite::cl().find(filter, options).await,
                "cl::find"
              );

              while let Some(doc) = loop_try!(cursor.try_next().await, "cl::find::cursor::try_next")
              {
                let (id, item) = split_id_item(doc);
                lock.insert(id, item);
              }

              log::info!(
                "stream connection stats indexing end: {} items in {}ms",
                lock.len(),
                indexing_start.elapsed().as_millis()
              );

              log::info!(
                "stream connection stats index size: {}",
                human_bytes::human_bytes(lock.deep_size_of() as f64),
              )
            }

            let options = ChangeStreamOptions::builder()
              .full_document(Some(FullDocumentType::UpdateLookup))
              .start_at_operation_time(Some(indexing_start_timestamp))
              .build();

            let mut cursor = loop_try!(
              StreamConnectionLite::cl().watch([], options).await,
              "cl::watch"
            );
            while let Some(event) =
              loop_try!(cursor.try_next().await, "cl::watch::cursor::try_next")
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
                        "stream connection stats: event.document_key is None for {:#?}",
                        event
                      );
                    }
                    Some(doc) => match mongodb::bson::from_document::<DocumentKey>(doc.clone()) {
                      Err(e) => {
                        log::warn!("stream connection stats: failed to deserialize event.document_key: {}, {:?}: {:?}", e, e, doc);
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
                        "stream connection stats: event without full document: {:?}",
                        event
                      );
                    }

                    Some(doc) => {
                      let key = doc.id.clone();
                      let item: Item = doc.into();
                      map.write().await.insert(key, item);
                    }
                  }
                }

                OperationType::Invalidate | OperationType::Drop | OperationType::DropDatabase => {
                  log::warn!(
                    "stream connection stats, operation {:?} received, sleeping and reseting index",
                    event.operation_type
                  );
                  tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                  continue 'root;
                }

                op => {
                  log::error!(
                    "stream connection stats, unknown operation_type {:?}, resetting cursor",
                    op
                  );
                  continue 'root;
                }
              }
            }
          }
        };

        tokio::join!(populate_task, clear_task);
      }
    });

    let _ = rx.recv().await;

    Self { map }
  }

  pub async fn get_stats<F: Filter + Send + Sync + 'static>(&self, filter: F) -> Stats {
    let start = Instant::now();
    log::info!("get stats started");

    let mut now = ProcessItem::default();
    let mut last_24h = ProcessItem::default();
    let mut last_7d = ProcessItem::default();
    let mut last_30d = ProcessItem::default();

    let date_now = time::OffsetDateTime::now_utc();
    let ago_24h =
      time::OffsetDateTime::unix_timestamp(date_now - (time::Duration::HOUR * 24)) as u32;
    let ago_7d = time::OffsetDateTime::unix_timestamp(date_now - (time::Duration::DAY * 7)) as u32;
    let ago_30d =
      time::OffsetDateTime::unix_timestamp(date_now - (time::Duration::DAY * 30)) as u32;

    let me = self.clone();
    tokio::task::spawn_blocking(move || {
      let lock = me.map.blocking_read();

      for conn in lock.values() {
        if !filter.filter(conn) {
          continue;
        }

        if conn.is_open {
          add(&mut now, conn);
        }

        if conn.created_at_secs > ago_30d {
          add(&mut last_30d, conn);
          if conn.created_at_secs > ago_7d {
            add(&mut last_7d, conn);
            if conn.created_at_secs > ago_24h {
              add(&mut last_24h, conn);
            }
          }
        }
      }

      let total = last_30d.sessions;
      let sample = total;
      let multiplier = 1.0;
      let sampled = false;

      let stats = Stats {
        total,
        sample,
        multiplier,
        sampled,
        now: now.into(),
        last_24h: last_24h.into(),
        last_7d: last_7d.into(),
        last_30d: last_30d.into(),
      };

      log::info!(
        "get stats ended: {} items in {}ms",
        total,
        start.elapsed().as_millis()
      );

      stats
    })
    .await
    .unwrap()
  }

  pub async fn get_stats_item<F: Filter + Send + Sync + 'static>(&self, filter: F) -> StatsItem {
    let start = Instant::now();
    log::info!("get stats item started");

    let mut item = ProcessItem::default();

    let me = self.clone();
    tokio::task::spawn_blocking(move || {
      let lock = me.map.blocking_read();

      for conn in lock.values() {
        if !filter.filter(conn) {
          continue;
        }
        add(&mut item, conn);
      }

      log::info!(
        "get stats item ended: {} items in {}ms",
        item.sessions,
        start.elapsed().as_millis()
      );

      item.into()
    })
    .await
    .unwrap()
  }
}

pub trait Filter {
  fn filter(&self, item: &Item) -> bool;
}

impl<F: Fn(&Item) -> bool> Filter for F {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    (self)(item)
  }
}

#[derive(Debug, Clone, Copy)]
pub struct IsOpenFilter(pub bool);

impl Filter for IsOpenFilter {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    item.is_open == self.0
  }
}

#[derive(Debug, Clone, Copy)]
pub struct AllFilter;

impl Filter for AllFilter {
  #[inline(always)]
  fn filter(&self, _item: &Item) -> bool {
    true
  }
}

#[derive(Debug, Clone)]
pub struct StationIdFilter(u64);

impl StationIdFilter {
  pub fn new(station_id: String) -> Self {
    Self(hash(&station_id))
  }
}

impl Filter for StationIdFilter {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    item.station_id == self.0
  }
}

#[derive(Debug, Clone)]
pub struct StationIdSetFilter(HashSet<u64>);

impl StationIdSetFilter {
  pub fn new(set: HashSet<String>) -> Self {
    let mapped = set.into_iter().map(|id| hash(&id)).collect();
    Self(mapped)
  }
}

impl Filter for StationIdSetFilter {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    self.0.contains(&item.station_id)
  }
}

impl<A: Filter, B: Filter> Filter for (A, B) {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    self.0.filter(item) && self.1.filter(item)
  }
}

impl<A: Filter, B: Filter, C: Filter> Filter for (A, B, C) {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    self.0.filter(item) && self.1.filter(item) && self.2.filter(item)
  }
}

#[derive(Debug, Clone)]
pub struct OrFilter<A, B>(pub A, pub B);

impl<A: Filter, B: Filter> Filter for OrFilter<A, B> {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    self.0.filter(item) || self.1.filter(item)
  }
}

#[derive(Debug, Clone)]
pub struct ArgsFnFilter<A, F>(pub A, pub F);

impl<A, F: Fn(&A, &Item) -> bool> Filter for ArgsFnFilter<A, F> {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    (self.1)(&self.0, item)
  }
}

fn hash<T: std::hash::Hash>(t: &T) -> u64 {
  use std::hash::Hasher;
  let mut s = std::collections::hash_map::DefaultHasher::new();
  t.hash(&mut s);
  s.finish()
}

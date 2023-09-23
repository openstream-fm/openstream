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
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

// use const_str::concat as str;

#[derive(Debug, Clone)]
pub struct MemIndex {
  map: Arc<RwLock<MultiMap>>,
}

#[derive(Debug, Clone, deepsize::DeepSizeOf)]
pub struct Item {
  pub station_id: u64,
  pub is_open: bool,
  pub ip: u64,
  pub country_code: Option<CountryCode>,
  pub created_at_secs: u32,
}

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
  // ips: HashSet<u64>,
  country_sessions: CountryCodeMap<u32>,
  // country_ips: HashMap<CountryCode, HashSet<u64>>,
}

impl From<ProcessItem> for StatsItem {
  fn from(v: ProcessItem) -> Self {
    Self {
      sessions: v.sessions as f64,
      country_sessions: v.country_sessions.into_btree_map_with(|v| v as f64),
      // ips: v.ips.len() as f64,
      // country_ips: v
      //   .country_ips
      //   .into_iter()
      //   .map(|(k, v)| (k, v.len() as f64))
      //   .collect(),
    }
  }
}

#[inline(always)]
fn add(item: &mut ProcessItem, conn: &Item) {
  item.sessions += 1;
  // item.ips.insert(conn.ip);
  if let Some(code) = conn.country_code {
    *item.country_sessions.get_mut(code) += 1;
    // item
    //   .country_ips
    //   .entry(code)
    //   .or_insert_with(Default::default)
    //   .insert(conn.ip);
  }
}

#[derive(Debug, Default, deepsize::DeepSizeOf)]
pub struct MultiMap {
  pub primary: HashMap<String, Item>,
  pub by_station_id: HashMap<u64, HashMap<String, Item>>,
}

impl MultiMap {
  pub fn len(&self) -> usize {
    self.primary.len()
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn insert(&mut self, id: String, item: Item) -> Option<Item> {
    self.primary.insert(id.clone(), item.clone());
    self
      .by_station_id
      .entry(item.station_id)
      .or_default()
      .insert(id, item)
  }

  pub fn remove(&mut self, id: &String) -> Option<Item> {
    if let Some(item) = self.primary.remove(id) {
      if let Some(map) = self.by_station_id.get_mut(&item.station_id) {
        let prev = map.remove(id);
        if map.is_empty() {
          self.by_station_id.remove(&item.station_id);
        }
        prev
      } else {
        None
      }
    } else {
      None
    }
  }

  pub fn retain<F: Fn(&Item) -> bool>(&mut self, filter: F) {
    let mut ids = vec![];

    for (_, map) in self.by_station_id.iter() {
      for (id, item) in map {
        if !filter(item) {
          ids.push(id.clone());
        }
      }
    }

    for id in ids {
      self.remove(&id);
    }
  }
}

impl MemIndex {
  pub async fn new() -> Self {
    let map = Arc::new(RwLock::new(MultiMap::default()));
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
              lock.retain(|item| item.created_at_secs > remove_since);
              let after = lock.len();
              (before, after)
            };

            log::info!(
              target: "stream_stats",
              "stream connection stats index clear: removed {} items, left {} items",
              before - after,
              after,
            );

            log::info!(
              target: "stream_stats",
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
                    log::error!(
                      target: "stream_stats",
                      "stream stats index error ({}): {} {:?}", $message, e, e
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
              target: "stream_stats",
              "stream connection indexing start, loop {loop_time}"
            );

            let indexing_start = Instant::now();
            let indexing_start_timestamp = Timestamp {
              time: DateTime::now().unix_timestamp() as u32,
              increment: 0,
            };

            {
              let mut lock = map.write().await;

              // resetting the map
              *lock = MultiMap::default();

              if loop_time == 1 {
                let _ = tx.send(());
              }

              let one_month_ago = time::OffsetDateTime::now_utc() - time::Duration::DAY * 30;

              let filter = doc! {
                "$and": [
                  {
                    "$or": [
                      { StreamConnectionLite::KEY_DURATION_MS: null },
                      { StreamConnectionLite::KEY_DURATION_MS: { "$gte": 5000 } }
                    ]
                  },
                  {
                    "$or": [
                      { StreamConnectionLite::KEY_CREATED_AT: { "$gte": serde_util::DateTime::from(one_month_ago) } },
                      { StreamConnectionLite::KEY_IS_OPEN: true },
                    ]
                  }
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
                target: "stream_stats",
                "stream connection stats indexing end: {} items in {}ms",
                lock.len(),
                indexing_start.elapsed().as_millis()
              );

              log::info!(
                target: "stream_stats",
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
                        target: "stream_stats",
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
                        target: "stream_stats",
                        "stream connection stats: event without full document: {:?}",
                        event
                      );
                    }

                    Some(doc) => {
                      // ignore sessions that are finished and had last less that 5_000ms
                      let retain = doc.duration_ms.is_none() || doc.duration_ms.unwrap() > 5_000;
                      if retain {
                        map.write().await.insert(doc.id.clone(), doc.into());
                      } else {
                        map.write().await.remove(&doc.id);
                      }
                    }
                  }
                }

                OperationType::Invalidate | OperationType::Drop | OperationType::DropDatabase => {
                  log::warn!(
                    target: "stream_stats",
                    "stream connection stats, operation {:?} received, sleeping and reseting index",
                    event.operation_type
                  );
                  tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                  continue 'root;
                }

                op => {
                  log::error!(
                    target: "stream_stats",
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

  pub async fn get_stats<F: Filter + Send + Sync + 'static>(
    &self,
    station_query: StationQuery,
    filter: F,
  ) -> Stats {
    let start = Instant::now();

    let mut now = ProcessItem::default();
    let mut last_24h = ProcessItem::default();
    let mut last_7d = ProcessItem::default();
    let mut last_30d = ProcessItem::default();

    let date_now = time::OffsetDateTime::now_utc();
    let ago_24h =
      time::OffsetDateTime::unix_timestamp(date_now - (time::Duration::HOUR * 24)) as u32;
    let ago_7d = time::OffsetDateTime::unix_timestamp(date_now - (time::Duration::DAY * 7)) as u32;
    let _ago_30d =
      time::OffsetDateTime::unix_timestamp(date_now - (time::Duration::DAY * 30)) as u32;

    let me = self.clone();
    tokio::task::spawn_blocking(move || {
      let lock = me.map.blocking_read();

      macro_rules! add_filtered {
        ($item:ident) => {
          if filter.filter($item) {
            if $item.is_open {
              add(&mut now, $item);
            }

            // TODO: re-add this?
            // if conn.created_at_secs > ago_30d {
            add(&mut last_30d, $item);
            if $item.created_at_secs > ago_7d {
              add(&mut last_7d, $item);
              if $item.created_at_secs > ago_24h {
                add(&mut last_24h, $item);
              }
            }
          }
        };
      }

      match &station_query {
        StationQuery::All => {
          for item in lock.primary.values() {
            add_filtered!(item);
          }
        }

        StationQuery::One { hashed, .. } => {
          if let Some(map) = lock.by_station_id.get(hashed) {
            for item in map.values() {
              add_filtered!(item)
            }
          }
        }

        StationQuery::Some { hashed, .. } => {
          for station_id in hashed {
            if let Some(map) = lock.by_station_id.get(station_id) {
              for item in map.values() {
                add_filtered!(item)
              }
            }
          }
        }
      }

      let total = last_30d.sessions;

      let stats = Stats {
        now: now.into(),
        last_24h: last_24h.into(),
        last_7d: last_7d.into(),
        last_30d: last_30d.into(),
      };

      log::info!(
        target: "stream_stats",
        "stats get, station({}) filter({}) => {} items in {}ms",
        station_query,
        filter,
        total,
        start.elapsed().as_millis()
      );

      stats
    })
    .await
    .unwrap()
  }

  pub async fn get_stats_item<F: Filter + Send + Sync + 'static>(
    &self,
    station_query: StationQuery,
    filter: F,
  ) -> StatsItem {
    let start = Instant::now();

    let mut item = ProcessItem::default();

    let me = self.clone();
    tokio::task::spawn_blocking(move || {
      let lock = me.map.blocking_read();

      macro_rules! add_filtered {
        ($conn:ident) => {
          if filter.filter($conn) {
            add(&mut item, $conn);
          }
        };
      }

      match &station_query {
        StationQuery::All => {
          for conn in lock.primary.values() {
            add_filtered!(conn);
          }
        }

        StationQuery::One { hashed, .. } => {
          if let Some(map) = lock.by_station_id.get(hashed) {
            for conn in map.values() {
              add_filtered!(conn)
            }
          }
        }

        StationQuery::Some { hashed, .. } => {
          for station_id in hashed {
            if let Some(map) = lock.by_station_id.get(station_id) {
              for conn in map.values() {
                add_filtered!(conn)
              }
            }
          }
        }
      }

      log::info!(
        target: "stream_stats",
        "stats get item, station({}) filter({}) => {} items in {}ms",
        station_query,
        filter,
        item.sessions,
        start.elapsed().as_millis()
      );

      item.into()
    })
    .await
    .unwrap()
  }

  pub async fn count<F: Filter + Send + Sync + 'static>(
    &self,
    station_query: StationQuery,
    filter: F,
  ) -> usize {
    let start = Instant::now();
    let me = self.clone();
    let total = tokio::task::spawn_blocking(move || {
      let lock = me.map.blocking_read();

      let mut sum: usize = 0;

      macro_rules! add_filtered {
        ($conn:ident) => {
          if filter.filter($conn) {
            sum += 1;
          }
        };
      }

      match &station_query {
        StationQuery::All => {
          for conn in lock.primary.values() {
            add_filtered!(conn);
          }
        }

        StationQuery::One { hashed, .. } => {
          if let Some(map) = lock.by_station_id.get(hashed) {
            for conn in map.values() {
              add_filtered!(conn)
            }
          }
        }

        StationQuery::Some { hashed, .. } => {
          for station_id in hashed {
            if let Some(map) = lock.by_station_id.get(station_id) {
              for conn in map.values() {
                add_filtered!(conn)
              }
            }
          }
        }
      }

      log::info!(
        target: "stream_stats",
        "stats count, station({}) filter({}) => {} items in {}ms",
        station_query,
        filter,
        sum,
        start.elapsed().as_millis()
      );

      sum
    })
    .await
    .unwrap();

    #[allow(clippy::let_and_return)]
    total
  }

  pub async fn count_by_station<F: Filter + Send + Sync + 'static>(
    &self,
    station_ids: HashSet<String>,
    filter: F,
  ) -> HashMap<String, u32> {
    let me = self.clone();
    tokio::task::spawn_blocking(move || {
      let lock = me.map.blocking_read();
      let mut map = HashMap::with_capacity(station_ids.len());

      for station_id in station_ids.into_iter() {
        let n = {
          match lock.by_station_id.get(&hash(&station_id)) {
            None => 0_u32,
            Some(map) => {
              if filter.is_all() {
                map.len() as u32
              } else {
                let mut n = 0_u32;

                for conn in map.values() {
                  if filter.filter(conn) {
                    n = n.saturating_add(1);
                  }
                }
                n
              }
            }
          }
        };

        map.insert(station_id, n);
      }

      map
    })
    .await
    .unwrap()
  }
}

pub trait Filter: std::fmt::Display {
  fn filter(&self, item: &Item) -> bool;

  fn is_all(&self) -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy)]
pub struct AllFilter;

impl Filter for AllFilter {
  #[inline(always)]
  fn filter(&self, _item: &Item) -> bool {
    true
  }

  #[inline(always)]
  fn is_all(&self) -> bool {
    true
  }
}

impl std::fmt::Display for AllFilter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "all")
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

impl std::fmt::Display for IsOpenFilter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      true => write!(f, "is_open"),
      false => write!(f, "is_closed"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct StationIdFilter {
  station_id: String,
  hash: u64,
}

impl StationIdFilter {
  pub fn new(station_id: String) -> Self {
    let hash = hash(&station_id);
    Self { station_id, hash }
  }
}

impl std::fmt::Display for StationIdFilter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "station_id={}", self.station_id)
  }
}

impl Filter for StationIdFilter {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    item.station_id == self.hash
  }
}

#[derive(Debug, Clone)]
pub struct StationIdSetFilter {
  station_ids: HashSet<String>,
  hashes: HashSet<u64>,
}

impl StationIdSetFilter {
  pub fn new(set: HashSet<String>) -> Self {
    let hashes = set.iter().map(|id| hash(&id)).collect();
    Self {
      station_ids: set,
      hashes,
    }
  }
}

impl std::fmt::Display for StationIdSetFilter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "station_id=in(")?;
    for (i, station_id) in self.station_ids.iter().enumerate() {
      if i == 0 {
        write!(f, "{}", station_id)?;
      } else {
        write!(f, ",{}", station_id)?;
      }
    }

    write!(f, ")")
  }
}

impl Filter for StationIdSetFilter {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    self.hashes.contains(&item.station_id)
  }
}

pub struct AndFilter<A: Filter, B: Filter>(pub A, pub B);

impl<A: Filter, B: Filter> std::fmt::Display for AndFilter<A, B> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({} and {})", self.0, self.1)
  }
}

impl<A: Filter, B: Filter> Filter for AndFilter<A, B> {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    self.0.filter(item) && self.1.filter(item)
  }
}

#[derive(Debug, Clone)]
pub struct OrFilter<A: Filter, B: Filter>(pub A, pub B);

impl<A: Filter, B: Filter> std::fmt::Display for OrFilter<A, B> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({} or {})", self.0, self.1)
  }
}

impl<A: Filter, B: Filter> Filter for OrFilter<A, B> {
  #[inline(always)]
  fn filter(&self, item: &Item) -> bool {
    self.0.filter(item) || self.1.filter(item)
  }
}

#[derive(Debug, Clone)]
pub struct SinceFilter {
  secs: u32,
  ts: u32,
}

impl SinceFilter {
  pub fn new(duration: time::Duration) -> Self {
    let date = time::OffsetDateTime::now_utc() - duration;
    let ts = date.unix_timestamp() as u32;
    let secs = duration.whole_seconds() as u32;
    Self { secs, ts }
  }
}

impl std::fmt::Display for SinceFilter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const MINUTE: u32 = 60;
    const HOUR: u32 = MINUTE * 60;
    const DAY: u32 = HOUR * 24;

    let d = self.secs / DAY;
    let h = (self.secs % DAY) / HOUR;
    let m = (self.secs % HOUR) / MINUTE;
    let s = self.secs % MINUTE;

    if d != 0 {
      if s != 0 {
        write!(f, "last {d} days, {h} hours, {m} mins, {s} secs")
      } else if m != 0 {
        write!(f, "last {d} days, {h} hours, {m} mins")
      } else if h != 0 {
        write!(f, "last {d} days, {h} hours")
      } else {
        write!(f, "last {d} days")
      }
    } else if h != 0 {
      if s != 0 {
        write!(f, "last {h} hours, {m} mins, {s}, secs")
      } else if m != 0 {
        write!(f, "last {h} hours, {m} mins")
      } else {
        write!(f, "last {h} hours")
      }
    } else if m != 0 {
      if s != 0 {
        write!(f, "last {m} mins, {s} secs")
      } else {
        write!(f, "last {m} mins")
      }
    } else {
      write!(f, "last {s} secs")
    }
  }
}

impl Filter for SinceFilter {
  fn filter(&self, item: &Item) -> bool {
    item.created_at_secs >= self.ts
  }
}

fn hash<T: std::hash::Hash>(t: &T) -> u64 {
  use std::hash::Hasher;
  let mut s = std::collections::hash_map::DefaultHasher::new();
  t.hash(&mut s);
  s.finish()
}

#[derive(Debug, Clone)]
pub struct CountryCodeMap<T>([T; 256]);

impl<T: Default> CountryCodeMap<T> {
  pub fn new() -> Self {
    Self(arr_macro::arr![T::default(); 256])
  }
}

impl<T: Default> Default for CountryCodeMap<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Default + Copy + Eq> CountryCodeMap<T> {
  pub fn into_btree_map(self) -> BTreeMap<CountryCode, T> {
    use strum::IntoEnumIterator;
    let mut map = BTreeMap::new();
    for cc in CountryCode::iter() {
      let v = self.0[cc as usize];
      if v != T::default() {
        map.insert(cc, v);
      }
    }
    map
  }

  pub fn into_mapped_btree_map<M: From<T>>(self) -> BTreeMap<CountryCode, M> {
    use strum::IntoEnumIterator;
    let mut map = BTreeMap::new();
    for cc in CountryCode::iter() {
      let v = self.0[cc as usize];
      if v != T::default() {
        map.insert(cc, v.into());
      }
    }
    map
  }

  pub fn into_btree_map_with<M, F: Fn(T) -> M>(self, f: F) -> BTreeMap<CountryCode, M> {
    use strum::IntoEnumIterator;
    let mut map = BTreeMap::new();
    for cc in CountryCode::iter() {
      let v = self.0[cc as usize];
      if v != T::default() {
        map.insert(cc, (f)(v));
      }
    }
    map
  }
}

impl<T> CountryCodeMap<T> {
  #[inline(always)]
  pub fn get(&self, key: CountryCode) -> &T {
    unsafe { self.0.get_unchecked(key as usize) }
  }

  #[inline(always)]
  pub fn get_mut(&mut self, key: CountryCode) -> &mut T {
    unsafe { self.0.get_unchecked_mut(key as usize) }
  }
}

#[derive(Debug, Clone)]
pub enum StationQuery {
  All,
  One {
    station_id: String,
    hashed: u64,
  },
  Some {
    station_ids: HashSet<String>,
    hashed: HashSet<u64>,
  },
}

impl std::fmt::Display for StationQuery {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StationQuery::All => write!(f, "all"),
      StationQuery::One { station_id, .. } => write!(f, "eq={station_id}"),
      StationQuery::Some { station_ids, .. } => {
        write!(f, "in=")?;
        for (i, station_id) in station_ids.iter().enumerate() {
          if i == 0 {
            write!(f, "{}", station_id)?;
          } else {
            write!(f, ",{}", station_id)?;
          }
        }

        write!(f, ")")
      }
    }
  }
}

impl StationQuery {
  pub fn all() -> Self {
    Self::All
  }

  pub fn one(station_id: String) -> Self {
    Self::One {
      hashed: hash(&station_id),
      station_id,
    }
  }

  pub fn some(station_ids: HashSet<String>) -> Self {
    let mut hashed = HashSet::with_capacity(station_ids.len());
    for id in station_ids.iter() {
      hashed.insert(hash(id));
    }
    Self::Some {
      station_ids,
      hashed,
    }
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  use strum::IntoEnumIterator;
  #[test]
  fn country_code_size() {
    for cc in CountryCode::iter() {
      assert!((cc as usize) < 256);
    }
  }
}

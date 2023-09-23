use burst::Burst;
use constants::STREAM_CHANNEL_CAPACITY;
use db::station::{OwnerDeploymentInfo, Station};
use db::Model;
use mongodb::bson::doc;
use parking_lot::{RwLock, RwLockReadGuard, RwLockUpgradableReadGuard, RwLockWriteGuard};
use serde_util::DateTime;
use shutdown::Shutdown;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

pub mod external_relay;
pub mod healthcheck;
pub mod live;
pub mod playlist;
pub mod relay;

use playlist::run_playlist_session;

static UID: AtomicU64 = AtomicU64::new(1);
fn uid() -> u64 {
  UID.fetch_add(1, Ordering::SeqCst)
}

pub struct ReadLock<'a> {
  lock: RwLockReadGuard<'a, Map>,
  #[allow(unused)]
  map: &'a MediaSessionMap,
}

pub struct UpgradableReadLock<'a> {
  lock: RwLockUpgradableReadGuard<'a, Map>,
  map: &'a MediaSessionMap,
}

pub struct WriteLock<'a> {
  lock: RwLockWriteGuard<'a, Map>,
  map: &'a MediaSessionMap,
}

use bytes::Bytes;
use drop_tracer::{DropTracer, Token};
// use std::time::Duration;
use log::*;
use tokio::sync::broadcast;

type Sender = broadcast::Sender<Bytes>;
type Receiver = broadcast::Receiver<Bytes>;

#[derive(Debug, Default)]
pub struct Map {
  inner: BTreeMap<String, MediaSession>,
}

impl Map {
  #[inline]
  pub fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum RestartError {
  #[error("internal error (db)")]
  Db(#[from] mongodb::error::Error),
  #[error("cannot restart, station is live streaming")]
  LiveStreaming,
  #[error("cannot restart, station is streaming from external relay")]
  ExternalRelay,
  #[error("cannot restart, deployment is not owner of relay")]
  Relay,
}

impl<'a> WriteLock<'a> {
  #[inline]
  pub fn get(&self, station_id: &str) -> Option<&MediaSession> {
    self.lock.inner.get(station_id)
  }

  #[inline]
  pub fn entry(&mut self, station_id: &str) -> Entry<'_, String, MediaSession> {
    self.lock.inner.entry(station_id.to_string())
  }

  pub async fn restart(
    &mut self,
    deployment_id: String,
    station_id: String,
    shutdown: Shutdown,
    drop_tracer: DropTracer,
  ) -> Result<(), RestartError> {
    if let Some(session) = self.get(&station_id) {
      match session.kind() {
        MediaSessionKind::Live { .. } => return Err(RestartError::LiveStreaming),
        MediaSessionKind::ExternalRelay => {
          return Err(RestartError::ExternalRelay);
        }
        MediaSessionKind::Relay { .. } => return Err(RestartError::Relay),
        MediaSessionKind::Playlist { .. } => {}
      }
    }

    let task_id = Station::random_owner_task_id();

    let owner_deployment_info = OwnerDeploymentInfo {
      deployment_id: deployment_id.to_string(),
      task_id: task_id.clone(),
      content_type: String::from("audio/mpeg"),
      health_checked_at: Some(DateTime::now()),
    };

    let update = doc! { "$set": { Station::KEY_OWNER_DEPLOYMENT_INFO: owner_deployment_info } };

    Station::update_by_id(&station_id, update).await?;

    let tx = self.transmit(&station_id, &task_id, MediaSessionKind::Playlist {});

    run_playlist_session(
      tx,
      self.map.deployment_id.clone(),
      shutdown,
      drop_tracer,
      false,
    );

    Ok(())
  }

  pub fn transmit(
    &mut self,
    station_id: &str,
    task_id: &str,
    kind: MediaSessionKind,
  ) -> Transmitter {
    let info = Arc::new(MediaSessionInfo {
      uid: uid(),
      station_id: station_id.to_string(),
      task_id: task_id.to_string(),
      kind,
    });

    let is_terminated = Arc::new(AtomicBool::new(false));
    let burst = Arc::new(RwLock::new(Burst::new()));

    let (sender, _) = broadcast::channel(STREAM_CHANNEL_CAPACITY);

    let session = MediaSession {
      info: info.clone(),
      is_terminated: is_terminated.clone(),
      burst: burst.clone(),
      sender: sender.clone(),
    };

    let transmitter = Transmitter {
      info,
      is_terminated,
      burst,
      sender,
      media_sessions_map: self.map.clone(),
    };

    self.lock.inner.insert(station_id.to_string(), session);

    transmitter
  }

  #[inline]
  pub fn terminate(&mut self, station_id: &str) -> bool {
    self.lock.inner.remove(station_id).is_some()
  }
}

impl<'a> ReadLock<'a> {
  #[inline]
  pub fn get(&self, station_id: &str) -> Option<&MediaSession> {
    self.lock.inner.get(station_id)
  }
}

impl<'a> UpgradableReadLock<'a> {
  #[inline]
  pub fn get(&self, station_id: &str) -> Option<&MediaSession> {
    self.lock.inner.get(station_id)
  }

  #[inline]
  pub fn upgrade(self) -> WriteLock<'a> {
    WriteLock {
      lock: RwLockUpgradableReadGuard::upgrade(self.lock),
      map: self.map,
    }
  }
}

#[derive(Debug, Clone)]
pub struct MediaSessionMap {
  pub deployment_id: String,
  pub(crate) map: Arc<RwLock<Map>>,
  pub(crate) drop_tracer: DropTracer,
}

impl MediaSessionMap {
  #[inline]
  pub fn new(deployment_id: String, drop_tracer: DropTracer) -> Self {
    Self {
      map: Default::default(),
      deployment_id,
      drop_tracer,
    }
  }

  #[inline]
  pub fn drop_token(&self) -> Token {
    self.drop_tracer.token()
  }

  #[inline]
  pub fn read(&self) -> ReadLock<'_> {
    ReadLock {
      lock: self.map.read(),
      map: self,
    }
  }

  #[inline]
  pub fn read_recursive(&self) -> ReadLock<'_> {
    ReadLock {
      lock: self.map.read_recursive(),
      map: self,
    }
  }

  #[inline]
  pub fn upgradable_read(&self) -> UpgradableReadLock<'_> {
    UpgradableReadLock {
      lock: self.map.upgradable_read(),
      map: self,
    }
  }

  #[inline]
  pub fn write(&self) -> WriteLock<'_> {
    WriteLock {
      lock: self.map.write(),
      map: self,
    }
  }
}

#[derive(Debug)]
pub struct MediaSession {
  pub(crate) info: Arc<MediaSessionInfo>,
  pub(crate) is_terminated: Arc<AtomicBool>,
  pub(crate) burst: Arc<RwLock<Burst>>,
  pub(crate) sender: Sender,
  // pub(crate) token: Token,
}

impl MediaSession {
  #[inline]
  pub fn subscribe(&self) -> Listener {
    Listener {
      info: self.info.clone(),
      burst: self.burst.read().clone(),
      is_terminated: self.is_terminated.clone(),
      recv: self.sender.subscribe(),
    }
  }

  #[inline]
  pub fn info(&self) -> &MediaSessionInfo {
    &self.info
  }

  #[inline]
  pub fn info_owned(&self) -> Arc<MediaSessionInfo> {
    self.info.clone()
  }

  #[inline]
  pub fn uid(&self) -> u64 {
    self.info.uid
  }

  #[inline]
  pub fn station_id(&self) -> &str {
    &self.info.station_id
  }

  #[inline]
  pub fn kind(&self) -> &MediaSessionKind {
    &self.info.kind
  }

  #[inline]
  pub fn is_live(&self) -> bool {
    self.info.is_live()
  }

  #[inline]
  pub fn is_external_relay(&self) -> bool {
    self.info.is_external_relay()
  }

  #[inline]
  pub fn is_playlist(&self) -> bool {
    self.info.is_playlist()
  }
}

impl Drop for MediaSession {
  fn drop(&mut self) {
    self.is_terminated.store(true, Ordering::SeqCst);
  }
}

#[derive(Debug)]
pub struct MediaSessionInfo {
  pub(crate) uid: u64,
  pub(crate) station_id: String,
  pub(crate) task_id: String,
  pub(crate) kind: MediaSessionKind,
}

impl MediaSessionInfo {
  #[inline]
  pub fn station_id(&self) -> &str {
    &self.station_id
  }

  #[inline]
  pub fn task_id(&self) -> &str {
    &self.task_id
  }

  #[inline]
  pub fn kind(&self) -> &MediaSessionKind {
    &self.kind
  }

  #[inline]
  pub fn is_live(&self) -> bool {
    self.kind.is_live()
  }

  #[inline]
  pub fn is_external_relay(&self) -> bool {
    self.kind.is_external_relay()
  }

  #[inline]
  pub fn is_playlist(&self) -> bool {
    self.kind.is_playlist()
  }

  #[inline]
  pub fn content_type(&self) -> &str {
    self.kind.content_type()
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MediaSessionKind {
  Live { content_type: String },
  Playlist {},
  Relay { content_type: String },
  ExternalRelay,
}

impl MediaSessionKind {
  #[inline]
  pub fn content_type(&self) -> &str {
    match self {
      MediaSessionKind::Live { content_type } => content_type,
      MediaSessionKind::Playlist {} => "audio/mpeg",
      MediaSessionKind::Relay { content_type, .. } => content_type,
      MediaSessionKind::ExternalRelay {} => "audio/mpeg",
    }
  }

  #[inline]
  pub fn is_live(&self) -> bool {
    matches!(self, MediaSessionKind::Live { .. })
  }

  #[inline]
  pub fn is_external_relay(&self) -> bool {
    matches!(self, MediaSessionKind::ExternalRelay { .. })
  }

  #[inline]
  pub fn is_playlist(&self) -> bool {
    matches!(self, MediaSessionKind::Playlist { .. })
  }

  #[inline]
  pub fn is_relay(&self) -> bool {
    matches!(self, MediaSessionKind::Relay { .. })
  }
}

#[derive(Debug)]
pub struct Transmitter {
  info: Arc<MediaSessionInfo>,
  burst: Arc<RwLock<Burst>>,
  is_terminated: Arc<AtomicBool>,
  sender: Sender,
  media_sessions_map: MediaSessionMap,
}

impl Transmitter {
  #[inline]
  pub fn listener_count(&self) -> usize {
    self.sender.receiver_count()
  }

  #[inline]
  pub fn subscribe(&self) -> Listener {
    Listener {
      info: self.info.clone(),
      burst: self.burst.read().clone(),
      is_terminated: self.is_terminated.clone(),
      recv: self.sender.subscribe(),
    }
  }

  #[inline]
  pub fn is_terminated(&self) -> bool {
    self.is_terminated.load(Ordering::SeqCst)
  }

  pub fn send(&self, bytes: Bytes) -> Result<usize, SendError> {
    if self.is_terminated() {
      return Err(SendError::Terminated(bytes));
    };

    {
      let mut burst = self.burst.write();
      if burst.is_full() {
        burst.pop_front();
      }

      // since we just removed one item, this will never fail
      burst.push_back(bytes.clone()).unwrap();
    }

    let n = self.sender.send(bytes)?;

    Ok(n)
  }
}

impl Drop for Transmitter {
  fn drop(&mut self) {
    let mut map = self.media_sessions_map.write();
    if let Entry::Occupied(entry) = map.entry(&self.info.station_id) {
      if entry.get().uid() == self.info.uid {
        entry.remove();
      }
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
  #[error("this media session has been terminated")]
  Terminated(Bytes),
  #[error("this media session does not have active listeners")]
  NoListeners(Bytes),
}

impl From<broadcast::error::SendError<Bytes>> for SendError {
  fn from(e: broadcast::error::SendError<Bytes>) -> Self {
    Self::NoListeners(e.0)
  }
}

#[derive(Debug)]
pub struct Listener {
  info: Arc<MediaSessionInfo>,
  is_terminated: Arc<AtomicBool>,
  burst: Burst,
  recv: Receiver,
}

impl Listener {
  pub async fn recv(&mut self) -> Result<Bytes, RecvError> {
    if let Some(bytes) = self.burst.pop_front() {
      return Ok(bytes);
    }

    let bytes = self.recv.recv().await?;

    Ok(bytes)
  }

  #[inline]
  pub fn is_terminated(&self) -> bool {
    self.is_terminated.load(Ordering::SeqCst)
  }

  #[inline]
  pub fn info(&self) -> &MediaSessionInfo {
    &self.info
  }
}

impl Drop for Listener {
  fn drop(&mut self) {
    // let mut map = self.media_sessions_map.write();
    // if let Entry::Occupied(entry) = map.entry(&self.info.station_id) {
    // if entry.get().uid() == self.info.uid {
    // entry.remove();
    // }
    // }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum RecvError {
  // #[error("this media session has been terminated")]
  // Terminated,
  #[error("this media session does not have active listeners")]
  Closed,
  #[error("this listener is lagged by {0} items")]
  Lagged(u64),
}

impl From<broadcast::error::RecvError> for RecvError {
  fn from(e: broadcast::error::RecvError) -> Self {
    match e {
      broadcast::error::RecvError::Closed => RecvError::Closed,
      broadcast::error::RecvError::Lagged(n) => RecvError::Lagged(n),
    }
  }
}

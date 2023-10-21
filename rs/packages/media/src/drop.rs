use db::station::{OwnerDeploymentInfo, Station};
use db::Model;
use drop_tracer::Token;
use mongodb::bson::doc;

use crate::MediaSessionMap;

#[derive(Debug)]
pub struct OwnerDeploymentDropper(Option<OwnerDeploymentDropperInner>);

impl OwnerDeploymentDropper {
  pub fn new(station_id: String, task_id: String, token: Token) -> Self {
    Self(Some(OwnerDeploymentDropperInner {
      station_id,
      task_id,
      token,
    }))
  }
}

#[derive(Debug)]
struct OwnerDeploymentDropperInner {
  station_id: String,
  task_id: String,
  token: Token,
}

impl Drop for OwnerDeploymentDropper {
  fn drop(&mut self) {
    if let Some(OwnerDeploymentDropperInner {
      station_id,
      task_id,
      token,
    }) = self.0.take()
    {
      tokio::spawn(async move {
        const KEY_OWNER_TASK: &str = db::key!(
          Station::KEY_OWNER_DEPLOYMENT_INFO,
          OwnerDeploymentInfo::KEY_TASK_ID
        );

        let filter = doc! {
          Station::KEY_ID: &station_id,
          KEY_OWNER_TASK: &task_id,
        };

        let update = doc! {
          "$set": {
            Station::KEY_OWNER_DEPLOYMENT_INFO: null,
          }
        };

        let r = Station::cl().update_one(filter, update, None).await;

        if let Err(e) = r {
          log::error!(target: "media", "failed to update station owner task id back to null for station {}, task {}: {}", station_id, task_id, e);
        };

        drop(token);
      });
    }
  }
}

#[derive(Debug)]
pub struct MapEntryRelease(Option<MapEntryReleaseInner>);

impl MapEntryRelease {
  pub fn new(station_id: String, task_id: String, map: MediaSessionMap) -> Self {
    Self(Some(MapEntryReleaseInner {
      station_id,
      task_id,
      map,
    }))
  }
}

#[derive(Debug)]
struct MapEntryReleaseInner {
  station_id: String,
  task_id: String,
  map: MediaSessionMap,
}

impl Drop for MapEntryRelease {
  fn drop(&mut self) {
    if let Some(inner) = self.0.take() {
      tokio::spawn(async move {
        let _ = inner
          .map
          .terminate_task(&inner.station_id, &inner.task_id)
          .await;
      });
    }
  }
}

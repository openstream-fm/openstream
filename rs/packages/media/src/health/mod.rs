use db::{
  media_session::MediaSession,
  station::{OwnerDeploymentInfo, Station},
  Model,
};
use mongodb::bson::doc;
use serde_util::DateTime;

pub async fn check_now() -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
  const KEY_STATION_HEALTH_CHECKED_AT: &str = db::key!(
    Station::KEY_OWNER_DEPLOYMENT_INFO,
    OwnerDeploymentInfo::KEY_HEALTH_CHECKED_AT
  );

  let limit: DateTime = (time::OffsetDateTime::now_utc()
    - time::Duration::seconds(constants::MEDIA_SESSION_HEALTH_SHUTDOWN_TIMEOUT_SECS as i64))
  .into();

  let filter = doc! {
    "$and": [
      { KEY_STATION_HEALTH_CHECKED_AT: { "$ne": null } },
      { KEY_STATION_HEALTH_CHECKED_AT: { "$lt": limit } },
    ],
  };

  let update = doc! { "$set": { Station::KEY_OWNER_DEPLOYMENT_INFO: null } };

  let r = Station::cl().update_many(filter, update, None).await?;

  Ok(r)
}

pub async fn health_shutdown_job() {
  let duration = tokio::time::Duration::from_secs(
    constants::MEDIA_SESSION_HEALTH_CHECK_KILL_INTERVAL_SECS as u64,
  );

  loop {
    match check_now().await {
      Ok(r) => {
        if r.modified_count > 0 {
          log::info!(
            target: "media-session-health",
            "closed owner_deployment_info of {} stations",
            r.modified_count
          )
        }
      }

      Err(e) => {
        log::error!(
          target: "media-session-health",
          "error checking media session health: {} => {}",
          e,
          e,
        )
      }
    }

    tokio::time::sleep(duration).await;
  }
}

pub async fn run_health_check_interval_for_station_and_media_session(
  station_id: &str,
  media_session_id: &str,
  task_id: &str,
) -> Result<std::convert::Infallible, mongodb::error::Error> {
  let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
    constants::MEDIA_SESSION_HEALTH_CHECK_INTERVAL_SECS as u64,
  ));

  loop {
    interval.tick().await;
    let now = DateTime::now();

    {
      const KEY_TASK_ID: &str = db::key!(
        Station::KEY_OWNER_DEPLOYMENT_INFO,
        OwnerDeploymentInfo::KEY_TASK_ID
      );

      const KEY_HEALTH_CHECKED_AT: &str = db::key!(
        Station::KEY_OWNER_DEPLOYMENT_INFO,
        OwnerDeploymentInfo::KEY_HEALTH_CHECKED_AT
      );

      let filter = doc! { Station::KEY_ID: station_id, KEY_TASK_ID: task_id };
      let update = doc! { "$set": { KEY_HEALTH_CHECKED_AT: now } };

      match tokio::spawn(async move { Station::cl().update_one(filter, update, None).await })
        .await
        .unwrap()
      {
        Ok(_) => {}
        Err(e) => {
          log::error!(
            target: "media-session-health",
            "error updating station {}: {} => {}",
            station_id,
            e,
            e,
          );

          return Err(e);
        }
      }
    }

    {
      let update = doc! { "$set": { MediaSession::KEY_HEALTH_CHECKED_AT: now } };
      let id = media_session_id.to_string();
      match tokio::spawn(async move { MediaSession::update_by_id(&id, update).await })
        .await
        .unwrap()
      {
        Ok(_) => {}
        Err(e) => {
          log::error!(
              target: "media-session-health",
              "error updating media session {}: {} => {}",
              media_session_id,
              e,
              e,
          );

          return Err(e);
        }
      };
    }
  }
}

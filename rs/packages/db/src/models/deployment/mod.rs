use crate::{
  account::recalculate_used_listeners_quota,
  run_transaction,
  stream_connection::{lite::StreamConnectionLite, StreamConnection},
  ws_stats_connection::WsStatsConnection,
  Model,
};
use futures_util::TryStreamExt;
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::net::IpAddr;
use ts_rs::TS;

crate::register!(Deployment);

#[allow(clippy::bool_comparison)]
fn is_false(v: &bool) -> bool {
  *v == false
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Deployment {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(with = "serde_util::ip")]
  pub local_ip: IpAddr,

  #[serde(with = "serde_util::u32_as_i64")]
  pub pid: u32,

  #[serde(with = "serde_util::port::vec")]
  #[serde(default)]
  pub source_ports: Vec<u16>,

  #[serde(with = "serde_util::port::vec")]
  #[serde(default)]
  pub stream_ports: Vec<u16>,

  #[serde(with = "serde_util::port::vec")]
  #[serde(default)]
  pub api_ports: Vec<u16>,

  pub state: DeploymentState,

  pub created_at: DateTime,
  pub updated_at: DateTime,

  // TODO: this Option<> is for back compat only
  // create a migration and change this to DateTime
  pub health_checked_at: Option<DateTime>,

  pub dropped_at: Option<DateTime>,

  #[serde(rename = "_m")]
  #[serde(default, skip_serializing_if = "is_false")]
  pub abnormally_closed: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "lowercase")]
#[macros::keys]
pub enum DeploymentState {
  Active,
  Closing,
  Closed,
}

impl Model for Deployment {
  const UID_LEN: usize = 8;
  const CL_NAME: &'static str = "deployments";

  fn indexes() -> Vec<IndexModel> {
    let state = IndexModel::builder()
      .keys(doc! { Deployment::KEY_STATE: 1 })
      .build();

    let created_at = IndexModel::builder()
      .keys(doc! { Deployment::KEY_CREATED_AT: 1 })
      .build();

    let dropped_at = IndexModel::builder()
      .keys(doc! { Deployment::KEY_DROPPED_AT: 1 })
      .build();

    vec![state, created_at, dropped_at]
  }
}

pub async fn check_now() -> Result<(), mongodb::error::Error> {
  let limit: DateTime = (time::OffsetDateTime::now_utc()
    - time::Duration::seconds(constants::DEPLOYMENT_HEALTH_CHECK_SHUTDOWN_DELAY_SECS as i64))
  .into();

  let filter = doc! {
    "$and": [
      { Deployment::KEY_STATE: { "$ne": DeploymentState::KEY_ENUM_VARIANT_CLOSED } },
      { Deployment::KEY_HEALTH_CHECKED_AT: { "$ne": null } },
      { Deployment::KEY_HEALTH_CHECKED_AT: { "$lt": limit } },
    ],
  };

  let mut r = Deployment::cl().find(filter, None).await?;

  while let Some(deployment) = r.try_next().await? {
    log::info!(
      target: "deployment-health",
      "found unclosed deployment {}: closing",
      deployment.id,
    );

    let closed_at = deployment.health_checked_at.unwrap();

    // StreamConnectionLite
    {
      const KEY_CA: &str = const_str::concat!("$", StreamConnectionLite::KEY_CREATED_AT);
      const KEY_DU: &str = const_str::concat!("$", StreamConnectionLite::KEY_DURATION_MS);

      let update = vec![
        doc! {
          "$set": {
            StreamConnectionLite::KEY_IS_OPEN: false,
            StreamConnection::KEY_ABNORNALLY_CLOSED: true,
            StreamConnectionLite::KEY_CLOSED_AT: {
              "$max": [
                {
                  "$dateAdd": {
                    "startDate": KEY_CA,
                    "unit": "millisecond",
                    "amount": 1,
                  }
                },
                closed_at
              ]
            },
          }
        },
        doc! {
          "$set": {
            StreamConnectionLite::KEY_DURATION_MS: {
              "$max": [
                1,
                {
                  "$dateDiff": {
                    "startDate": KEY_CA,
                    "endDate": closed_at,
                    "unit": "millisecond",
                  }
                }
              ]
            }
          }
        },
        doc! {
          "$set": {
            StreamConnectionLite::KEY_TRANSFER_BYTES: {
              "$multiply": [ KEY_DU, 128_000 / 1000 / 8 ]
            }
          }
        },
      ];

      let filter = doc! {
        StreamConnectionLite::KEY_DEPLOYMENT_ID: &deployment.id,
        StreamConnectionLite::KEY_IS_OPEN: true,
      };

      let r = StreamConnectionLite::cl()
        .update_many(filter, update, None)
        .await?;

      log::info!(
        target: "deployment-health",
        "closed {} stream_connections_lite for deployment {}",
        r.matched_count,
        deployment.id,
      );
    };

    // StreamConnection
    {
      const KEY_CREATED_AT: &str = const_str::concat!("$", StreamConnection::KEY_CREATED_AT);
      const KEY_DURATION_MS: &str = const_str::concat!("$", StreamConnection::KEY_DURATION_MS);
      const KEY_CLOSED_AT: &str = const_str::concat!("$", StreamConnection::KEY_CLOSED_AT);

      let update = vec![
        doc! {
          "$set": {
            StreamConnection::KEY_IS_OPEN: false,
            StreamConnection::KEY_ABNORNALLY_CLOSED: true,
            StreamConnection::KEY_CLOSED_AT: {
              "$max": [
                {
                  "$dateAdd": {
                    "startDate": KEY_CREATED_AT,
                    "unit": "millisecond",
                    "amount": 1,
                  }
                },
                closed_at
              ]
            },
          }
        },
        doc! {
          "$set": {
            StreamConnection::KEY_LAST_TRANSFER_AT: KEY_CLOSED_AT,
            StreamConnection::KEY_DURATION_MS: {
              "$max": [
                1,
                {
                  "$dateDiff": {
                    "startDate": KEY_CREATED_AT,
                    "endDate": closed_at,
                    "unit": "millisecond",
                  }
                }
              ]
            }
          }
        },
        doc! {
          "$set": {
            StreamConnection::KEY_TRANSFER_BYTES: {
              "$multiply": [ KEY_DURATION_MS, 128_000 / 1000 / 8 ]
            }
          }
        },
      ];

      let filter = doc! {
        StreamConnection::KEY_DEPLOYMENT_ID: &deployment.id,
        StreamConnection::KEY_IS_OPEN: true,
      };

      let r = StreamConnection::cl()
        .update_many(filter, update, None)
        .await?;

      log::info!(
        target: "deployment-health",
        "closed {} stream_connections for deployment {}",
        r.matched_count,
        deployment.id,
      );
    };

    // WsStatsConnection
    {
      const KEY_CA: &str = const_str::concat!("$", WsStatsConnection::KEY_CREATED_AT);

      let update = vec![doc! {
        "$set": {
          WsStatsConnection::KEY_IS_OPEN: false,
          WsStatsConnection::KEY_ABNORMALLY_CLOSED: true,
          WsStatsConnection::KEY_CLOSED_AT: {
            "$max": [
              {
                "$dateAdd": {
                  "startDate": KEY_CA,
                  "unit": "millisecond",
                  "amount": 1,
                }
              },
              closed_at
            ]
          },
          WsStatsConnection::KEY_DURATION_MS: {
            "$max": [
              1,
              {
                "$dateDiff": {
                  "startDate": KEY_CA,
                  "endDate": closed_at,
                  "unit": "millisecond",
                }
              }
            ]
          }
        }
      }];

      let filter = doc! {
        WsStatsConnection::KEY_DEPLOYMENT_ID: &deployment.id,
        WsStatsConnection::KEY_IS_OPEN: true,
      };

      let r = WsStatsConnection::cl()
        .update_many(filter, update, None)
        .await?;

      log::info!(
        target: "deployment-health",
        "closed {} ws_stats_connections for deployment {}",
        r.matched_count,
        deployment.id,
      );
    };

    // Deployment
    {
      let update = doc! {
        "$set": {
          Deployment::KEY_STATE: DeploymentState::KEY_ENUM_VARIANT_CLOSED,
          Deployment::KEY_ABNORMALLY_CLOSED: true,
          Deployment::KEY_DROPPED_AT: closed_at,
        }
      };

      Deployment::update_by_id(&deployment.id, update).await?;

      log::info!(
        target: "deployment-health",
        "deployment {} marked as closed in database",
        deployment.id,
      );

      log::info!(
        target: "deployment-health",
        "recalculating used listeners quota for all accounts",
      );

      run_transaction!(session => {
        tx_try!(recalculate_used_listeners_quota(&mut session).await);
      });

      log::info!(
        target: "deployment-health",
        "used listeners quota recalculated for all accounts",
      );
    };
  }

  Ok(())
}

pub async fn start_health_check_job(deployment_id: String) {
  let shutdown_task = async {
    let interval_duration = tokio::time::Duration::from_secs(
      constants::DEPLOYMENT_HEALTH_CHECK_SHUTDOWN_INTERVAL_SECS as u64,
    );

    loop {
      match check_now().await {
        Ok(_) => {}
        Err(e) => {
          log::warn!(
            target: "deployment-health",
            "error in deployment health shutdown task: {} => {}",
            e,
            e,)
        }
      }

      tokio::time::sleep(interval_duration).await;
    }
  };

  let keep_alive_task = async {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
      constants::DEPLOYMENT_HEALTH_CHECK_INTERVAL_SECS as u64,
    ));
    loop {
      interval.tick().await;
      let now = serde_util::DateTime::now();
      let update = doc! { "$set": { Deployment::KEY_HEALTH_CHECKED_AT: now } };
      match Deployment::update_by_id(&deployment_id, update).await {
        Ok(_) => continue,
        Err(e) => {
          log::error!(
            target: "deployment-health",
            "error updating deployment {}: {} => {}",
            deployment_id,
            e,
            e,
          )
        }
      };
    }
  };

  tokio::join!(shutdown_task, keep_alive_task);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, Deployment::KEY_ID);
  }
}

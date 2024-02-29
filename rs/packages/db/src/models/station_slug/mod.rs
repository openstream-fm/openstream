use crate::Model;
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

crate::register!(StationSlug);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StationSlug {
  #[serde(rename = "_id")]
  pub id: String,
  pub station_id: String,
  pub slug: String,
  pub created_at: DateTime,
}

impl Model for StationSlug {
  const UID_LEN: usize = 6;
  const CL_NAME: &'static str = "admins";

  fn indexes() -> Vec<IndexModel> {
    let station_id = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1 })
      .build();

    let slug = IndexModel::builder()
      .keys(doc! { Self::KEY_SLUG: 1 })
      .build();

    vec![station_id, slug]
  }
}

impl StationSlug {
  pub async fn is_slug_available_for_station(
    station_id: Option<&str>,
    slug: &str,
  ) -> Result<bool, mongodb::error::Error> {
    let filter = match station_id {
      Some(station_id) => {
        doc! { Self::KEY_SLUG: slug, Self::KEY_STATION_ID: { "$ne": station_id } }
      }
      None => doc! { Self::KEY_SLUG: slug },
    };

    let slug_exists_for_other_station = Self::exists(filter).await?;

    Ok(!slug_exists_for_other_station)
  }

  pub async fn is_slug_available_for_station_with_session(
    station_id: Option<&str>,
    slug: &str,
    session: &mut mongodb::ClientSession,
  ) -> Result<bool, mongodb::error::Error> {
    let filter = match station_id {
      Some(station_id) => {
        doc! { Self::KEY_SLUG: slug, Self::KEY_STATION_ID: { "$ne": station_id } }
      }
      None => doc! { Self::KEY_SLUG: slug },
    };

    let slug_exists_for_other_station = Self::exists_with_session(filter, session).await?;

    Ok(!slug_exists_for_other_station)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, StationSlug::KEY_ID);
  }
}

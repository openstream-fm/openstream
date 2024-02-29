use std::collections::HashSet;

use crate::{run_transaction, station::Station, Model};
use log::info;
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

#[async_trait::async_trait]
impl Model for StationSlug {
  const UID_LEN: usize = 12;
  const CL_NAME: &'static str = "station_slugs";

  fn indexes() -> Vec<IndexModel> {
    let station_id = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1 })
      .build();

    let slug = IndexModel::builder()
      .keys(doc! { Self::KEY_SLUG: 1 })
      .build();

    vec![station_id, slug]
  }

  async fn ensure_collection() -> Result<(), mongodb::error::Error> {
    Self::ensure_indexes().await?;

    // this is a migration from old naive slugs to new ones (v0.47.0)
    run_transaction!(session => {
      let slugs_count = tx_try!(Self::cl().count_documents_with_session(doc!{}, None, &mut session).await);
      if slugs_count != 0 {
        return Ok(());
      }

      let stations_count = tx_try!(Station::cl().count_documents_with_session(doc!{}, None, &mut session).await);
      if stations_count == 0 {
        return Ok(());
      }

      info!(target: "migration", "slug migration v0.47.0 started");

      let mut slugs = HashSet::<String>::new();

      let mut slugs_docs = Vec::<Self>::new();
      let mut station_ids_to_delete_slug = Vec::<String>::new();

      let sort = doc! { Self::KEY_CREATED_AT: 1 };
      let options = mongodb::options::FindOptions::builder().sort(sort).build();
      let mut stations = tx_try!(Station::cl().find_with_session(None, options, &mut session).await);

      let now = DateTime::now();

      while let Some(station) = tx_try!(stations.next(&mut session).await.transpose()) {
        if let Some(slug) = &station.slug {
          if slugs.contains(slug) {
            station_ids_to_delete_slug.push(station.id.clone())
          } else {
            slugs.insert(slug.to_string());
            slugs_docs.push(Self {
              id: Self::uid(),
              station_id: station.id.clone(),
              slug: slug.to_string(),
              created_at: now
            });
          }
        }
      }


      let filter = doc!{ Station::KEY_ID: { "$in": station_ids_to_delete_slug } };
      let update = doc! { "$set": { Station::KEY_SLUG: null } };
      tx_try!(Station::cl().update_many_with_session(filter, update, None, &mut session).await);
      tx_try!(StationSlug::cl().insert_many_with_session(slugs_docs, None, &mut session).await);
    });

    info!(target: "migration", "slug migration v0.47.0 finished");

    Ok(())
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

use crate::{storage_db, Model};
use bytes::Bytes;
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

crate::register!(StationPictureVariant);

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StationPictureVariant {
  #[serde(rename = "_id")]
  pub id: String,
  pub picture_id: String,
  pub format: StationPictureVariantFormat,
  pub size: f64,
  pub size_bytes: f64,
  pub content_type: String,
  #[serde(with = "serde_util::bytes")]
  #[ts(type = "Uint8Array")]
  pub data: Bytes,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

impl Model for StationPictureVariant {
  const CL_NAME: &'static str = "station_picture_variants";
  const UID_LEN: usize = 10;

  fn db() -> mongodb::Database {
    storage_db()
  }

  fn indexes() -> Vec<IndexModel> {
    let composed_id = IndexModel::builder()
      .keys(doc! { Self::KEY_PICTURE_ID: 1, Self::KEY_FORMAT: 1, Self::KEY_SIZE: 1 })
      .options(IndexOptions::builder().unique(true).build())
      .build();

    vec![composed_id]
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "kebab-case")]
pub enum StationPictureVariantFormat {
  Webp,
  Png,
  Source,
}

impl From<StationPictureVariantFormat> for mongodb::bson::Bson {
  fn from(value: StationPictureVariantFormat) -> Self {
    mongodb::bson::to_bson(&value).unwrap()
  }
}

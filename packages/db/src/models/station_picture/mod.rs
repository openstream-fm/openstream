use crate::station_picture_variant::StationPictureVariant;
use crate::Model;
use mongodb::bson::doc;
use mongodb::{ClientSession, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StationPicture {
  #[serde(rename = "_id")]
  pub id: String,

  pub version: f64,

  pub src_filename: String,
  pub src_content_type: String,
  pub src_size: f64,
  pub src_size_bytes: f64,

  pub created_at: DateTime,
  pub updated_at: DateTime,
}

impl Model for StationPicture {
  const CL_NAME: &'static str = "station_pictures";
  const UID_LEN: usize = 8;

  fn indexes() -> Vec<IndexModel> {
    vec![]
  }
}

impl StationPicture {
  pub const VERSION: f64 = 1.0;
  pub const PNG_SIZES: [f64; 2] = [192.0, 512.0];
  pub const WEBP_SIZES: [f64; 5] = [32.0, 64.0, 128.0, 256.0, 512.0];

  pub async fn delete_with_session(
    id: &str,
    session: &mut ClientSession,
  ) -> Result<bool, mongodb::error::Error> {
    let r1 = Self::delete_by_id_with_session(id, session).await?;
    let r2 = StationPictureVariant::cl()
      .delete_many_with_session(
        doc! { StationPictureVariant::KEY_PICTURE_ID: id },
        None,
        session,
      )
      .await?;

    let r = r1.deleted_count != 0 || r2.deleted_count != 0;

    Ok(r)
  }
}

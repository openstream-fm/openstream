use crate::metadata::Metadata;
use crate::Model;
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
  #[serde(rename = "_id")]
  id: String,
  account_id: String,
  name: String,
  description: String,
  port_image: Option<String>,
  square_image: Option<String>,
  user_metadata: Metadata,
  system_metadata: Metadata,
}

impl Model for Station {
  const UID_LEN: usize = 10;
  const CL_NAME: &'static str = "stations";

  fn indexes() -> Vec<IndexModel> {
    let account_id = IndexModel::builder().keys(doc! { "accountId": 1 }).build();
    vec![account_id]
  }
}

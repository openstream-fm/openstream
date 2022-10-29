use crate::Model;
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
}

impl Model for Station {
  fn uid_len() -> usize {
    10
  }
  fn cl_name() -> &'static str {
    "stations"
  }
}

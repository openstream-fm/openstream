use crate::model;
use serde::{Deserialize, Serialize};

pub const CL_NAME: &str = "stations";
pub const UID_LEN: usize = 10;

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

model!(Station);

use crate::Query;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename = "Query")]
pub struct GenerateClientToken {
  pub customer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "Response")]
pub struct GenerateClientTokenResponse {
  pub client_token: String,
}

impl Query for GenerateClientToken {
  const PATH: &'static str = "/generate-client-token";
  type Response = GenerateClientTokenResponse;
}

crate::export!(GenerateClientToken);

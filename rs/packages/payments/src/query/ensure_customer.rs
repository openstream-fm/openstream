use crate::Query;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename = "Query")]
pub struct EnsureCustomer {
  pub customer_id: String,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "Response")]
pub struct EnsureCustomerResponse {
  pub customer_id: String,
}

impl Query for EnsureCustomer {
  const PATH: &'static str = "/ensure-customer";
  type Response = EnsureCustomerResponse;
}

crate::export!(EnsureCustomer);

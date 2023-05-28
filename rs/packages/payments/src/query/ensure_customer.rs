use crate::Query;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename = "Query")]
pub struct EnsureCustomer {
  customer_id: String,
  email: String,
  first_name: String,
  last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "Response")]
pub struct EnsureCustomerResponse {
  customer_id: String,
}

impl Query for EnsureCustomer {
  const PATH: &'static str = "/ensure-customer";
  type Response = EnsureCustomerResponse;
}

crate::export!(EnsureCustomer);

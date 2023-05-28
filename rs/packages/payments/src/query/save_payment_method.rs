use crate::Query;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename = "Query")]
pub struct SavePaymentMethod {
  customer_id: String,
  payment_method_nonce: String,
  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  device_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "Response")]
pub struct SavePaymentMethodResponse {
  card_type: String,
  expiration_month: Option<String>,
  expiration_year: Option<String>,
  last_4: String,
  payment_method_token: String,
}

impl Query for SavePaymentMethod {
  const PATH: &'static str = "/save-payment-method";
  type Response = SavePaymentMethodResponse;
}

crate::export!(SavePaymentMethod);

use crate::Query;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename = "Query")]
pub struct SavePaymentMethod {
  pub customer_id: String,
  pub payment_method_nonce: String,
  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub device_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "Response")]
pub struct SavePaymentMethodResponse {
  pub card_type: String,
  pub expiration_month: Option<String>,
  pub expiration_year: Option<String>,
  pub last_4: String,
  pub payment_method_token: String,
}

impl Query for SavePaymentMethod {
  const PATH: &'static str = "/save-payment-method";
  type Response = SavePaymentMethodResponse;
}

crate::export!(SavePaymentMethod);

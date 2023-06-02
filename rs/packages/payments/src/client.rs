use reqwest::Client;

use crate::error::{PaymentsErrorPayload, PerformError};
use crate::Query;

#[derive(Debug, Clone)]
pub struct PaymentsClient {
  base_url: String,
  access_token: String,
  client: Client,
}

impl PaymentsClient {
  pub fn new(base_url: String, access_token: String) -> Self {
    Self::new_with_client(base_url, access_token, Client::default())
  }

  pub fn new_with_client(base_url: String, access_token: String, client: Client) -> Self {
    Self {
      base_url: base_url.trim_end_matches('/').to_string(),
      access_token,
      client,
    }
  }

  pub async fn perform<Q: Query>(&self, query: Q) -> Result<Q::Response, PerformError> {
    let res = self
      .client
      .post(format!("{}{}", self.base_url, Q::PATH))
      .json(&query)
      .header("x-access-token", &self.access_token)
      .send()
      .await
      .map_err(PerformError::Fetch)?;

    if res.status().as_u16() == 200 {
      let payload: Q::Response = res.json().await.map_err(PerformError::GetPayload)?;
      Ok(payload)
    } else {
      let status = res.status().as_u16();
      let payload: PaymentsErrorPayload = res
        .json()
        .await
        .map_err(|source| PerformError::GetErrorPayload { status, source })?;

      Err(PerformError::Endpoint(payload.error))
    }
  }
}

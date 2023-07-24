use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait JsonEndpoint {
  type Payload: Serialize + DeserializeOwned;
  type QueryString: Serialize + DeserializeOwned;
  type Params: self::Params;
  type Auth: self::Auth;
}

pub trait Params {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EmptyParams {}

impl Params for EmptyParams {}

#[async_trait]
pub trait Auth {
  async fn from_request() -> Self;
}

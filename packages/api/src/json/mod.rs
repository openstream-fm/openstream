use async_trait::async_trait;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, StatusCode};
use prex::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorCode {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
  status: u16,
  code: ApiErrorCode,
  message: String,
}

impl ApiError {
  fn into_json_response(self) -> Response {
    let mut res =
      Response::new(StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR));

    let body = serde_json::to_vec(&self).expect("ApiError JSON serialize");

    res
      .headers_mut()
      .append(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    res.headers_mut().append(
      CONTENT_LENGTH,
      HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
    );

    *res.body_mut() = Body::from(body);

    res
  }
}

impl Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "ApiError: {:?}: {} => {}",
      self.code, self.status, self.message
    )
  }
}

impl Error for ApiError {}

#[async_trait]
pub trait JsonHandler: Send + Sync + 'static {
  type Input: Send;
  type Output: Serialize;
  type ParseError: Into<ApiError>;
  type HandleError: Into<ApiError>;

  async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError>;

  async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError>;

  async fn handle(&self, request: Request) -> Response {
    let input = match self.parse(request).await {
      Err(e) => return e.into().into_json_response(),
      Ok(input) => input,
    };

    let output = match self.perform(input).await {
      Err(e) => return e.into().into_json_response(),
      Ok(output) => output,
    };

    let mut res = Response::new(StatusCode::OK);
    // TODO: remove this expect
    let body = serde_json::to_vec(&output).expect("JsonHandler JSON serialize");

    res
      .headers_mut()
      .append(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    res.headers_mut().append(
      CONTENT_LENGTH,
      HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
    );

    *res.body_mut() = Body::from(body);

    res
  }
}

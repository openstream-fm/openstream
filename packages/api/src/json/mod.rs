use crate::error::ApiError;
use async_trait::async_trait;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, StatusCode};
use prex::handler::Handler;
use prex::*;
use serde::Serialize;
use serde_json;

#[async_trait]
pub trait JsonHandler: Send + Sync + Sized + 'static {
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

    res.headers_mut().append(
      CONTENT_TYPE,
      HeaderValue::from_static("application/json;charset=utf-8"),
    );

    res.headers_mut().append(
      CONTENT_LENGTH,
      HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
    );

    *res.body_mut() = Body::from(body);

    res
  }

  fn into_handler(self) -> PrexJsonHandler<Self> {
    PrexJsonHandler(self)
  }
}

pub struct PrexJsonHandler<T>(T);

#[async_trait]
impl<T: JsonHandler> Handler for PrexJsonHandler<T> {
  async fn call(&self, req: Request, _next: Next) -> Response {
    self.0.handle(req).await
  }
}

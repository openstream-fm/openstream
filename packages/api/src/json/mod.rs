use crate::error::ApiError;
use async_trait::async_trait;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE, ETAG, IF_NONE_MATCH};
use hyper::http::HeaderValue;
use hyper::{Body, Method, StatusCode};
use log::warn;
use prex::handler::Handler;
use prex::*;
use serde::Serialize;
use serde_json;
use sha1::{Digest, Sha1};

#[async_trait]
pub trait JsonHandler: Send + Sync + Sized + 'static {
  type Input: Send;
  type Output: Serialize;
  type ParseError: Into<ApiError>;
  type HandleError: Into<ApiError>;

  async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError>;

  async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError>;

  async fn handle(&self, req: Request) -> Response {
    let method = req.method().clone();
    let req_etag = req.headers().get(IF_NONE_MATCH).cloned();
    let path = req.uri().path().to_string();

    let input = match self.parse(req).await {
      Err(e) => {
        let err = e.into();
        warn!(
          "APIError (parse): {} {} => {:?}",
          method.to_string(),
          path,
          err
        );
        return err.into_json_response();
      }
      Ok(input) => input,
    };

    let output = match self.perform(input).await {
      Err(e) => {
        let err = e.into();
        warn!(
          "APIError (perform): {} {} => {:?}",
          method.to_string(),
          path,
          err
        );
        return err.into_json_response();
      }
      Ok(output) => output,
    };

    let is_cacheable_method = method == Method::GET || method == Method::HEAD;

    // TODO: remove this expect
    let body = serde_json::to_vec(&output).expect("JsonHandler JSON serialize");

    if is_cacheable_method {
      let req_etag = req_etag.and_then(|etag| etag.to_str().map(String::from).ok());

      let mut hasher = Sha1::new();
      hasher.update(&body);
      let hash = hasher.finalize();

      let res_etag = format!("\"{}\"", hex::encode(hash));

      let etag_match = match req_etag.as_ref() {
        None => false,
        Some(req_etag) => req_etag == &res_etag,
      };

      if etag_match {
        Response::new(StatusCode::NOT_MODIFIED)
      } else {
        let mut res = Response::new(StatusCode::OK);

        res
          .headers_mut()
          .append(ETAG, HeaderValue::from_str(res_etag.as_str()).unwrap());

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
    } else {
      let mut res = Response::new(StatusCode::OK);

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

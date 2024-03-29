use crate::error::ApiError;
use async_trait::async_trait;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE, ETAG, IF_NONE_MATCH, SET_COOKIE};
use hyper::http::HeaderValue;
use hyper::{Body, Method, StatusCode};
use log::warn;
use prex::handler::Handler;
use prex::*;
use serde::Serialize;
use serde_json;
use sha1::{Digest, Sha1};

#[async_trait]
pub trait JsonHandler: Send + Sync + Sized + Clone + 'static {
  type Input: Send;
  type Output: Serialize;
  type ParseError: Into<ApiError> + std::error::Error;
  type HandleError: Into<ApiError> + std::error::Error;

  fn ignore_etag(&self) -> bool {
    false
  }

  fn cookies(&self, _output: &Self::Output) -> Vec<cookie::Cookie> {
    vec![]
  }

  async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError>;

  async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError>;

  /// handle consumes self because the future must be 'static, so we use clone Self on each request
  /// the performance cost of doing this is insignificant
  async fn handle(self, req: Request) -> Response {
    let method = req.method().clone();
    let req_etag = req.headers().get(IF_NONE_MATCH).cloned();
    let path = req.uri().path().to_string();

    let input = match self.parse(req).await {
      Err(e) => {
        let original_err_debug = format!("{:?}", e);
        let err: ApiError = e.into();
        let status = err.status().canonical_reason();
        let code = err.code();
        warn!(
          "APIError (parse): {} {} => {:?} {:?} => {} => {:?}",
          method.as_str(),
          path,
          status,
          code,
          original_err_debug,
          err
        );
        return err.into_json_response();
      }
      Ok(input) => input,
    };

    let output = match self.perform(input).await {
      Err(e) => {
        let original_err_debug = format!("{:?}", e);
        let err: ApiError = e.into();
        let status = err.status().canonical_reason();
        let code = err.code();
        warn!(
          "APIError (perform): {} {} => {:?} {:?} => {} => {:?}",
          method.as_str(),
          path,
          status,
          code,
          original_err_debug,
          err
        );
        return err.into_json_response();
      }
      Ok(output) => output,
    };

    let is_cacheable_method = method == Method::GET || method == Method::HEAD;

    let body = match serde_json::to_vec(&output) {
      Ok(vec) => vec,
      Err(e) => return ApiError::SerializeJSON(e).into_json_response(),
    };

    if is_cacheable_method && !self.ignore_etag() {
      let req_etag = req_etag.and_then(|etag| etag.to_str().map(String::from).ok());

      let mut hasher = Sha1::new();
      hasher.update(&body);
      let hash = hasher.finalize();

      let res_etag = format!("\"{}\"", hex::encode(hash));

      let etag_match = match req_etag {
        None => false,
        Some(ref req_etag) => req_etag == &res_etag,
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
          HeaderValue::from_str(format!("{}", body.len()).as_str()).unwrap(),
        );

        *res.body_mut() = Body::from(body);

        for cookie in self.cookies(&output) {
          res.headers_mut().append(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
          );
        }

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

      for cookie in self.cookies(&output) {
        res.headers_mut().append(
          SET_COOKIE,
          HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
      }

      res
    }
  }

  fn into_handler(self) -> PrexJsonHandler<Self> {
    PrexJsonHandler(self)
  }
}

#[derive(Debug, Clone)]
pub struct PrexJsonHandler<T: JsonHandler>(pub T);

#[async_trait]
impl<T: JsonHandler> Handler for PrexJsonHandler<T> {
  async fn call(&self, req: Request, _: Next) -> Response {
    let me = self.clone();
    // prevent hyper from dropping the future before completion
    tokio::spawn(async move { me.0.handle(req).await })
      .await
      .unwrap()
  }
}

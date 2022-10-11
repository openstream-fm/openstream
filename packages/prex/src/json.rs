use super::response::Response;
use hyper::header::HeaderValue;
use hyper::Body;
use hyper::StatusCode;
use serde::Serialize;
use serde_json;

pub struct Json<T>(pub T);

impl<T: Serialize> Into<Response> for Json<T> {
  fn into(self) -> Response {
    match serde_json::to_string(&self.0) {
      Ok(v) => {
        let mut res = Response::new(StatusCode::OK);
        *res.body_mut() = Body::from(v);
        res
      }

      Err(_e) => {
        let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
        *res.body_mut() = Body::from("500 Internal Server Error (Serialize)");
        res
      }
    }
  }
}

impl<S: Into<StatusCode>, T: Serialize> Into<Response> for (S, Json<T>) {
  fn into(self) -> Response {
    match serde_json::to_string(&self.1 .0) {
      Ok(v) => {
        let mut res = Response::new(self.0.into());
        res.set_content_type(HeaderValue::from_static("application/json"));
        res.set_charset(HeaderValue::from_static("utf-8"));
        *res.body_mut() = Body::from(v);
        res
      }

      Err(_e) => {
        let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
        res.set_content_type(HeaderValue::from_static("text/plain"));
        res.set_charset(HeaderValue::from_static("utf-8"));
        *res.body_mut() = Body::from("500 Internal Server Error (Serialize)");
        res
      }
    }
  }
}

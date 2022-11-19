use super::response::Response;
use hyper::header::HeaderValue;
use hyper::Body;
use hyper::StatusCode;
use serde::Serialize;
use serde_json;

pub struct Json<T>(pub T);

impl<T: Serialize> From<Json<T>> for Response {
  fn from(body: Json<T>) -> Response {
    match serde_json::to_string(&body.0) {
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

impl<S: Into<StatusCode>, T: Serialize> From<(S, Json<T>)> for Response {
  fn from((status, body): (S, Json<T>)) -> Response {
    match serde_json::to_string(&body.0) {
      Ok(v) => {
        let mut res = Response::new(status.into());
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

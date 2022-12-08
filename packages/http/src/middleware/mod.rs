use hyper::{
  header::{HeaderValue, SERVER},
  StatusCode,
};
use prex::{json::Json, Next, Request, Response};
use serde_json::json;

pub async fn server(request: Request, next: Next) -> Response {
  let mut res = next.run(request).await;

  res
    .headers_mut()
    .insert(SERVER, HeaderValue::from_static("openstream"));

  res
}

pub async fn status(_: Request, _: Next) -> Response {
  Response::from((StatusCode::OK, Json(json! {{"ok": true }})))
}

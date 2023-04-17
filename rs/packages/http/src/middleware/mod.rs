use hyper::{
  header::{HeaderValue, SERVER},
  StatusCode,
};
use prex::{json::Json, Next, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Status {
  ok: bool,
}

pub async fn server(request: Request, next: Next) -> Response {
  let mut res = next.run(request).await;

  res.headers_mut().insert(
    SERVER,
    HeaderValue::from_static(concat!("openstream/", env!("CARGO_PKG_VERSION"))),
  );

  res
}

pub async fn status(_: Request, _: Next) -> Response {
  Response::from((StatusCode::OK, Json(Status { ok: true })))
}

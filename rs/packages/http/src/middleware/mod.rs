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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonErrorPayload {
  pub error: JsonError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonError {
  pub status: u16,
  pub message: String,
}

pub async fn server(request: Request, next: Next) -> Response {
  let mut res = next.run(request).await;

  res.headers_mut().insert(
    SERVER,
    HeaderValue::from_static(concat!("openstream/", env!("CARGO_PKG_VERSION"))),
  );

  res
}

#[derive(Debug, thiserror::Error)]
pub enum StatusError {
  #[error("db: {0}")]
  Db(#[from] mongodb::error::Error),
}

impl From<StatusError> for Response {
  fn from(e: StatusError) -> Self {
    match e {
      StatusError::Db(_) => {
        let error = JsonErrorPayload {
          error: JsonError {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: String::from("Internal server error (db)"),
          },
        };

        Response::from((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
      }
    }
  }
}

pub async fn status(_: Request, _: Next) -> Result<Response, StatusError> {
  db::models::db_writable_test::test().await?;
  let res = Response::from((StatusCode::OK, Json(Status { ok: true })));
  Ok(res)
}

use std::net::SocketAddr;

use hyper::{Server, StatusCode};
use log::*;
use logger::init;
use prex::{Next, Request, Response};

#[tokio::main]
async fn main() {
  init();

  let mut app = prex::prex();

  app.with(logger);
  app.with(ok);

  let app = app.build().expect("prex build");

  let addr = SocketAddr::from(([0, 0, 0, 0], 21000));

  let server = Server::bind(&addr);
  info!("server bound to {addr}");

  server.serve(app).await.expect("hyper serve");
}

async fn logger(req: Request, next: Next) -> Response {
  let method = req.method().to_string();
  let uri = req
    .uri()
    .path_and_query()
    .map(ToString::to_string)
    .unwrap_or_else(|| String::new());

  info!("{method} {uri}");

  next.run(req).await
}

async fn ok(_: Request, _: Next) -> Response {
  let res = Response::new(StatusCode::OK);
  res
}

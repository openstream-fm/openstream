use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use prex::{Next, Request, Response};
use std::future::Future;
use std::net::SocketAddr;

use tokio::sync::broadcast::error::RecvError;

pub fn start() -> impl Future<Output = ()> + Send + Sync + 'static {
  let mut app = prex::prex();

  app.get("/stream/:id", handle);

  let app = app.build().expect("prex app build stream");

  let addr = SocketAddr::from(([0, 0, 0, 0], 20300));
  info!("stream server bound to {addr}");

  async move {
    let server = Server::try_bind(&addr).expect("hyper bind stream");
    server.serve(app).await.expect("hyper serve stream");
  }
}

async fn handle(req: Request, _next: Next) -> Response {
  // unwrap: "id" is a required param in path definition
  let id = req.param("id").unwrap();

  let mut stream = match channels::subscribe(id) {
    Some(stream) => stream,
    None => {
      let mut res = Response::new(StatusCode::NOT_FOUND);
      *res.body_mut() = Body::from(format!(
        "Stream with id {id} is not actively streaming right now"
      ));
      return res;
    }
  };

  let (mut body_sender, response_body) = Body::channel();

  tokio::spawn(async move {
    loop {
      match stream.recv().await {
        // if lagged we ignore the error and continue with the oldest message buffered in the channel
        // TODO: maybe we should advance to the newest message with stream.resubscribe
        Err(RecvError::Lagged(_)) => continue,

        // Here the channel has been dropped
        Err(RecvError::Closed) => break,

        // Receive bytes and pass it to response body
        Ok(bytes) => {
          match body_sender.send_data(bytes).await {
            Err(_) => break,
            Ok(()) => continue,
          };
        }
      }
    }
  });

  let mut res = Response::new(StatusCode::OK);
  res
    .headers_mut()
    .insert(CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));
  *res.body_mut() = response_body;

  res
}

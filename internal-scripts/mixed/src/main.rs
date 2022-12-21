use std::net::SocketAddr;

use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use prex::*;

#[tokio::main]
async fn main() {
  let mut app = prex::prex();

  app.get("/mixed", mixed);

  let app = app.build().unwrap();

  let addr = SocketAddr::from(([0, 0, 0, 0], 20700));

  let server = Server::bind(&addr);

  println!("server listening at {addr}");

  server.serve(app).await.unwrap();
}

async fn mixed(_: Request, _: Next) -> Response {
  static MP3: &[u8] = include_bytes!("../samples/mp3.mp3");
  static AAC: &[u8] = include_bytes!("../samples/aac.aac");
  //static OGG: &[u8] = include_bytes!("../samples/ogg.ogg");
  //static WEBM: &[u8] = include_bytes!("../samples/webm.webm");

  let mut body = vec![];
  for _ in 0..1 {
    for slice in [AAC, MP3 /* OGG, WEBM*/].into_iter() {
      body.extend_from_slice(&slice[17853..]);
    }
  }

  let mut res = Response::new(StatusCode::OK);
  res
    .headers_mut()
    .append(CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));

  *res.body_mut() = Body::from(body);

  res
}

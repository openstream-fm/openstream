use std::str::FromStr;

use bytes::Bytes;
use hyper::{Body, Client, Method, Request};

static AUDIO: &'static [u8] = include_bytes!("../../../audio.aac");

#[tokio::main]
async fn main() {
  let client = Client::new();

  let (mut sender, body) = Body::channel();

  let request = Request::builder()
    .uri("http://localhost:20500/source/1")
    .method(Method::from_str("SOURCE").unwrap())
    //.header("expect", "100-continue")
    .body(body)
    .unwrap();

  let handle1 = tokio::spawn(async move {
    loop {
      if sender.send_data(Bytes::from_static(AUDIO)).await.is_err() {
        break;
      }
    }
  });

  let handle2 = tokio::spawn(async move {
    client.request(request).await.unwrap();
  });

  handle1.await.unwrap();
  handle2.await.unwrap();
}

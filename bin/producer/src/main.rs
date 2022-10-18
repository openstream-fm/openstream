use std::str::FromStr;

use bytes::Bytes;
use hyper::{Body, Client, Method, Request};

static AUDIO: Bytes = Bytes::from_static(include_bytes!("../../../audio.aac"));

#[tokio::main]
async fn main() {
  let client = Client::new();

  let (mut sender, body) = Body::channel();

  let request = Request::builder()
    .uri("http://localhost:20600/source/1")
    .method(Method::from_str("SOURCE").unwrap())
    //.header("expect", "100-continue")
    .body(body)
    .unwrap();

  let tx_handle = async move {
    loop {
      if sender.send_data(AUDIO.clone()).await.is_err() {
        break;
      }
    }
  };

  let response = async move {
    client.request(request).await.unwrap();
  };

  let (_response, _tx_handle) = tokio::join!(response, tx_handle);
}

use bytes::Bytes;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use hyper::body::HttpBody;
use hyper::{Body, Client, Method, Request, Uri};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

static CURRENT_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static HISTORIC_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static BYTES_READED: AtomicUsize = AtomicUsize::new(0);
static ERRORS: AtomicUsize = AtomicUsize::new(0);

static BODY: Bytes = Bytes::from_static(include_bytes!("../../../audio.mp3"));

const C: usize = 5_000;

#[tokio::main]
async fn main() {
  let _ = tokio::join!(clients(C), producer());
}

async fn producer() {
  let client = Client::new();
  let (mut tx, body) = Body::channel();

  let sender = async move {
    loop {
      tx.send_data(BODY.clone()).await.unwrap();
    }
  };

  let request = Request::builder()
    .uri("http://192.168.0.102:20600/1/source")
    .method("SOURCE")
    .body(body)
    .unwrap();

  let response = client.request(request);

  let _ = tokio::join!(response, sender);
}

async fn clients(n: usize) {
  let mut queue = FuturesUnordered::new();
  for i in 0..n {
    queue.push(client())
  }

  loop {
    match queue.next().await {
      Some(Err(e)) => {
        ERRORS.fetch_add(1, Ordering::Relaxed);
      }
      Some(Ok(_)) => {}
      None => {}
    };
    CURRENT_CLIENTS.fetch_sub(1, Ordering::Relaxed);
    queue.push(client());
  }
}

async fn client() -> Result<(), hyper::Error> {
  CURRENT_CLIENTS.fetch_add(1, Ordering::Relaxed);
  HISTORIC_CLIENTS.fetch_add(1, Ordering::Relaxed);

  let client = Client::new();
  let mut res = client.get("http://192.168.0.102:20300/stream/1").await?;
  while let Some(data) = res.data().await {
    let data = data?;
    BYTES_READED.fetch_add(data.len(), Ordering::Relaxed);
  }

  Ok(())
}

async fn print_stats() {
  let mut prev = 0;
  let mut interval = tokio::time::interval(Duration::from_secs(1));
  loop {
    interval.tick().await;
    let bytes_readed = BYTES_READED.load(Ordering::Relaxed);
    let speed = bytes_readed - prev;
    prev = bytes_readed;

    let historic_clients = HISTORIC_CLIENTS.load(Ordering::Relaxed);
    let current_clients = CURRENT_CLIENTS.load(Ordering::Relaxed);
    let errors = ERRORS.load(Ordering::Relaxed);

    println!("==========================================");
    println!("{current_clients} open connections");
    println!("{historic_clients} all time connections");
    println!("{errors} errors");
    println!("{} MB", bytes_readed as f64 / 1024 as f64 / 1024 as f64);
    println!("{} MB/sec", speed / 1024 * 1024);
  }
}

use bytes::Bytes;
use hyper::body::HttpBody;
use hyper::{Body, Client, Request, Uri};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::oneshot;

static CURRENT_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static HISTORIC_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static BYTES_READED: AtomicUsize = AtomicUsize::new(0);
static ERRORS: AtomicUsize = AtomicUsize::new(0);

static BODY: Bytes = Bytes::from_static(include_bytes!("../../../audio.mp3"));

const DEFAULT_C: usize = 1_000;

#[tokio::main]
async fn main() {
  let _ = dotenv::dotenv();

  let source_base = std::env::var("SOURCE_BASE_URL")
    .expect("SOURCE_BASE_URL env not set")
    .trim_end_matches('/')
    .to_string();
  let stream_base = std::env::var("STREAM_BASE_URL")
    .expect("STREAM_BASE_URL env not set")
    .trim_end_matches('/')
    .to_string();

  let _: Uri = source_base.parse().expect("SOURCE_BASE invalid URL");
  let _: Uri = stream_base.parse().expect("STREAM_BASE invalid URL");

  let c: usize = match std::env::var("C") {
    Ok(c) => c.parse().unwrap_or(DEFAULT_C),
    Err(_) => DEFAULT_C,
  };

  let (send, recv) = oneshot::channel::<()>();
  let _ = tokio::try_join!(
    tokio::spawn(producer(source_base, send)),
    tokio::spawn(clients(c, stream_base, recv)),
    tokio::spawn(print_stats())
  )
  .unwrap();
}

async fn producer(base: String, ready: oneshot::Sender<()>) {
  let client = Client::new();
  let (mut tx, body) = Body::channel();

  let sender = async move {
    tx.send_data(BODY.clone()).await.unwrap();
    ready.send(()).unwrap();

    loop {
      tx.send_data(BODY.clone()).await.unwrap();
    }
  };

  let request = Request::builder()
    .uri(format!("{base}/1/source"))
    .method("SOURCE")
    .body(body)
    .unwrap();

  let response = client.request(request);

  let _ = tokio::join!(response, sender);
}

async fn clients(n: usize, base: String, ready: oneshot::Receiver<()>) {
  ready.await.unwrap();

  tokio::spawn(async move {
    for _i in 0..n {
      let base = base.clone();
      tokio::spawn(async move {
        loop {
          match client(base.as_str()).await {
            Err(_) => {
              ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
          };
          CURRENT_CLIENTS.fetch_sub(1, Ordering::Relaxed);
        }
      });
    }
  })
  .await
  .unwrap();
}

async fn client(base: &str) -> Result<(), hyper::Error> {
  CURRENT_CLIENTS.fetch_add(1, Ordering::Relaxed);
  HISTORIC_CLIENTS.fetch_add(1, Ordering::Relaxed);

  let client = Client::new();
  let mut res = client
    .get(format!("{base}/stream/1").parse().expect("client uri"))
    .await?;
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
    println!("{} MB/sec", speed / 1024 / 1024);
  }
}

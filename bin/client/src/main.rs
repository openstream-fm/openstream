use bytes::Bytes;
use hyper::{Body, Uri};
use reqwest::Client;
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

  let delay: u64 = match std::env::var("D") {
    Ok(s) => s.parse().unwrap_or(100),
    Err(_) => 100,
  };

  let _: Uri = source_base.parse().expect("SOURCE_BASE_URL invalid URL");
  let _: Uri = stream_base.parse().expect("STREAM_BASE_URL invalid URL");

  let c: usize = match std::env::var("C") {
    Ok(s) => s.parse().unwrap_or(DEFAULT_C),
    Err(_) => DEFAULT_C,
  };

  println!("source base: {source_base}");
  println!("stream base: {stream_base}");
  println!("concurrency: {c}");
  println!("delay: {delay}");

  let _ = tokio::try_join!(
    tokio::spawn(producer(source_base)),
    tokio::spawn(clients(c, stream_base, delay)),
    tokio::spawn(print_stats())
  )
  .unwrap();
}

async fn producer(base: String) {
  let client = Client::new();
  let (mut tx, body) = Body::channel();

  let _sender = tokio::spawn(async move {
    loop {
      match tx.send_data(BODY.clone()).await {
        Ok(_) => continue,
        Err(_e) => break,
      }
    }
  });

  let response = client
    .put(format!("{base}/1/source"))
    .body(body)
    .send()
    .await
    .expect("producer send().await");

  println!("producer status: {:?}", response.status());

  let body = response.text().await.expect("producer text().await");

  println!("producer body: {body}");
  panic!("producer terminated");
}

async fn clients(n: usize, base: String, delay: u64) {
  tokio::time::sleep(Duration::from_millis(1_000)).await;

  tokio::spawn(async move {
    for _i in 0..n {
      tokio::time::sleep(Duration::from_millis(delay)).await;
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

async fn client(base: &str) -> Result<(), reqwest::Error> {
  CURRENT_CLIENTS.fetch_add(1, Ordering::Relaxed);
  HISTORIC_CLIENTS.fetch_add(1, Ordering::Relaxed);

  let client = Client::new();
  let mut res = client.get(format!("{base}/stream/1")).send().await?;

  while let Some(data) = res.chunk().await? {
    BYTES_READED.fetch_add(data.len(), Ordering::Relaxed);
  }

  Ok(())
}

async fn print_stats() {
  let mut prev = 0;
  let mut interval = tokio::time::interval(Duration::from_secs(1));
  interval.tick().await;
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

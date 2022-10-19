use bytes::Bytes;
use hyper::body::HttpBody;
use hyper::{Body, Client, Request, Uri};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
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
  let base: Arc<Uri> = Arc::new(
    std::env::var("TARGET")
      .expect("TARGET env not set")
      .parse()
      .expect("Invalid URL"),
  );

  let c: usize = match std::env::var("C") {
    Ok(c) => c.parse().unwrap_or(DEFAULT_C),
    Err(_) => DEFAULT_C,
  };

  let (send, recv) = oneshot::channel::<()>();
  let _ = tokio::try_join!(
    tokio::spawn(clients(c, base.clone(), recv)),
    tokio::spawn(producer(base, send)),
    tokio::spawn(print_stats())
  )
  .unwrap();
}

async fn producer(base: Arc<Uri>, ready: oneshot::Sender<()>) {
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

async fn clients(n: usize, base: Arc<Uri>, ready: oneshot::Receiver<()>) {
  ready.await.unwrap();

  tokio::spawn(async move {
    for _i in 0..n {
      let base = base.clone();
      tokio::spawn(async move {
        loop {
          let base = base.clone();
          match client(base.clone()).await {
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

async fn client(base: Arc<Uri>) -> Result<(), hyper::Error> {
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

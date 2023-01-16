use bytes::Bytes;
use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
use hyper::Body;
use reqwest::Client;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

static CONNECTING_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static CURRENT_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static HISTORIC_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static BYTES_READED: AtomicUsize = AtomicUsize::new(0);
static ERRORS: AtomicUsize = AtomicUsize::new(0);

static BODY: &[u8] = include_bytes!("../../../audio.mp3");

const DEFAULT_C: usize = 20_000;

const DEFAULT_S: u64 = 10_000;

lazy_static::lazy_static! {
  static ref S: u64 = {
    match std::env::var("S") {
      Err(_) => DEFAULT_S,
      Ok(s) => s.parse::<u64>().unwrap_or(DEFAULT_S),
    }
  };
}

fn rr_test_account_n() -> u64 {
  static RR_ACCOUNT_N: AtomicU64 = AtomicU64::new(0);
  let v = RR_ACCOUNT_N.fetch_add(1, Ordering::SeqCst);
  1 + (v % *S)
}

fn rr_test_account_id() -> String {
  format!("test{}", rr_test_account_n())
}

#[tokio::main]
async fn main() {
  let _ = dotenv::dotenv();

  // let source_base = std::env::var("SOURCE_BASE_URL")
  //   .expect("SOURCE_BASE_URL env not set")
  //   .trim_end_matches('/')
  //   .to_string();

  let stream_base_url = std::env::var("STREAM_BASE_URL")
    .expect("STREAM_BASE_URL env is not set")
    .trim_end_matches('/')
    .to_string();

  let ports: Vec<u16> = std::env::var("STREAM_PORTS")
    .expect("STREAM_PORTS env is not set")
    .split(',')
    .map(|s| s.trim().parse().expect("invalid STREAM_PORTS env"))
    .collect();

  let delay: u64 = match std::env::var("D") {
    Ok(s) => s.parse().unwrap_or(5),
    Err(_) => 5,
  };

  // let mountpoint_id = std::env::var("S").unwrap_or_else(|_| String::from("jr8n73bs"));
  // let _: Uri = source_base.parse().expect("SOURCE_BASE_URL invalid URL");

  let c: usize = match std::env::var("C") {
    Ok(s) => s.parse().unwrap_or(DEFAULT_C),
    Err(_) => DEFAULT_C,
  };

  // println!("mounpoint id: {mountpoint_id}");
  // println!("source base: {source_base}");
  println!("stream base url: {stream_base_url}");
  println!("stream ports: {ports:?}");
  println!("concurrency: {c}");
  println!("delay: {delay}");

  let _ = tokio::try_join!(
    // tokio::spawn(producer(source_base, mountpoint_id.clone())),
    tokio::spawn(clients(c, stream_base_url, ports, delay)),
    tokio::spawn(print_stats())
  )
  .unwrap();
}

#[allow(unused)]
async fn producer(base: String, id: String) {
  let client = Client::new();
  let (mut tx, body) = Body::channel();

  let config = FfmpegConfig {
    readrate: true,
    ..Default::default()
  };

  let FfmpegSpawn {
    child: _child,
    mut stdin,
    mut stdout,
    ..
  } = Ffmpeg::new(config).spawn().expect("ffmpeg spawn");

  tokio::spawn(async move {
    loop {
      stdin.write_all(BODY).await.expect("ffmpeg write");
    }
  });

  let _sender = tokio::spawn(async move {
    let mut buf = [0u8; 8000];
    loop {
      let n = stdout.read(&mut buf).await.expect("ffmpeg read");
      tx.send_data(Bytes::copy_from_slice(&buf[0..n]))
        .await
        .expect("producer send_data");
    }
  });

  let response = client
    .put(format!("{base}/{id}/source"))
    .body(body)
    .send()
    .await
    .expect("producer send request");

  println!("producer status: {:?}", response.status());

  let body = response.text().await.expect("producer text().await");

  println!("producer body: {body}");
  panic!("producer terminated");
}

async fn clients(n: usize, stream_base_url: String, ports: Vec<u16>, delay: u64) {
  tokio::time::sleep(Duration::from_millis(1_000)).await;

  let http_client = Client::builder()
    .http1_only()
    .build()
    .expect("build client");

  tokio::spawn(async move {
    for i in 0..n {
      tokio::time::sleep(Duration::from_millis(delay)).await;
      let port = ports[i % ports.len()];
      let base_url = stream_base_url.clone();
      let http_client = http_client.clone();
      tokio::spawn(async move {
        loop {
          let r = client(
            &http_client,
            base_url.as_str(),
            port,
            &&rr_test_account_id(),
          )
          .await;
          if let Err(e) = r {
            ERRORS.fetch_add(1, Ordering::Relaxed);
            println!("err: {}", e);
          }
          CURRENT_CLIENTS.fetch_sub(1, Ordering::Relaxed);
        }
      });
    }
  })
  .await
  .unwrap();
}

/*
async fn client(base: &str, id: &str) -> Result<(), std::io::Error> {
  let url: Uri = base.parse().unwrap();
  let addr = SocketAddr::from(([0, 0, 0, 0], url.port_u16().unwrap_or(80)));

  CONNECTING_CLIENTS.fetch_add(1, Ordering::Relaxed);

  let mut socket = TcpStream::connect(addr).await?;

  CONNECTING_CLIENTS.fetch_sub(1, Ordering::Relaxed);
  CURRENT_CLIENTS.fetch_add(1, Ordering::Relaxed);
  HISTORIC_CLIENTS.fetch_add(1, Ordering::Relaxed);

  socket
    .write_all(format!("GET /broadcast/{id} HTTP/1.0\r\n").as_bytes())
    .await?;
  socket
    .write_all(format!("host: localhost\r\n\r\n").as_bytes())
    .await?;

  let mut buf = [0; 8000];
  loop {
    let n = socket.read(&mut buf).await?;
    if n == 0 {
      break;
    }
    BYTES_READED.fetch_add(n, Ordering::Relaxed);
  }

  Ok(())
}
*/
async fn client(
  client: &Client,
  base_url: &str,
  port: u16,
  id: &str,
) -> Result<(), reqwest::Error> {
  CURRENT_CLIENTS.fetch_add(1, Ordering::Relaxed);
  HISTORIC_CLIENTS.fetch_add(1, Ordering::Relaxed);

  //let client = Client::new();
  let mut res = client
    .get(format!("{base_url}:{port}/stream/{id}"))
    .send()
    .await?;

  let start = Instant::now();

  while let Some(data) = res.chunk().await? {
    BYTES_READED.fetch_add(data.len(), Ordering::Relaxed);
    if start.elapsed() > Duration::from_secs(60_000) {
      break;
    }
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
    let connecting_clients = CONNECTING_CLIENTS.load(Ordering::Relaxed);
    let errors = ERRORS.load(Ordering::Relaxed);

    println!("==========================================");
    println!("{connecting_clients} connecting clients");
    println!("{current_clients} open connections");
    println!("{historic_clients} all time connections");
    println!("{errors} errors");
    println!("{} MB", bytes_readed as f64 / 1024_f64 / 1024_f64);
    println!("{} MB/sec", speed / 1024 / 1024);
  }
}

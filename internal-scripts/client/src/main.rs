use bytes::Bytes;
use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
use hyper::{Body, Uri};
use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

static CONNECTING_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static CURRENT_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static HISTORIC_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static BYTES_READED: AtomicUsize = AtomicUsize::new(0);
static ERRORS: AtomicUsize = AtomicUsize::new(0);

static BODY: &[u8] = include_bytes!("../../../audio.mp3");

const DEFAULT_C: usize = 1_000;

#[tokio::main]
async fn main() {
  let _ = dotenv::dotenv();

  let source_base = std::env::var("SOURCE_BASE_URL")
    .expect("SOURCE_BASE_URL env not set")
    .trim_end_matches('/')
    .to_string();

  let stream_host = std::env::var("STREAM_HOST")
    .expect("STREAM_BASE_URL env not set")
    .trim_end_matches('/')
    .to_string();

  let ports: Vec<u16> = std::env::var("STREAM_PORTS")
    .expect("no PORTS ENV")
    .split(',')
    .map(|s| s.trim().parse().expect("invalid STREAM_PORTS env"))
    .collect();

  let delay: u64 = match std::env::var("D") {
    Ok(s) => s.parse().unwrap_or(100),
    Err(_) => 100,
  };

  let mountpoint_id = std::env::var("S").unwrap_or_else(|_| String::from("1"));

  let _: Uri = source_base.parse().expect("SOURCE_BASE_URL invalid URL");

  let c: usize = match std::env::var("C") {
    Ok(s) => s.parse().unwrap_or(DEFAULT_C),
    Err(_) => DEFAULT_C,
  };

  println!("mounpoint id: {mountpoint_id}");
  println!("source base: {source_base}");
  println!("stream host: {stream_host}");
  println!("stream ports: {ports:?}");
  println!("concurrency: {c}");
  println!("delay: {delay}");

  let _ = tokio::try_join!(
    tokio::spawn(producer(source_base, mountpoint_id.clone())),
    tokio::spawn(clients(c, stream_host, ports, mountpoint_id, delay)),
    tokio::spawn(print_stats())
  )
  .unwrap();
}

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

async fn clients(n: usize, host: String, ports: Vec<u16>, id: String, delay: u64) {
  tokio::time::sleep(Duration::from_millis(1_000)).await;

  let http_client = Client::new();

  tokio::spawn(async move {
    for i in 0..n {
      tokio::time::sleep(Duration::from_millis(delay)).await;
      let port = ports[i % ports.len()];
      let host = host.clone();
      let id = id.clone();
      let http_client = http_client.clone();
      tokio::spawn(async move {
        loop {
          let r = client(&http_client, host.as_str(), port, id.as_str()).await;
          if r.is_err() {
            ERRORS.fetch_add(1, Ordering::Relaxed);
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
async fn client(client: &Client, host: &str, port: u16, id: &str) -> Result<(), reqwest::Error> {
  CURRENT_CLIENTS.fetch_add(1, Ordering::Relaxed);
  HISTORIC_CLIENTS.fetch_add(1, Ordering::Relaxed);

  //let client = Client::new();
  let mut res = client
    .get(format!("http://{host}:{port}/broadcast/{id}"))
    .send()
    .await?;

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

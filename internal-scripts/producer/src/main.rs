use std::{
  str::FromStr,
  sync::atomic::{AtomicUsize, Ordering},
};

use bytes::{Buf, Bytes};
use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
use hyper::{Method, StatusCode};
use reqwest::{Body, Client, Response, Url};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use multiqueue::{
  broadcast_fut_queue as channel,
  BroadcastFutReceiver as Receiver, /*BroadcastFutSender as Sender,*/
};

use futures::sink::Sink;
use futures::Future;

static ACTIVE_COUNT: AtomicUsize = AtomicUsize::new(0);
static TOTAL_COUNT: AtomicUsize = AtomicUsize::new(0);
static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static DATA_WRITTEN: AtomicUsize = AtomicUsize::new(0);

static AUDIO: Bytes = Bytes::from_static(include_bytes!("../../../audio.mp3"));

static TOKEN: &str = "sb8rqnt7nxkx8t8ca3uypg83k87qrw4258cs35s29ekqa4kv";

#[derive(Debug, thiserror::Error)]
enum Error {
  #[error("hyper: {0}")]
  Hyper(#[from] hyper::Error),
  #[error("reqwest: {0}")]
  Reqwest(#[from] reqwest::Error),
}

#[tokio::main]
async fn main() {
  dotenv::dotenv().expect("dotenv");

  let s = std::env::var("S")
    .expect("S env not present")
    .parse::<usize>()
    .expect("invalid S env");
  let d = std::env::var("D")
    .unwrap_or_else(|_| String::from("1000"))
    .parse::<u64>()
    .expect("invalid D env");

  if s == 0 {
    panic!("S env cant be 0");
  }

  // let (tx, rx) = channel::<Bytes>(1024);

  // let config = FfmpegConfig {
  //   copycodec: true,
  //   readrate: true,
  //   ..Default::default()
  // };

  // let spawn = Ffmpeg::new(config).spawn().expect("ffmpeg spawn");

  // let FfmpegSpawn {
  //   mut stdin,
  //   mut stdout,
  //   mut stderr,
  //   mut child,
  //   ..
  // } = spawn;

  // let future_stderr = async move {
  //   let mut buf = String::new();
  //   stderr
  //     .read_to_string(&mut buf)
  //     .await
  //     .expect("ffmpeg stderr read to string");
  //   buf
  // };

  // let future_stdin = async move {
  //   loop {
  //     stdin.write_all(AUDIO.as_ref()).await.expect("ffmpeg write");
  //   }
  // };

  // let future_stdout = {
  //   let tx = tx.clone();
  //   async move {
  //     let mut buf = [0; 256];
  //     loop {
  //       stdout.read_exact(&mut buf).await.expect("ffmpeg read");
  //       let bytes = Bytes::copy_from_slice(&buf);
  //       tx.send(bytes).await.expect("tx send");
  //     }
  //   }
  // };

  // let future_exit = async move { child.wait().await.expect("ffmpeg wait") };

  // tokio::spawn(future_stdin);
  // tokio::spawn(future_stdout);
  // tokio::spawn(async move {
  //   let (exit, stderr) = tokio::join!(future_exit, future_stderr);
  //   eprintln!("ffmpeg exit: {exit:?}, stderr: {stderr}");
  //   panic!("ffmpeg exit");
  // });

  println!("logger");
  tokio::spawn(logger());

  let client = reqwest::Client::new();

  let delay = tokio::time::Duration::from_millis(d);

  for i in 0..s {
    tokio::time::sleep(delay).await;
    producer(client.clone(), format!("test{}", i + 1));
  }

  // drop(rx);

  tokio::time::sleep(std::time::Duration::from_secs(u64::MAX)).await;
}

fn producer(client: reqwest::Client, id: String) -> tokio::task::JoinHandle<()> {
  tokio::spawn(async move {
    let active = ACTIVE_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
    let total = TOTAL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
    let errors = ERROR_COUNT.load(Ordering::SeqCst);

    eprintln!("start | active: {active}, total: {total}, errors: {errors}");

    #[allow(clippy::expect_fun_call)]
    let password = get_source_password(&client, &id)
      .await
      .expect(&format!("get password for {id}"));

    // let stream = async_stream::stream! {
    //   loop {
    //     let data = rx.recv().await.expect("rx recv");
    //     let len = data.len();
    //     yield Ok::<Bytes, std::convert::Infallible>(data);
    //     DATA_WRITTEN.fetch_add(len, Ordering::SeqCst);
    //   };
    // };
    // let body = Body::wrap_stream(stream);

    let (mut body_sender, body) = hyper::Body::channel();

    tokio::spawn(async move {
      loop {
        for data in AUDIO.chunks(256) {
          let len = data.len();
          body_sender
            .send_data(Bytes::copy_from_slice(data))
            .await
            .expect("body send");
          DATA_WRITTEN.fetch_add(len, Ordering::SeqCst);
        }
      }
    });

    let body = Body::from(body);

    let method = Method::from_str("SOURCE").unwrap();

    let url = Url::parse(&format!(
      "http://source.local.openstream.fm:20600/{id}/source"
    ))
    .unwrap();

    let response = client
      .request(method, url)
      .basic_auth("source", Some(password))
      .body(body)
      .send()
      .await;

    let active = ACTIVE_COUNT.fetch_sub(1, Ordering::SeqCst) - 1;
    let total = TOTAL_COUNT.load(Ordering::SeqCst);

    match response {
      Ok(_) => {
        eprintln!("end | active: {active}, total: {total}, errors: {errors}");
      }
      Err(e) => {
        let errors = ERROR_COUNT.fetch_add(1, Ordering::SeqCst) - 1;
        eprintln!("error: {e} => {e:?}");
        eprintln!("error | active: {active}, total: {total}, errors: {errors}");
      }
    };

    producer(client, id);
  })
}

#[derive(Debug, thiserror::Error)]
enum GetPasswordError {
  #[error("reqwest fetch: {0}")]
  Fetch(#[source] reqwest::Error),
  #[error("reqwest body: {0}")]
  Body(#[source] reqwest::Error),
  #[error("status not ok: {:?}", response.status())]
  Status { response: Response },
  #[error("response json: {error}, status: {status:?}")]
  Json {
    status: StatusCode,
    body: Bytes,
    #[source]
    error: serde_json::Error,
  },
}

async fn get_source_password(
  client: &Client,
  station_id: &str,
) -> Result<String, GetPasswordError> {
  let url = Url::parse(&format!(
    "https://api.local.openstream.fm/stations/{station_id}"
  ))
  .unwrap();
  let res = client
    .get(url)
    .header("x-access-token", TOKEN)
    .send()
    .await
    .map_err(GetPasswordError::Fetch)?;

  if !res.status().is_success() {
    return Err(GetPasswordError::Status { response: res });
  };

  let status = res.status();
  let body = res.bytes().await.map_err(GetPasswordError::Body)?;

  let output: api::routes::stations::id::get::Output = match serde_json::from_slice(&body) {
    Ok(out) => out,
    Err(error) => {
      return Err(GetPasswordError::Json {
        status,
        body,
        error,
      })
    }
  };

  match output.station {
    db::station::PublicStation::Admin(station) => Ok(station.0.source_password),
    db::station::PublicStation::User(station) => Ok(station.source_password),
  }
}

async fn logger() {
  let mut prev_written = DATA_WRITTEN.load(Ordering::SeqCst);
  let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
  loop {
    interval.tick().await;
    let written = DATA_WRITTEN.load(Ordering::SeqCst);
    let tick_written = written - prev_written;
    prev_written = written;
    let stations = ACTIVE_COUNT.load(Ordering::SeqCst);
    let per_station = tick_written / if stations == 0 { 1 } else { stations };
    println!(
      "data: {} / s - {} per station ({})",
      B(tick_written),
      B(per_station),
      stations
    )
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct B(pub usize);

impl std::fmt::Display for B {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const K: usize = 1_000;
    const M: usize = 1_000_000;
    const G: usize = 1_000_000_000;

    let b = self.0;

    if b < K {
      write!(f, "{b} B")
    } else if b < M {
      let k = b as f64 / K as f64;
      write!(f, "{:.2} KB", k)
    } else if b < G {
      let m = b as f64 / M as f64;
      write!(f, "{:.2} MB", m)
    } else {
      let g = b as f64 / G as f64;
      write!(f, "{:.2} GB", g)
    }
  }
}

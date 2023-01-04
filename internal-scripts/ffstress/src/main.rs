use std::{
  sync::atomic::{AtomicUsize, Ordering},
  time::Duration,
};

use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

static AUDIO: &[u8] = include_bytes!("../../../audio.mp3");

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static IN: AtomicUsize = AtomicUsize::new(0);
static OUT: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
  let c: usize = std::env::var("C")
    .expect("C env missing")
    .parse()
    .expect("C env invalid");

  let mut handles = vec![];

  for i in 0..c {
    handles.push(tokio::spawn(spawn(i)));
  }

  let cmds = async move {
    for h in handles {
      h.await.expect("tokio::spawn");
    }
  };

  tokio::join!(logger(), cmds);
}

async fn spawn(_i: usize) {
  tokio::spawn(async move {
    CURRENT.fetch_add(1, Ordering::SeqCst);

    let config = FfmpegConfig {
      readrate: true,
      kbitrate: 320_000,
      ..Default::default()
    };

    let FfmpegSpawn {
      mut stdin,
      mut stdout,
      child: _child,
      ..
    } = Ffmpeg::new(config).spawn().expect("ffmpeg spawn");

    let write = async move {
      loop {
        for chunk in AUDIO.chunks(256) {
          stdin.write_all(chunk).await.expect("ffmpeg write");
          IN.fetch_add(chunk.len(), Ordering::SeqCst);
        }
      }
    };

    let read = async move {
      let mut buf = [0; 1000];

      loop {
        let n = stdout.read(&mut buf).await.expect("ffmpeg read");

        if n == 0 {
          panic!("ffmpeg end");
        }

        OUT.fetch_add(n, Ordering::SeqCst);
      }
    };

    tokio::join!(write, read);
  })
  .await
  .expect("ffmpeg tokio::spawn panic");

  CURRENT.fetch_sub(1, Ordering::SeqCst);
}

async fn logger() {
  let mut interval = tokio::time::interval(Duration::from_secs(1));
  interval.tick().await;
  let mut curr_in = IN.load(Ordering::SeqCst);
  let mut curr_out = OUT.load(Ordering::SeqCst);
  loop {
    interval.tick().await;
    let prev_in = curr_in;
    let prev_out = curr_out;
    curr_in = IN.load(Ordering::SeqCst);
    curr_out = OUT.load(Ordering::SeqCst);

    let cmds = CURRENT.load(Ordering::SeqCst);

    let kbps_in = (curr_in - prev_in) * 8 / 1000 / cmds;
    let kbps_out = (curr_out - prev_out) * 8 / 1000 / cmds;

    println!("==============================");
    println!("{cmds} stations");
    println!(" IN: {kbps_in:5} kbps/station");
    println!("OUT: {kbps_out:5} kbps/station");
  }
}

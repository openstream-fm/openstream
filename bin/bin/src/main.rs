use log::*;

fn main() {
  proctitle::set_title("openstream-rs");

  logger::init();

  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();

  rt.block_on(tokio_main())
}

async fn tokio_main() {
  info!("openstream process started");

  let handle1 = tokio::spawn(source::start(([0, 0, 0, 0], 20500)));
  let handle2 = tokio::spawn(source_hyper::start());
  let handle3 = tokio::spawn(stream::start());

  tokio::select! {
      r = handle1 => r.expect("source panicked").expect("source errored"),
      r = handle2 => r.expect("hyper source panicked"),
      r = handle3 => r.expect("stream panicked"),
  };
}

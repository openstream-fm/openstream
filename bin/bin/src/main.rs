use log::*;

fn main() {
  logger::init();

  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .unhandled_panic(tokio::runtime::UnhandledPanic::ShutdownRuntime)
    .build()
    .unwrap();

  rt.block_on(tokio_main())
}

async fn tokio_main() {
  info!("openstream process started");

  //let handle1 = tokio::spawn(source::start(([0, 0, 0, 0], 20500)));
  let handle2 = tokio::spawn(source::start());
  let handle3 = tokio::spawn(stream::start());

  tokio::select! {
      //r = handle1 => r.expect("source panicked").expect("source errored"),
      r = handle2 => r.expect("source terminated"),
      _ = handle3 => panic!("stream terminated"),
  };
}

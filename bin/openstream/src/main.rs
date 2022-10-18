use log::*;

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  logger::init();
  let _ = dotenv::dotenv();

  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .unhandled_panic(tokio::runtime::UnhandledPanic::ShutdownRuntime)
    .build()
    .unwrap();

  rt.block_on(tokio_main())
}

async fn tokio_main() {
  info!("openstream v{VERSION} process started");

  //let handle1 = tokio::spawn(source_alt::start(([0, 0, 0, 0], 20500)));
  let handle2 = tokio::spawn(source::start());
  let handle3 = tokio::spawn(stream::start());

  tokio::select! {
      //r = handle1 => r.expect("source panicked").expect("source errored"),
      r = handle2 => {
        r.expect("source panicked");
        info!("source terminated");
      },

      r = handle3 => {
        r.expect("stream panicked");
        info!("stream terminated");
      }
  };
}

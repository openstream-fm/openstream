use log::*;

use channels::ChannelMap;
use owo::*;
use source::SourceServer;
use std::sync::Arc;
use stream::StreamServer;
use tokio::try_join;

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn std::error::Error>> {
  logger::init();
  let _ = dotenv::dotenv();

  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .unhandled_panic(tokio::runtime::UnhandledPanic::ShutdownRuntime)
    .build()
    .unwrap();

  rt.block_on(tokio_main())
}

async fn tokio_main() -> Result<(), Box<dyn std::error::Error>> {
  info!(
    "openstream {}{} process started",
    "v".yellow(),
    VERSION.yellow()
  );

  let channels = Arc::new(ChannelMap::new());

  let source = SourceServer::new(([0, 0, 0, 0], 20600), channels.clone());
  let stream = StreamServer::new(([0, 0, 0, 0], 20300), channels.clone());

  let source_fut = source.start()?;
  let stream_fut = stream.start()?;

  let ((), ()) = try_join!(source_fut, stream_fut)?;

  Ok(())
}

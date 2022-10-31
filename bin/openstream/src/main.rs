use config::Both;
use log::*;

use channels::ChannelMap;
use cond_count::CondCount;
use owo::*;
use rust_ipify::ipify;
use shutdown::Shutdown;
use source::SourceServer;
use std::{net::Ipv4Addr, sync::Arc};
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

  let config_path = "./config.toml";

  let canonical_config_path = match std::fs::canonicalize(config_path) {
    Err(_) => String::from(config_path),
    Ok(path) => path.to_string_lossy().to_string(),
  };

  info!(
    "loading config file from {}",
    canonical_config_path.yellow()
  );

  let config = config::load(config_path)?;

  debug!("resolved config: {:#?}", config);

  let client = mongodb::Client::with_uri_str(config.mongodb.url.as_str())
    .await
    .expect("failed to create mongodb client");

  if client.default_database().is_none() {
    panic!("no database specified in config, under [mongodb] url");
  }

  info!("connecting to mongodb...");
  client
    .default_database()
    .unwrap()
    .run_command(mongodb::bson::doc! { "ping": 1 }, None)
    .await?;

  info!("mongodb client connected");

  db::init(client);

  info!("retrieving public ip...");
  let ip: Ipv4Addr = ipify::get_ip4_string()?.parse()?;
  info!("public ip obtained: {}", ip.yellow());

  let config::Config {
    mongodb: _,
    interfaces,
  } = config;

  let shutdown = Shutdown::new();

  match interfaces {
    config::Interfaces::Both(Both {
      source: source_config,
      stream: stream_config,
    }) => {
      let source_condcount = CondCount::new();
      let source_channels = Arc::new(ChannelMap::new(source_condcount.clone()));
      let source = SourceServer::new(
        ([0, 0, 0, 0], source_config.receiver.port),
        ([0, 0, 0, 0], source_config.broadcaster.port),
        source_channels,
        shutdown.clone(),
        source_condcount,
      );

      let stream_condcount = CondCount::new();
      let stream_channels = Arc::new(ChannelMap::new(stream_condcount.clone()));
      let stream = StreamServer::new(
        ([0, 0, 0, 0], stream_config.port),
        stream_channels,
        shutdown.clone(),
        stream_condcount,
      );

      let source_fut = source.start()?;
      let stream_fut = stream.start()?;

      tokio::spawn(async move {
        tokio::signal::ctrl_c()
          .await
          .expect("failed to listen for SIGINT signal");
        info!("{} received, starting graceful shutdown", "SIGINT".yellow());
        shutdown.shutdown();
      });

      let ((), ()) = try_join!(source_fut, stream_fut)?;
    }

    config::Interfaces::Source(config) => {
      let condcount = CondCount::new();
      let channels = Arc::new(ChannelMap::new(condcount.clone()));
      let source = SourceServer::new(
        ([0, 0, 0, 0], config.receiver.port),
        ([0, 0, 0, 0], config.broadcaster.port),
        channels,
        shutdown.clone(),
        condcount,
      );

      let source_fut = source.start()?;

      tokio::spawn(async move {
        tokio::signal::ctrl_c()
          .await
          .expect("failed to listen for SIGINT signal");

        info!("{} received, starting graceful shutdown", "SIGINT".yellow());

        shutdown.shutdown();
      });

      source_fut.await?;
    }

    config::Interfaces::Stream(config) => {
      let cond_count = CondCount::new();
      let channels = Arc::new(ChannelMap::new(cond_count.clone()));
      let stream = StreamServer::new(
        ([0, 0, 0, 0], config.port),
        channels,
        shutdown.clone(),
        cond_count,
      );

      let stream_fut = stream.start()?;

      tokio::spawn(async move {
        tokio::signal::ctrl_c()
          .await
          .expect("failed to listen for SIGINT signal");

        info!("{} received, starting graceful shutdown", "SIGINT".yellow());

        shutdown.shutdown();
      });

      stream_fut.await?;
    }
  };

  Ok(())
}

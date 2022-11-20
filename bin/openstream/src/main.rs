use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::Both;
use log::*;

use owo_colors::*;
use shutdown::Shutdown;
use source::SourceServer;
use stream::StreamServer;
use tokio::{runtime::Runtime, try_join};

static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[command(author, version, about = "openstream radio streaming server")]
struct Cli {
  #[structopt(subcommand)]
  command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
  Start(Start),
  CreateConfig(CreateConfig),
}

#[derive(Debug, Parser)]
#[command(about = "Start openstream server(s) from a config file")]
struct Start {
  #[clap(short, long, default_value_t = String::from("./config.toml"))]
  config: String,
}

#[derive(Debug, Parser)]
#[command(about = "Create a default config file")]
struct CreateConfig {
  #[clap(short, long, default_value_t = String::from("./config.toml"))]
  output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  cmd()
}

fn cmd() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();
  match cli.command {
    Command::Start(opts) => start(opts),
    Command::CreateConfig(opts) => create_config(opts),
  }
}

fn runtime() -> Runtime {
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .unhandled_panic(tokio::runtime::UnhandledPanic::ShutdownRuntime)
    .build()
    .unwrap()
}

fn start(opts: Start) -> Result<(), Box<dyn std::error::Error>> {
  logger::init();
  let _ = dotenv::dotenv();
  runtime().block_on(start_async(opts))
}

async fn start_async(Start { config }: Start) -> Result<(), Box<dyn std::error::Error>> {
  info!(
    "openstream {}{} process started",
    "v".yellow(),
    VERSION.yellow()
  );

  let canonical_config_path = match std::fs::canonicalize(config.as_str()) {
    Err(_) => config.clone(),
    Ok(path) => path.to_string_lossy().to_string(),
  };

  info!(
    "loading config file from {}",
    canonical_config_path.yellow()
  );

  let config = config::load(config)?;

  debug!("resolved config: {:#?}", config);

  let client = mongodb::Client::with_uri_str(config.mongodb.url.as_str())
    .await
    .expect("failed to create mongodb client");

  if client.default_database().is_none() {
    panic!("no database specified in config, under [mongodb] url");
  }

  let ffmpeg_path = which::which("ffmpeg")?;

  info!(
    "using system ffmpeg from {}",
    ffmpeg_path.to_string_lossy().yellow()
  );

  info!("connecting to mongodb...");
  client
    .default_database()
    .unwrap()
    .run_command(mongodb::bson::doc! { "ping": 1 }, None)
    .await?;

  info!("mongodb client connected");

  db::init(client);

  info!("ensuring mongodb indexes...");
  db::ensure_indexes().await?;

  info!("retrieving public ip...");
  let ip = ip::get_ip_v4().await?;
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
      let source = SourceServer::new(
        source_config.receiver.addrs,
        source_config.broadcaster.addrs,
        shutdown.clone(),
      );

      let stream = StreamServer::new(stream_config.addrs, shutdown.clone());

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
      let source = SourceServer::new(
        config.receiver.addrs,
        config.broadcaster.addrs,
        shutdown.clone(),
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
      let stream = StreamServer::new(config.addrs, shutdown.clone());

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

fn create_config(CreateConfig { output }: CreateConfig) -> Result<(), Box<dyn std::error::Error>> {
  let canonical_config_path = match std::fs::canonicalize(output.as_str()) {
    Err(_) => output.clone(),
    Ok(path) => path.to_string_lossy().to_string(),
  };

  eprintln!(
    "creating default config file into {}",
    canonical_config_path.yellow()
  );

  let file = PathBuf::from(output);

  let exists = file.metadata().is_ok();

  if exists {
    eprintln!("file already exists, operation aborted");
    std::process::exit(1);
  }

  std::fs::write(file, include_bytes!("../../../config.example.toml"))?;

  eprintln!("config file created in {}", canonical_config_path.yellow());

  Ok(())
}

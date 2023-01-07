// use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use config::Config;
// use db::access_token::{AccessToken, GeneratedBy};
// use db::Model;
use futures::{FutureExt, TryStreamExt};
use log::*;

use anyhow::{bail, Context};
// use api::ApiServer;
use defer_lite::defer;
use media_sessions::MediaSessionMap;
use mongodb::bson::doc;
use mongodb::bson::Document;
use owo_colors::*;
// use router::RouterServer;
// use serde_util::DateTime;
use shutdown::Shutdown;
//use source::SourceServer;
use stream::StreamServer;
use tokio::runtime::Runtime;

static VERSION: &str = env!("CARGO_PKG_VERSION");

// #[global_allocator]
// static ALLOCATOR: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Debug, Parser)]
#[command(author, version, about = "openstream radio streaming server")]
struct Cli {
  #[structopt(subcommand)]
  command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
  Start(Start),
}

#[derive(Debug, Parser)]
#[command(about = "Start openstream stream server instance from a config file")]
struct Start {
  /// Path to the configuration file (relative to cwd)
  #[clap(short, long, default_value_t = String::from("./config.toml"))]
  config: String,
}

fn main() -> Result<(), anyhow::Error> {
  cmd()
}

fn cmd() -> Result<(), anyhow::Error> {
  let cli = Cli::parse();
  match cli.command {
    Command::Start(opts) => start(opts),
  }
}

fn runtime() -> Runtime {
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .unhandled_panic(tokio::runtime::UnhandledPanic::ShutdownRuntime)
    .build()
    .unwrap()
}

async fn shared_init(config: String) -> Result<Config, anyhow::Error> {
  logger::init();
  let _ = dotenv::dotenv();

  info!(
    "openstream-stream {}{} process started",
    "v".yellow(),
    VERSION.yellow()
  );

  let canonical_config_path = std::fs::canonicalize(config.as_str())
    .with_context(|| format!("error loading config file from {}", config.yellow()))?;

  info!(
    "loading config file from {}",
    canonical_config_path.to_string_lossy().yellow()
  );

  let config = config::load(config).with_context(|| {
    format!(
      "error loading config file from {}",
      canonical_config_path.to_string_lossy().yellow(),
    )
  })?;

  debug!("config loaded: resolved config: {:#?}", config);

  let client = mongodb::Client::with_uri_str(config.mongodb.url.as_str())
    .await
    .context("failed to create mongodb client")?;

  if client.default_database().is_none() {
    bail!("no database specified in config, under [mongodb] url");
  }

  info!("connecting to mongodb and testing transactions support...");

  {
    let test_cl_name = "__transactions_test";
    let db = client.default_database().unwrap();
    let cl = db.collection::<Document>(test_cl_name);

    {
      let mut session = client
        .start_session(None)
        .await
        .context("mongodb error when creating new client session")?;

      session
        .start_transaction(None)
        .await
        .context("mongodb error when starting new transaction")?;

      let cl2 = cl.clone();
      defer! {
        tokio::task::block_in_place(|| {
          tokio::runtime::Handle::current().block_on(async move {
            let _ = cl2.drop(None).await;
          })
        })
      }

      cl.insert_one_with_session(doc! {}, None, &mut session)
        .await
        .context("mongodb error when creating test document into test collection")?;

      cl.delete_many_with_session(doc! {}, None, &mut session)
        .await
        .context("mongodb error when deleting test document from test collection")?;

      session
        .commit_transaction()
        .await
        .context("mongodb error when commiting test transaction")?;
    }
  }

  info!("mongodb client connected with transactions support");

  db::init(client);

  info!("ensuring mongodb collections...");
  db::ensure_collections()
    .await
    .context("error ensuring mongodb collections and indexes")?;

  Ok(config)
}

fn start(opts: Start) -> Result<(), anyhow::Error> {
  runtime().block_on(start_async(opts))
}

async fn start_async(Start { config }: Start) -> Result<(), anyhow::Error> {
  let config = Arc::new(shared_init(config).await?);

  // let ffmpeg_path = which::which("ffmpeg")
  //   .context("error getting ffmpeg path (is ffmpeg installed and available in executable path?)")?;

  // info!(
  //   "using system ffmpeg from {}",
  //   ffmpeg_path.to_string_lossy().yellow()
  // );

  info!("retrieving public ip...");
  let ip = ip::get_ip_v4().await.context("error obtaining public ip")?;
  info!("public ip obtained: {}", ip.yellow());

  let config::Config { ref stream, .. } = config.as_ref();

  let shutdown = Shutdown::new();

  tokio::spawn({
    let shutdown = shutdown.clone();
    async move {
      tokio::signal::ctrl_c()
        .await
        .expect("failed to listen to SIGINT signal");
      info!("{} received, starting graceful shutdown", "SIGINT".yellow());
      shutdown.shutdown();
    }
  });

  let futs = futures::stream::FuturesUnordered::new();

  // if let Some(source_config) = source {
  //   let source = SourceServer::new(
  //     source_config.receiver.addrs.clone(),
  //     source_config.broadcaster.addrs.clone(),
  //     shutdown.clone(),
  //   );

  //   let fut = source.start()?;

  //   futs.push(fut.boxed());
  // }

  if let Some(stream_config) = stream {
    let media_sessions = MediaSessionMap::new();
    let stream = StreamServer::new(
      stream_config.addrs.clone(),
      shutdown.clone(),
      media_sessions,
    );

    let fut = stream.start()?;

    futs.push(fut.boxed());
  }

  // if let Some(api_config) = api {
  //   let api = ApiServer::new(api_config.addrs.clone(), shutdown.clone());
  //   let fut = api.start()?;
  //   futs.push(fut.boxed());
  // }

  // if let Some(router_config) = router {
  //   let router = RouterServer::new(router_config.addrs.clone(), shutdown.clone());
  //   let fut = router.start()?;
  //   futs.push(fut.boxed());
  // }

  futs.try_collect().await?;

  Ok(())
}

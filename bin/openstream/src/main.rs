use std::path::PathBuf;
use std::process::ExitStatus;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use config::Config;
use db::access_token::{AccessToken, GeneratedBy};
use db::Model;
use futures::stream::FuturesUnordered;
use futures::{FutureExt, TryStreamExt};
use log::*;

use anyhow::{bail, Context};
use api::ApiServer;
use defer_lite::defer;
use media_sessions::MediaSessionMap;
use mongodb::bson::doc;
use mongodb::bson::Document;
use owo_colors::*;
use router::RouterServer;
use serde_util::DateTime;
use shutdown::Shutdown;
use source::SourceServer;
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
  Cluster(Cluster),
  CreateConfig(CreateConfig),
  CreateToken(CreateToken),
}

#[derive(Debug, Parser)]
#[command(about = "Start openstream server(s) from a config file")]
struct Start {
  /// Path to the configuration file (relative to cwd)
  #[clap(short, long, default_value_t = String::from("./config.toml"))]
  config: String,
}

#[derive(Debug, Parser)]
#[command(about = "Create a cluster of n --instances of `openstream start` processes")]
struct Cluster {
  /// Number of instances to spawn
  #[clap(short, long)]
  instances: u16,

  /// Path to the configuration file (relative to cwd)
  #[clap(short, long, default_value_t = String::from("./config.toml"))]
  config: String,
}

#[derive(Debug, Parser)]
#[command(
  about = "Create a new global access token to use in all openstream instances that share the same mongodb deployment"
)]
struct CreateToken {
  /// Path to the configuration file (relative to cwd)
  #[clap(short, long, default_value_t = String::from("./config.toml"))]
  config: String,

  /// Do not ask for confirmation
  #[clap(short = 'y', long, default_value_t = false)]
  assume_yes: bool,

  /// Use this value for the access token title instead of asking
  #[clap(long)]
  title: Option<String>,
}

#[derive(Debug, Parser)]
#[command(about = "Create a default config file for later editing")]
struct CreateConfig {
  /// Path to the output file (relative to cwd) to write the default config to
  #[clap(short, long, default_value_t = String::from("./config.toml"))]
  output: String,
}

fn main() -> Result<(), anyhow::Error> {
  cmd()
}

fn cmd() -> Result<(), anyhow::Error> {
  let cli = Cli::parse();
  match cli.command {
    Command::Cluster(opts) => cluster(opts),
    Command::Start(opts) => start(opts),
    Command::CreateConfig(opts) => create_config(opts),
    Command::CreateToken(opts) => token(opts),
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
    "openstream {}{} process started",
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
    let test_cl_name = format!("__transactions_test_{}", uid::uid(5));
    let db = client.default_database().unwrap();
    let cl = db.collection::<Document>(&test_cl_name);

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

  db::init(client, config.mongodb.storage_db_name.clone());

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

  let ffmpeg_path = which::which("ffmpeg")
    .context("error getting ffmpeg path (is ffmpeg installed and available in executable path?)")?;

  info!(
    "using system ffmpeg from {}",
    ffmpeg_path.to_string_lossy().yellow()
  );

  let local_ip = local_ip_address::local_ip().context("error obtaining local ip")?;
  info!("local ip address: {}", local_ip.yellow());


  info!("retrieving public ip...");
  let ip = ip::get_ip_v4().await.context("error obtaining public ip")?;
  info!("public ip address: {}", ip.yellow());

  
  let config::Config {
    mongodb: _,
    ref stream,
    ref source,
    ref api,
    ref router,
  } = config.as_ref();

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

  if let Some(source_config) = source {
    let source = SourceServer::new(
      source_config.receiver.addrs.clone(),
      source_config.broadcaster.addrs.clone(),
      shutdown.clone(),
    );

    let fut = source.start()?;

    futs.push(fut.boxed());
  }

  if let Some(stream_config) = stream {
    let media_sessions = MediaSessionMap::new();
    let stream = StreamServer::new(stream_config.addrs.clone(), shutdown.clone(), media_sessions);

    let fut = stream.start()?;

    futs.push(fut.boxed());
  }

  if let Some(api_config) = api {
    let api = ApiServer::new(api_config.addrs.clone(), shutdown.clone());
    let fut = api.start()?;
    futs.push(fut.boxed());
  }

  if let Some(router_config) = router {
    let router = RouterServer::new(router_config.addrs.clone(), shutdown.clone());
    let fut = router.start()?;
    futs.push(fut.boxed());
  }

  db::models::transfer_checkpoint::start_background_task();

  futs.try_collect().await?;

  Ok(())
}

fn cluster(opts: Cluster) -> Result<(), anyhow::Error> {
  runtime().block_on(cluster_async(opts))
}

async fn cluster_async(Cluster { instances, config }: Cluster) -> Result<(), anyhow::Error> {
  
  println!("======== cluster start ========");


  let futs = FuturesUnordered::new();

  if instances == 0 {
    anyhow::bail!("instances must be greater than 0")
  }

  let exe = std::env::current_exe().context("failed to get curret exe")?;


  for i in 0..instances {
    let exe = exe.clone();
    let config = config.clone();
    //let config = config.clone();
    futs.push(async move {
      let mut cmd = tokio::process::Command::new(exe);
      cmd.arg("start");
      cmd.arg("--config");
      cmd.arg(&config);
      cmd.env("INSTANCE_ID", &format!("{}", i));
      
      cmd.stdin(std::process::Stdio::inherit());
      cmd.stdout(std::process::Stdio::inherit());
      cmd.stderr(std::process::Stdio::inherit());

      let mut child = cmd.spawn()?;

      let status = child.wait().await?;

      Ok::<_, std::io::Error>(status)
    })
  }

  let results: Vec<ExitStatus> = futs.try_collect().await.context("spawn processes")?;

  println!("======== cluster end ========");
  println!("{:#?}", results);

  Ok(())
}
 
fn token(
  CreateToken {
    config,
    title,
    assume_yes,
  }: CreateToken,
) -> Result<(), anyhow::Error> {
  runtime().block_on(async move {
    
    async fn create(title: String) -> Result<AccessToken, anyhow::Error> {
      
      let key = AccessToken::random_key();

      let token = AccessToken {
        id: AccessToken::uid(),
        key,
        scope: db::access_token::Scope::Global,
        generated_by: GeneratedBy::Cli { title },
        created_at: DateTime::now(),
        last_used_at: None,
        hits: 0,
      };

      AccessToken::insert(&token).await.context("mongodb error ocurred when inserting new access token")?;
      Ok(token)
    }

    shared_init(config).await?;

    let title = {
      if let Some(title) = title {
        title.trim().to_string()
      } else {
        loop {
          let title: String = dialoguer::Input::new()
            .with_prompt("Title for the new access token?")
            .allow_empty(true)
            .interact()?;

          if title.trim().is_empty() {
            println!("The title is required");
            continue;
          } else {
            break title.trim().to_string();
          }
        }
      }
    };

    if assume_yes {
      let token = create(title).await?;
      println!("New global access token generated => {}", token.key);
    } else {
      let confirm = dialoguer::Confirm::new()
        .with_prompt("This will generate a global access token to use in all openstream instances that share this mongodb deployment?")
        .default(true)
        .show_default(true)
        .wait_for_newline(true)
        .interact()?;

      if confirm {
        let token = create(title).await?;
        println!("New global access token generated => {}", token.key)
      } else {
        eprintln!("Operation aborted")
      }
    }

    Ok(())
  })
}

fn create_config(CreateConfig { output }: CreateConfig) -> Result<(), anyhow::Error> {
  eprintln!("creating default config file into {}", output.yellow());

  let file = PathBuf::from(&output);

  let exists = file.metadata().is_ok();

  if exists {
    bail!(
      "file {} already exists, operation aborted",
      file.to_string_lossy()
    );
  }

  std::fs::write(file.clone(), include_bytes!("../../../config.default.toml"))
    .with_context(|| format!("error writing config file to {}", file.to_string_lossy()))?;

  eprintln!("config file created in {}", output.yellow());

  Ok(())
}

use std::path::PathBuf;
use std::process::ExitStatus;
use std::sync::Arc;

use api::storage::StorageServer;
use clap::{Parser, Subcommand};
use config::Config;
use db::access_token::{AccessToken, GeneratedBy};
use db::Model;
use drop_tracer::DropTracer;
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
use serde_util::DateTime;
use shutdown::Shutdown;
// use source::SourceServer;
use stream::StreamServer;
use tokio::runtime::Runtime;

use jemallocator::Jemalloc;

pub mod error;

#[global_allocator]
static ALLOCATOR: Jemalloc = Jemalloc;

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
  #[clap(short, long, default_value_t = String::from("./openstream.toml"))]
  config: String,
}

#[derive(Debug, Parser)]
#[command(about = "Create a cluster of n --instances of `openstream start` processes")]
struct Cluster {
  /// Number of instances to spawn
  #[clap(short, long)]
  instances: u16,

  /// Path to the configuration file (relative to cwd)
  #[clap(short, long, default_value_t = String::from("./openstream.toml"))]
  config: String,
}

#[derive(Debug, Parser)]
#[command(
  about = "Create a new global access token to use in all openstream instances that share the same mongodb deployment"
)]
struct CreateToken {
  /// Path to the configuration file (relative to cwd)
  #[clap(short, long, default_value_t = String::from("./openstream.toml"))]
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
  /// Path to the output file (relative to cwd) to write the default config to.
  /// It can be in .toml or .json format (format is guessed from output filename)
  #[clap(short, long, default_value_t = String::from("./openstream.toml"))]
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
    target: "start",
    "openstream {}{} process started",
    "v".yellow(),
    VERSION.yellow()
  );

  let canonical_config_path = std::fs::canonicalize(config.as_str())
    .with_context(|| format!("error loading config file from {}", config.yellow()))?;

  info!(
    target: "start",
    "loading config file from {}",
    canonical_config_path.to_string_lossy().yellow()
  );

  let config = config::load(config).with_context(|| {
    format!(
      "error loading config file from {}",
      canonical_config_path.to_string_lossy().yellow(),
    )
  })?;

  debug!(target: "start", "config loaded: resolved config: {:#?}", config);

  let client_options = mongodb::options::ClientOptions::parse(config.mongodb.url.as_str())
    .await
    .context("failed to parse mongodb connection string")?;

  info!(target: "start", "mongodb config hosts: {:?}", client_options.hosts);
  info!(
    target: "start",
    "mongodb client compressors: {:?}",
    client_options.compressors
  );

  let client = mongodb::Client::with_options(client_options.clone())
    .context("failed to create mongodb client")?;

  if client.default_database().is_none() {
    bail!("no database specified in config, under [mongodb] url");
  }

  info!(target: "start", "connecting to mongodb and testing transactions support...");

  {
    let test_cl_name = format!("__transactions_test_{}", uid::uid(5));
    let db = client.default_database().unwrap();
    let cl = db.collection::<Document>(&test_cl_name);

    {
      let mut session = client
        .start_session(db::transaction_session_options())
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

  info!(target: "start", "mongodb client connected with transactions support");

  db::init(client, config.mongodb.storage_db_name.clone());

  info!(target: "start", "ensuring mongodb collections...");
  db::ensure_collections()
    .await
    .context("error ensuring mongodb collections and indexes")?;

  Ok(config)
}

fn start(opts: Start) -> Result<(), anyhow::Error> {
  runtime().block_on(start_async(opts))
}

async fn start_async(Start { config }: Start) -> Result<(), anyhow::Error> {
  
  use db::models::deployment::{Deployment, DeploymentState};

  let config = Arc::new(shared_init(config).await?);

  let pid = std::process::id();

  let ffmpeg_path = which::which("ffmpeg")
    .context("error getting ffmpeg path (is ffmpeg installed and available in executable path?)")?;

  info!(
    target: "start",
    "using system ffmpeg from {}",
    ffmpeg_path.to_string_lossy().yellow()
  );

  let local_ip = local_ip_address::local_ip().context("error obtaining local ip")?;
  info!(target: "start", "local ip address: {}", local_ip.yellow());

  // info!("retrieving public ip...");
  // let ip = ip::get_ip_v4().await.context("error obtaining public ip")?;
  // info!("public ip address: {}", ip.yellow());

  let now = DateTime::now();

  let source_ports = match &config.source {
    None => vec![],
    Some(source) => {
      source.addrs.iter().map(|addr| addr.port()).collect()
    }
  };

  let stream_ports = match &config.stream {
    None => vec![],
    Some(stream) => {
      stream.addrs.iter().map(|addr| addr.port()).collect()
    }
  };

  let deployment = Deployment {
    id: Deployment::uid(),
    pid,
    local_ip,
    source_ports,
    stream_ports,
    state: DeploymentState::Active,
    created_at: now,
    updated_at: now,
    dropped_at: None,
  };

  let config::Config {
    mongodb: _,
    ref stream,
    ref source,
    ref api,
    ref storage,
  } = config.as_ref();

  let shutdown = Shutdown::new();
  let drop_tracer = DropTracer::new("main");
  let media_sessions = MediaSessionMap::new(deployment.id.clone(), drop_tracer.clone());

  let futs = futures::stream::FuturesUnordered::new();



  if let Some(source_config) = source {
    // let source = SourceServer::new(
    //   source_config.addrs.clone(),
    //   media_sessions.clone(),
    //   drop_tracer.clone(),
    //   shutdown.clone(),
    // );

    // let fut = source.start()?;

    // futs.push(fut.boxed());

    for addr in source_config.addrs.iter().copied() {
      let fut = source_alt::start(
        deployment.id.clone(),
        addr,
        media_sessions.clone(),
        drop_tracer.clone(),
        shutdown.clone()
      );

      futs.push(async move {
        fut.await?;
        Ok::<(), crate::error::ServerStartError>(())
      }.boxed());
    }
  }

  if let Some(stream_config) = stream {
    let stream = StreamServer::new(
      deployment.id.clone(),
      stream_config.addrs.clone(),
      shutdown.clone(),
      drop_tracer.clone(),
      media_sessions.clone(),
    );
    let fut = stream.start()?;
    futs.push(async move {
      fut.await.map_err(crate::error::ServerStartError::from)?;
      Ok(())
    }.boxed());
  }

  if let Some(api_config) = api {
    let stream_connections_index = db::stream_connection::index::MemIndex::new().await;
    let api = ApiServer::new(
      deployment.id.clone(),
      api_config.addrs.clone(),
      shutdown.clone(),
      drop_tracer.clone(),
      media_sessions.clone(),
      stream_connections_index,
    );
    let fut = api.start()?;
    futs.push(async move {
      fut.await.map_err(crate::error::ServerStartError::from)?;
      Ok(())
    }.boxed());
  }

  if let Some(storage_config) = storage {
    let storage = StorageServer::new(
      deployment.id.clone(),
      storage_config.addrs.clone(),
      shutdown.clone()
    );
    let fut = storage.start()?;
    futs.push(async move {
      fut.await.map_err(crate::error::ServerStartError::from)?;
      Ok(())
    }.boxed());
  }

  // if let Some(router_config) = router {
  //   let router = RouterServer::new(router_config.addrs.clone(), shutdown.clone());
  //   let fut = router.start()?;
  //   futs.push(fut.boxed());
  // }

  db::models::transfer_checkpoint::start_background_task();

  info!(
    target: "start",
    "inserting deployment document: _id={} pid={} local_ip={} ", deployment.id, deployment.pid, deployment.local_ip
  );

  let deployment_id = deployment.id.clone();
  Deployment::insert(&deployment).await?;

  tokio::spawn({
    let shutdown = shutdown.clone();
    let deployment_id = deployment_id.clone();
    async move {
      tokio::signal::ctrl_c()
        .await
        .expect("failed to listen to SIGINT signal");
      
      info!(target: "start", "{} received, starting graceful shutdown", "SIGINT".yellow());
      
      let query = doc! {
        Deployment::KEY_ID: deployment_id,
        Deployment::KEY_STATE: DeploymentState::KEY_ENUM_VARIANT_ACTIVE, 
      };

      let update = doc! {
        "$set": {
          Deployment::KEY_STATE: DeploymentState::KEY_ENUM_VARIANT_CLOSING,
        }
      };

      if let Err(e) = Deployment::cl().update_one(query, update, None).await {
        error!(
          target: "shutdown",
          "error setting deployment state to 'closing' => {} => {:?}", e, e,
        )
      };

      shutdown.shutdown();
    }
  });

  futs.try_collect().await?;

  drop(drop_tracer);

  let now = DateTime::now();

  let query = doc! {
    Deployment::KEY_ID: &deployment_id,
  };

  let update = doc! {
    "$set": {
      Deployment::KEY_STATE: DeploymentState::KEY_ENUM_VARIANT_CLOSED,
      Deployment::KEY_UPDATED_AT: now,
      Deployment::KEY_DROPPED_AT: Some(now),
    }
  };

  info!(
    target: "shutdown",
    "setting deployment {} as closed", deployment_id
  );

  if let Err(e) = Deployment::cl().update_one(query, update, None).await {
    error!(
      target: "shutdown",
      "error setting deployment state to 'closed' => {} => {:?}", e, e,
    )
  } else {
    info!(
      target: "shutdown",
      "deployment {} closed in database", deployment_id
    );
  }

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
    
    async fn create(title: String) -> Result<(AccessToken, String, String), anyhow::Error> {
      
      let key = AccessToken::random_key();

      let media_key = AccessToken::random_media_key();

      let token = AccessToken {
        id: AccessToken::uid(),
        hash: crypt::sha256(&key),
        media_hash: crypt::sha256(&media_key),
        scope: db::access_token::Scope::Global,
        generated_by: GeneratedBy::Cli { title },
        hits: 0,
        created_at: DateTime::now(),
        last_used_at: None,
        deleted_at: None,
      };

      AccessToken::insert(&token).await.context("mongodb error ocurred when inserting new access token")?;
      Ok((token, key, media_key))
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
      let (token, key, media_key) = create(title).await?;
      println!("New global access token generated => {}-{} \ntoken media_key => {}-{}", token.id, key, token.id, media_key)
    } else {
      let confirm = dialoguer::Confirm::new()
        .with_prompt("This will generate a global access token to use in all openstream instances that share this mongodb deployment?")
        .default(true)
        .show_default(true)
        .wait_for_newline(true)
        .interact()?;

      if confirm {
        let (token, key, media_key) = create(title).await?;
        println!("New global access token generated => {}-{} \nToken media_key => {}-{}", token.id, key, token.id, media_key)
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

  let contents = if output.ends_with(".json") {
    include_str!("../../../../openstream.sample.json")
  } else {
    include_str!("../../../../openstream.sample.toml")
  };

  std::fs::write(file.clone(), contents)
    .with_context(|| format!("error writing config file to {}", file.to_string_lossy()))?;

  eprintln!("config file created in {}", output.yellow());

  Ok(())
}

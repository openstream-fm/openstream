use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::Config;
use db::access_token::{AccessToken, GeneratedBy};
use db::Model;
use futures::{FutureExt, TryStreamExt};
use log::*;

use api::ApiServer;
use defer_lite::defer;
use mongodb::bson::doc;
use mongodb::bson::Document;
use owo_colors::*;
use router::RouterServer;
use shutdown::Shutdown;
use source::SourceServer;
use stream::StreamServer;
use tokio::runtime::Runtime;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
  cmd()
}

fn cmd() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();
  match cli.command {
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

async fn shared_init(config: String) -> Result<Config, Box<dyn std::error::Error>> {
  logger::init();
  let _ = dotenv::dotenv();

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

  info!("connecting to mongodb (and testing transactions support)...");
  {
    let test_cl_name = "  __transactions_test";
    let db = client.default_database().unwrap();
    let cl = db.collection::<Document>(test_cl_name);

    {
      let mut session = client.start_session(None).await?;
      session.start_transaction(None).await?;

      let cl2 = cl.clone();
      defer! {
        tokio::task::block_in_place(|| {
          tokio::runtime::Handle::current().block_on(async move {
            let _ = cl2.drop(None).await;
          })
        })
      }

      cl.insert_one_with_session(doc! {}, None, &mut session)
        .await?;

      cl.delete_many_with_session(doc! {}, None, &mut session)
        .await?;

      session.commit_transaction().await?;
    }
  }

  info!("mongodb client connected and OK");

  db::init(client);

  info!("ensuring mongodb collections...");
  db::ensure_collections().await?;

  Ok(config)
}

fn start(opts: Start) -> Result<(), Box<dyn std::error::Error>> {
  runtime().block_on(start_async(opts))
}

async fn start_async(Start { config }: Start) -> Result<(), Box<dyn std::error::Error>> {
  let config = shared_init(config).await?;

  let ffmpeg_path = which::which("ffmpeg")?;

  info!(
    "using system ffmpeg from {}",
    ffmpeg_path.to_string_lossy().yellow()
  );

  info!("retrieving public ip...");
  let ip = ip::get_ip_v4().await?;
  info!("public ip obtained: {}", ip.yellow());

  let config::Config {
    mongodb: _,
    stream,
    source,
    api,
    router,
  } = config;

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
      source_config.receiver.addrs,
      source_config.broadcaster.addrs,
      shutdown.clone(),
    );

    let fut = source.start()?;

    futs.push(fut.boxed());
  }

  if let Some(stream_config) = stream {
    let stream = StreamServer::new(stream_config.addrs, shutdown.clone());

    let fut = stream.start()?;

    futs.push(fut.boxed());
  }

  if let Some(api_config) = api {
    let api = ApiServer::new(api_config.addrs, shutdown.clone());
    let fut = api.start()?;
    futs.push(fut.boxed());
  }

  if let Some(router_config) = router {
    let router = RouterServer::new(router_config.addrs, shutdown.clone());
    let fut = router.start()?;
    futs.push(fut.boxed());
  }

  futs.try_collect().await?;

  Ok(())
}

fn token(
  CreateToken {
    config,
    title,
    assume_yes,
  }: CreateToken,
) -> Result<(), Box<dyn std::error::Error>> {
  runtime().block_on(async move {
    async fn create(title: String) -> Result<AccessToken, Box<dyn std::error::Error>> {
      let token = AccessToken {
        id: AccessToken::uid(),
        key: AccessToken::random_key(),
        scope: db::access_token::Scope::Global,
        generated_by: GeneratedBy::Cli { title },
        created_at: chrono::Utc::now(),
        last_used_at: None,
        hits: 0,
      };

      AccessToken::insert(&token).await?;
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

  std::fs::write(file, include_bytes!("../../../config.default.toml"))?;

  eprintln!("config file created in {}", canonical_config_path.yellow());

  Ok(())
}

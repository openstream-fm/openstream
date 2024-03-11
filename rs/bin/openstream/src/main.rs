use std::path::PathBuf;
use std::sync::Arc;

use api::storage::StorageServer;
use api::ws_stats::WsStatsServer;
use clap::{Parser, Subcommand};
use config::Config;
use db::access_token::{AccessToken, GeneratedBy};
use db::Model;
use db::admin::Admin;
use db::registry::Registry;
use drop_tracer::DropTracer;
use futures::{FutureExt, TryStreamExt};
use log::*;

use anyhow::{bail, Context};
use api::ApiServer;
use defer_lite::defer;
use mongodb::bson::doc;
use mongodb::bson::Document;
use serde_util::DateTime;
use shutdown::Shutdown;
use stream::StreamServer;
use assets::StaticServer;
use tokio::runtime::Runtime;

use jemallocator::Jemalloc;

pub mod error;

#[global_allocator]
static ALLOCATOR: Jemalloc = Jemalloc;

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
  // Cluster(Cluster),
  CreateConfig(CreateConfig),
  CreateToken(CreateToken),
  CreateAdmin(CreateAdmin),
  CheckDb(CheckDb),
}

#[derive(Debug, Parser)]
#[command(about = "Start openstream server(s) from a config file")]
struct Start {
  /// Path to the configuration file (relative to cwd)
  #[clap(short, long, default_value_t = String::from("./openstream.toml"))]
  config: String,
}

#[derive(Debug, Parser)]
#[command(about = "Check that all documents in the database can be deserialized into their rust form")]
struct CheckDb {
  /// Path to the configuration file (relative to cwd)
  #[clap(short, long, default_value_t = String::from("./openstream.toml"))]
  config: String,
}

// #[derive(Debug, Parser)]
// #[command(about = "Create a cluster of n --instances of `openstream start` processes")]
// struct Cluster {
//   /// Number of instances to spawn
//   #[clap(short, long)]
//   instances: u16,

//   /// Path to the configuration file (relative to cwd)
//   #[clap(short, long, default_value_t = String::from("./openstream.toml"))]
//   config: String,
// }

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
#[command(
  about = "Create a new administrator in all deployments that share the same mongodb config"
)]
struct CreateAdmin {
  /// Path to the configuration file (relative to cwd)
  #[clap(short, long, default_value_t = String::from("./openstream.toml"))]
  config: String,

  /// Do not ask for confirmation
  #[clap(short = 'y', long, default_value_t = false)]
  assume_yes: bool,

  /// Use this value for the first_name instead of asking
  #[clap(long)]
  first_name: Option<String>,

  /// Use this value for the last_name instead of asking
  #[clap(long)]
  last_name: Option<String>,

  /// Use this value for the email instead of asking
  #[clap(long)]
  email: Option<String>,

  /// Use this value for the password instead of asking
  #[clap(long)]
  password: Option<String>,
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
    // Command::Cluster(opts) => cluster(opts),
    Command::Start(opts) => start(opts),
    Command::CreateConfig(opts) => create_config(opts),
    Command::CreateToken(opts) => token(opts),
    Command::CreateAdmin(opts) => create_admin(opts),
    Command::CheckDb(opts) => check_db(opts),
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
  let _ = dotenv::dotenv();

  {
    use owo_colors::*;
    info!(
      target: "start",
      "openstream {}{} process started",
      "v".yellow(),
      VERSION.yellow()
    );
  }


  let config: Config = {
    if config.as_str() != "none" {
      let canonical_config_path = std::fs::canonicalize(config.as_str())
        .with_context(|| { 
          use owo_colors::*;
          format!("error loading config file from {}", config.yellow())
      })?;

      {
        use owo_colors::*;
        info!(
          target: "start",
          "loading config file from {}",
          canonical_config_path.to_string_lossy().yellow()
        );
      }

      config::load(Some(config)).with_context(|| {
        use owo_colors::*;
        format!(
          "error loading config file from {}",
          canonical_config_path.to_string_lossy().yellow(),
        )
      })?
    } else {
      info!(target: "start", "loading config only from env variables");
      config::load(Option::<&str>::None).context("error loading config from env variables")?
    }
  };
  

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
        .context("mongodb error when inserting test document into test collection")?;

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

fn check_db(opts: CheckDb) -> Result<(), anyhow::Error> {
  runtime().block_on(check_db_async(opts))
}

async fn check_db_async(opts: CheckDb) -> Result<(), anyhow::Error> {
  logger::init();
  shared_init(opts.config).await?;

  let registry = Registry::global();

  let map = registry.check_all().await;

  let mut has_errors = false;

  info!("=================");

  for (name, result) in map.iter() {
    match result {
      Ok(n) => {
        {
          use owo_colors::*;
          info!("collection {} is ok, checked {} documents", name.yellow(), n.yellow());
        }
      },
      Err(e) => {
        has_errors = true;
        {
          use owo_colors::*;
          warn!("collection {} failed with error: {}", name.red(), e.red());
        }
      }
    }
  };

  if has_errors {
    warn!("status: fail");
    info!("=================");
    bail!("status: fail");
  } else {
    info!("=================");
    info!("status: success");
    Ok(())
  }
}

#[cfg(feature = "tracing")]
fn start_tracing() {
  let console_addr = std::env::var("TOKIO_CONSOLE_BIND").unwrap_or_else(|_| format!("{}:{}", console_subscriber::Server::DEFAULT_IP, console_subscriber::Server::DEFAULT_PORT));
  
  {
     use owo_colors::*;
     info!(
      target: "start",
      "feature tracing enabled"
    );
     info!(
       target: "start",
       "intializing console subscriber server on addr {}", console_addr.yellow())
  }

  console_subscriber::init();
}

#[cfg(not(feature = "tracing"))]
fn start_tracing() {
  info!(
    target: "start",
    "feature tracing not enabled"
  );
}

async fn start_async(Start { config }: Start) -> Result<(), anyhow::Error> {
  

  logger::init();
  start_tracing();  
  
  // console_subscriber::Builder::default().with_default_env().server_addr(addr);
  
  //console_subscriber::init();

  use db::models::deployment::{Deployment, DeploymentState};

  let config = Arc::new(shared_init(config).await?);

  let pid = std::process::id();

  let ffmpeg_path = which::which("ffmpeg")
    .context("error getting ffmpeg path (is ffmpeg installed and available in executable path?)")?;

  {
    use owo_colors::OwoColorize;

    info!(
      target: "start",
      "using system ffmpeg from {}",
      ffmpeg_path.to_string_lossy().yellow()
    );
  }

  let local_ip = local_ip_address::local_ip().context("error obtaining local ip")?;

  {
    use owo_colors::*;
    info!(target: "start", "local ip address: {}", local_ip.yellow());
  }
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

  let api_ports = match &config.api {
    None => vec![],
    Some(api) => {
      api.addrs.iter().map(|addr| addr.port()).collect()
    }
  };

  let deployment = Deployment {
    id: Deployment::uid(),
    pid,
    local_ip,
    source_ports,
    stream_ports,
    api_ports,
    state: DeploymentState::Active,
    created_at: now,
    updated_at: now,
    health_checked_at: Some(now),
    dropped_at: None,
    abnormally_closed: false,
  };

  let config::Config {
    mongodb: _,
    ref stream,
    ref source,
    ref api,
    ref storage,
    ref assets,
    ref smtp,
    ref payments,
    ref ws_stats,
  } = config.as_ref();

  db::access_token::AccessToken::start_autoremove_job();

  let mailer = mailer::send::Mailer {
    hostname: smtp.hostname.clone(),
    port: smtp.port,
    password: smtp.password.clone(),
    username: smtp.username.clone(),
  };

  let r = {
    let shutdown = Shutdown::new();
    let drop_tracer = DropTracer::new("main");
    //let media_sessions = MediaSessionMap::new(deployment.id.clone(), drop_tracer.clone());
    let media_sessions = media::MediaSessionMap::new(deployment.id.clone(), drop_tracer.clone(), shutdown.clone());

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
      
      let payments_client = payments::client::PaymentsClient::new(payments.base_url.clone(), payments.access_token.clone());
      let stream_connections_index = db::stream_connection::index::MemIndex::new().await;

      let api = ApiServer::new(
        deployment.id.clone(),
        api_config.addrs.clone(),
        shutdown.clone(),
        drop_tracer.clone(),
        media_sessions.clone(),
        stream_connections_index,
        payments_client,
        mailer,
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

    if let Some(ws_stats_config) = ws_stats {
      let ws_stats = WsStatsServer::new(
        deployment.id.clone(),
        ws_stats_config.addrs.clone(),
        drop_tracer.clone(),
        shutdown.clone(),
      );
      let fut = ws_stats.start()?;
      futs.push(async move {
        fut.await.map_err(crate::error::ServerStartError::from)?;
        Ok(())
      }.boxed());
    }

    if let Some(static_config) = assets {
      let assets = StaticServer::new(
        static_config.addrs.clone(),
        shutdown.clone(),
      );
      let fut = assets.start()?;
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

    tokio::spawn(db::deployment::start_health_check_job(deployment_id.clone()));
    tokio::spawn(db::station_picture::upgrade_images_if_needed());
    tokio::spawn(media::health::health_shutdown_job());
    tokio::spawn(db::probe::start_probe_background_job());

    tokio::spawn({
      let shutdown = shutdown.clone();
      let deployment_id = deployment_id.clone();
      async move {
        tokio::signal::ctrl_c()
          .await
          .expect("failed to listen to SIGINT signal");
        
        {
          use owo_colors::*;
          info!(target: "start", "{} received, starting graceful shutdown", "SIGINT".yellow());
        }

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

    let r: Result<(), crate::error::ServerStartError> = futs.try_collect().await;

    drop(drop_tracer);

    r
  };

  let now = DateTime::now();

  let query = doc! {
    Deployment::KEY_ID: &deployment.id,
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
    "setting deployment {} as closed", deployment.id
  );

  if let Err(e) = Deployment::cl().update_one(query, update, None).await {
    error!(
      target: "shutdown",
      "error setting deployment state to 'closed' => {} => {:?}", e, e,
    )
  } else {
    info!(
      target: "shutdown",
      "deployment {} closed in database", deployment.id
    );
  }

  r.context("A server instance returned a non ok result")?;

  Ok(())
  
}

// fn cluster(opts: Cluster) -> Result<(), anyhow::Error> {
//   runtime().block_on(cluster_async(opts))
// }

// async fn cluster_async(Cluster { instances, config }: Cluster) -> Result<(), anyhow::Error> {
//   logger::init();
//   println!("======== cluster start ========");

//   let futs = FuturesUnordered::new();

//   if instances == 0 {
//     anyhow::bail!("instances must be greater than 0")
//   }

//   let exe = std::env::current_exe().context("failed to get curret exe")?;

//   for i in 0..instances {
//     let exe = exe.clone();
//     let config = config.clone();
//     //let config = config.clone();
//     futs.push(async move {
//       let mut cmd = tokio::process::Command::new(exe);
//       cmd.arg("start");
//       cmd.arg("--config");
//       cmd.arg(&config);
//       cmd.env("INSTANCE_ID", &format!("{}", i));

//       cmd.stdin(std::process::Stdio::inherit());
//       cmd.stdout(std::process::Stdio::inherit());
//       cmd.stderr(std::process::Stdio::inherit());

//       let mut child = cmd.spawn()?;

//       let status = child.wait().await?;

//       Ok::<_, std::io::Error>(status)
//     })
//   }

//   let results: Vec<ExitStatus> = futs.try_collect().await.context("spawn processes")?;

//   println!("======== cluster end ========");
//   println!("{:#?}", results);

//   Ok(())
// }

fn token(
  CreateToken {
    config,
    title,
    assume_yes,
  }: CreateToken,
) -> Result<(), anyhow::Error> {
  runtime().block_on(async move {
    logger::init();

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

fn create_admin(
  CreateAdmin {
    config,
    first_name,
    last_name,
    email,
    password,
    assume_yes,
  }: CreateAdmin,
) -> Result<(), anyhow::Error> {
  runtime().block_on(async move {
    logger::init();

    async fn create(first_name: String, last_name: String, email: String, password: String) -> Result<Admin, anyhow::Error> {
      
      let now = DateTime::now();

      let hash = crypt::hash(&password);

      let admin = Admin {
        id: Admin::uid(),
        first_name,
        last_name,
        email,
        password: hash,
        language: None,
        system_metadata: Default::default(),
        created_at: now,
        updated_at: now,
        deleted_at: None,
      };

      Admin::insert(&admin).await.context("mongodb error ocurred when inserting new admin")?;
      Ok(admin)
    }

    shared_init(config).await?;

    let first_name = {
      if let Some(v) = first_name {
        let v = v.trim().to_string();
        if v.is_empty() {
          bail!("First name is required");
        } else {
          v
        }
      } else {
        loop {
          let v: String = dialoguer::Input::new()
            .with_prompt("First name of the new administrator?")
            .allow_empty(true)
            .interact()?;

          if v.trim().is_empty() {
            println!("The first name is required");
            continue;
          } else {
            break v.trim().to_string();
          }
        }
      }
    };

    let last_name = {
      if let Some(v) = last_name {
        let v = v.trim().to_string();
        if v.is_empty() {
          bail!("Last name is required");
        } else {
          v
        }
      } else {
        loop {
          let v: String = dialoguer::Input::new()
            .with_prompt("Last name of the new administrator?")
            .allow_empty(true)
            .interact()?;

          if v.trim().is_empty() {
            println!("Last name is required");
            continue;
          } else {
            break v.trim().to_string();
          }
        }
      }
    };

    let email = {
      if let Some(v) = email {
        let v = v.trim().to_string(); 
        if !validate::email::is_valid_email(&v) {
          bail!("Email address is invalid");
        } else {
          v
        }
      } else {
        loop {
          let v: String = dialoguer::Input::new()
            .with_prompt("Email address of the new administrator?")
            .allow_empty(true)
            .interact()?;

          if v.trim().is_empty() {
            println!("Email address name is required");
            continue;
          } else if !validate::email::is_valid_email(&v) {
            println!("Email address is invalid");
            continue
          } else {        
            break v.trim().to_string();
          }
        }
      }
    };

    let password = {
      if let Some(v) = password {
        if v.len() < 8 {
          bail!("Password must have 8 characters or more");
        } else {
          v
        }
      } else {
        loop {
          let v: String = dialoguer::Input::new()
            .with_prompt("Password for the new administrator?")
            .allow_empty(true)
            .interact()?;

          if v.len() < 8 {
            println!("Password must have 8 characters or more");
            continue;
          } else {        
            break v
          }
        }
      }
    };


    if assume_yes {
      let admin = create(first_name, last_name, email, password).await?;
      println!("New admin created: {} {} <{}>", admin.first_name, admin.last_name, admin.email);
    } else {
      let confirm = dialoguer::Confirm::new()
        .with_prompt("This will generate a global admininistrator in all openstream instances that share this mongodb deployment, continue?")
        .default(true)
        .show_default(true)
        .wait_for_newline(true)
        .interact()?;

      if confirm {
        let admin = create(first_name, last_name, email, password).await?;
        println!("New admin created: {} {} <{}>", admin.first_name, admin.last_name, admin.email);
        } else {
        eprintln!("Operation aborted")
      }
    }

    Ok(())
  })
}

fn create_config(CreateConfig { output }: CreateConfig) -> Result<(), anyhow::Error> {
  
  logger::init();
  
  {
    use owo_colors::*;
    eprintln!("creating default config file into {}", output.yellow());
  }

  let file = PathBuf::from(&output);

  let exists = file.metadata().is_ok();

  if exists {
    bail!(
      "file {} already exists, operation aborted",
      file.to_string_lossy()
    );
  }

  let contents = if output.ends_with(".json") || output.ends_with(".jsonc") {
    include_str!("../../../../openstream.sample.jsonc")
  } else {
    include_str!("../../../../openstream.sample.toml")
  };

  std::fs::write(file.clone(), contents)
    .with_context(|| format!("error writing config file to {}", file.to_string_lossy()))?;

  {
    use owo_colors::*;
    eprintln!("config file created in {}", output.yellow());
  }

  Ok(())
}

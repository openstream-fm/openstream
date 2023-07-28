#![allow(unreachable_code)]

use anyhow::Context;
use config::Config;
use futures_util::TryStreamExt;
use log::*;
use mongodb::bson::doc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  // anyhow::bail!("already migrated");
  shared_init(String::from("./openstream.toml")).await?;
  migrate().await?;
  Ok(())
}

async fn shared_init(config: String) -> Result<Config, anyhow::Error> {
  use owo_colors::*;

  logger::init();
  let _ = dotenv::dotenv();

  let canonical_config_path = std::fs::canonicalize(config.as_str())
    .with_context(|| format!("error loading config file from {}", config.yellow()))?;

  info!(
    "loading config file from {}",
    canonical_config_path.to_string_lossy().yellow()
  );

  let config = config::load(Some(config)).with_context(|| {
    format!(
      "error loading config file from {}",
      canonical_config_path.to_string_lossy().yellow(),
    )
  })?;

  debug!("config loaded: resolved config: {:#?}", config);

  let client_options = mongodb::options::ClientOptions::parse(config.mongodb.url.as_str())
    .await
    .context("failed to parse mongodb connection string")?;

  info!("mongodb config hosts: {:?}", client_options.hosts);
  info!(
    "mongodb client compressors: {:?}",
    client_options.compressors
  );

  let client =
    mongodb::Client::with_options(client_options).context("failed to create mongodb client")?;

  if client.default_database().is_none() {
    anyhow::bail!("no database specified in config, under [mongodb] url");
  }

  info!("mongodb client created");

  db::init(client, config.mongodb.storage_db_name.clone());

  // info!("ensuring mongodb collections...");
  // db::ensure_collections()
  //   .await
  //   .context("error ensuring mongodb collections and indexes")?;

  Ok(config)
}

async fn migrate() -> Result<(), mongodb::error::Error> {
  use db::stream_connection::lite::StreamConnectionLite;
  use db::stream_connection::StreamConnection;
  use db::Model;

  db::run_transaction!(session => {

    info!("getting full documents");
    let cursor = tx_try!(StreamConnection::cl().find(doc!{}, None).await);
    let full_documents: Vec<StreamConnection> = tx_try!(cursor.try_collect().await);

    info!("got {} full documents", full_documents.len());

    info!("removing old documents");
    let r = tx_try!(StreamConnectionLite::cl().delete_many(doc!{}, None).await);
    info!("{} old documents deleted", r.deleted_count);

    info!("mapping documents");
    let documents: Vec<StreamConnectionLite> = full_documents.into_iter().map(StreamConnectionLite::from).collect();

    info!("{} documents mapped", documents.len());

    info!("inserting {} documents", documents.len());

    let r = tx_try!(StreamConnectionLite::cl().insert_many(documents, None).await);

    info!("{} documents inserted", r.inserted_ids.len());
  });

  info!("OK");
  info!("Bye!");

  Ok(())
}

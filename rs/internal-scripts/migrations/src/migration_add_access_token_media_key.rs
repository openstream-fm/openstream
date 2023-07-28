#![allow(unreachable_code)]

use anyhow::Context;
use config::Config;
use db::access_token::{GeneratedBy, Scope};
use log::*;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  anyhow::bail!("already migrated");
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct PrevAccessToken {
  #[serde(rename = "_id")]
  pub id: String,

  pub key: String,

  /// the media_key is used to access streams and files with access token scope directly
  /// from the client without exposing a full access token
  // pub media_key: String,

  #[serde(flatten)]
  pub scope: Scope,

  #[serde(flatten)]
  pub generated_by: GeneratedBy,

  pub last_used_at: Option<DateTime>,

  #[serde(with = "serde_util::as_f64")]
  pub hits: u64,

  pub created_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

async fn migrate() -> Result<(), mongodb::error::Error> {
  use db::models::access_token::AccessToken;
  use db::Model;

  let mut count: usize = 0;
  db::run_transaction!(session => {
    let mut cursor = tx_try!(AccessToken::cl_as::<PrevAccessToken>().find_with_session(None, None, &mut session).await);
    while let Some(document) = cursor.next(&mut session).await {
      let src = tx_try!(document);
      let media_key = AccessToken::random_media_key();
      let media_hash = crypt::sha256(media_key);
      let target = AccessToken {
        id: src.id,
        hash: crypt::sha256(src.key),
        media_hash,
        created_at: src.created_at,
        deleted_at: src.deleted_at,
        last_used_at: src.last_used_at,
        generated_by: src.generated_by,
        hits: src.hits,
        scope: src.scope,
      };



      let r = tx_try!(AccessToken::replace_with_session(&target.id, &target, &mut session).await);
      assert_eq!(r.matched_count, 1);
      count += 1;
    };
  });

  info!("{} access tokens modified", count);
  info!("Bye!");

  Ok(())
}

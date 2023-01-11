use anyhow::Context;
use db::{account::Account, audio_chunk::AudioChunk, audio_file::AudioFile, Model};
use futures::TryStreamExt;
use log::*;
use mongodb::bson::doc;

const BASE_ACCOUNT_ID: &str = "2tu6aps9";
const C: usize = 1000;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  craete_test_accounts().await
}

async fn craete_test_accounts() -> Result<(), anyhow::Error> {
  use owo_colors::*;
  logger::init();
  //let _ = dotenv::dotenv();

  // let canonical_config_path = std::fs::canonicalize(config.as_str())
  //   .with_context(|| format!("error loading config file from {}", config.yellow()))?;

  let config = "./config.toml";

  info!("loading config file from {}", config.yellow());

  let config = config::load(config)
    .with_context(|| format!("error loading config file from {}", config.yellow(),))?;

  debug!("config loaded: resolved config: {:#?}", config);

  let client = mongodb::Client::with_uri_str(config.mongodb.url.as_str())
    .await
    .context("failed to create mongodb client")?;

  if client.default_database().is_none() {
    anyhow::bail!("no database specified in config, under [mongodb] url");
  }

  db::init(client);

  info!("ensuring mongodb collections...");
  db::ensure_collections()
    .await
    .context("error ensuring mongodb collections and indexes")?;

  let account = match Account::get_by_id(BASE_ACCOUNT_ID).await? {
    None => anyhow::bail!("cannot find account with id {BASE_ACCOUNT_ID}"),
    Some(account) => account,
  };

  for i in 1..=C {
    create_test_account(i, account.clone()).await?
  }

  println!("Done!");

  Ok(())
}

async fn create_test_account(i: usize, base: Account) -> Result<(), anyhow::Error> {
  info!("creating test account {i} of {C}");
  let account_id = format!("test{i}");
  let now = serde_util::DateTime::now();
  let account = Account {
    id: account_id.clone(),
    name: format!("Test Account {i}"),
    created_at: now,
    updated_at: now,
    source_password: Account::random_source_password(),
    limits: base.limits.clone(),
    owner_id: base.owner_id.clone(),
    system_metadata: base.system_metadata.clone(),
    user_metadata: base.user_metadata.clone(),
  };

  Account::insert(account).await?;

  let filter = doc! { AudioFile::KEY_ACCOUNT_ID: &base.id };

  let files: Vec<AudioFile> = AudioFile::cl()
    .find(filter, None)
    .await?
    .try_collect()
    .await?;

  for base in files {
    info!("{} - duplicating file {}", account_id, base.filename);
    let file_id = AudioFile::uid();
    let file = AudioFile {
      id: file_id.clone(),
      account_id: account_id.clone(),
      created_at: now,
      bytes_sec: base.bytes_sec,
      chunk_count: base.chunk_count,
      chunk_duration_ms: base.chunk_duration_ms,
      chunk_len: base.chunk_len,
      duration_ms: base.duration_ms,
      filename: base.filename.clone(),
      len: base.len,
      md5: base.md5.clone(),
      metadata: base.metadata.clone(),
    };

    AudioFile::insert(file).await?;

    let filter = doc! { AudioChunk::KEY_AUDIO_FILE_ID: &base.id };
    let chunks: Vec<AudioChunk> = AudioChunk::cl()
      .find(filter, None)
      .await?
      .try_collect()
      .await?;

    info!("{} - duplicating {} chunks", account_id, chunks.len());

    for base in chunks {
      let chunk = AudioChunk {
        id: AudioChunk::uid(),
        audio_file_id: file_id.clone(),
        account_id: account_id.clone(),
        created_at: now,
        bytes_sec: base.bytes_sec,
        data: base.data,
        duration_ms: base.duration_ms,
        end_ms: base.end_ms,
        i: base.i,
        len: base.len,
        start_ms: base.start_ms,
      };

      AudioChunk::insert(chunk).await?;
    }
  }

  Ok(())
}

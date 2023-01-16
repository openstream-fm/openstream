use anyhow::Context;
use db::{
  account::Account,
  audio_chunk::AudioChunk,
  audio_file::AudioFile,
  models::user_account_relation::{UserAccountRelation, UserAccountRelationKind},
  run_transaction, Model,
};
use futures::{StreamExt, TryStreamExt};
use log::*;
use mongodb::bson::doc;

const BASE_ACCOUNT_ID: &str = "erxppjmd";
const C: usize = 10_000;

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

  db::init(client, Some("openstream_storage".into()));

  info!("ensuring mongodb collections...");
  db::ensure_collections()
    .await
    .context("error ensuring mongodb collections and indexes")?;

  let account = match Account::get_by_id(BASE_ACCOUNT_ID).await? {
    None => anyhow::bail!("cannot find account with id {BASE_ACCOUNT_ID}"),
    Some(account) => account,
  };

  let filter = doc! {
    UserAccountRelation::KEY_ACCOUNT_ID: &account.id,
    UserAccountRelation::KEY_KIND: UserAccountRelationKind::TAG_OWNER
  };

  let owner_relation = UserAccountRelation::get(filter)
    .await?
    .expect("cannot find owner relation for acccount");

  let user_id = owner_relation.user_id;

  for i in 1..=C {
    create_test_account(i, &user_id, account.clone()).await?
  }

  println!("Done!");

  Ok(())
}

async fn create_test_account(
  i: usize,
  user_id: impl ToString,
  base: Account,
) -> Result<(), anyhow::Error> {
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
    system_metadata: base.system_metadata.clone(),
    user_metadata: base.user_metadata.clone(),
  };

  let relation = UserAccountRelation {
    id: UserAccountRelation::uid(),
    user_id: user_id.to_string(),
    account_id: account.id.clone(),
    kind: UserAccountRelationKind::Owner,
    created_at: now,
  };

  run_transaction!(session => {
    tx_try!(Account::insert_with_session(&account, &mut session).await);
    tx_try!(UserAccountRelation::insert_with_session(&relation, &mut session).await)
  });

  let filter = doc! { AudioFile::KEY_ACCOUNT_ID: &base.id };

  let files: Vec<AudioFile> = AudioFile::cl()
    .find(filter, None)
    .await?
    .take(1)
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
      sha256: base.sha256.clone(),
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

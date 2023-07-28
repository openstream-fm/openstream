use anyhow::Context;
use db::{
  audio_chunk::AudioChunk, audio_file::AudioFile, run_transaction, station::Station, Model,
};
use futures::{StreamExt, TryStreamExt};
use log::*;
use mongodb::bson::doc;

const BASE_STATION_ID: &str = "zrmgqj2f";
const C: usize = 10_000;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  craete_test_stations().await
}

async fn craete_test_stations() -> Result<(), anyhow::Error> {
  use owo_colors::*;
  logger::init();
  //let _ = dotenv::dotenv();

  // let canonical_config_path = std::fs::canonicalize(config.as_str())
  //   .with_context(|| format!("error loading config file from {}", config.yellow()))?;

  let config = "./openstream.toml";

  info!("loading config file from {}", config.yellow());

  let config = config::load(Some(config))
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

  let station = match Station::get_by_id(BASE_STATION_ID).await? {
    None => anyhow::bail!("cannot find station with id {BASE_STATION_ID}"),
    Some(station) => station,
  };

  for i in 1..=C {
    create_test_station(i, station.clone()).await?
  }

  println!("Done!");

  Ok(())
}

async fn create_test_station(i: usize, base: Station) -> Result<(), anyhow::Error> {
  info!("creating test station {i} of {C}");
  let station_id = format!("test{i}");
  let now = serde_util::DateTime::now();
  let station = Station {
    id: station_id.clone(),
    account_id: base.account_id,

    name: format!("Test Station {i}"),

    playlist_is_randomly_shuffled: false,
    source_password: Station::random_source_password(),

    system_metadata: base.system_metadata.clone(),
    user_metadata: base.user_metadata.clone(),

    created_at: now,
    updated_at: now,
    deleted_at: None,

    ..base
  };

  run_transaction!(session => {
    tx_try!(Station::insert_with_session(&station, &mut session).await);
  });

  let filter = doc! { AudioFile::KEY_STATION_ID: &base.id };

  let files: Vec<AudioFile> = AudioFile::cl()
    .find(filter, None)
    .await?
    .take(1)
    .try_collect()
    .await?;

  for base in files {
    info!("{} - duplicating file {}", station_id, base.filename);
    let order = AudioFile::next_max_order(&station_id, None).await?;
    let file_id = AudioFile::uid();
    let file = AudioFile {
      id: file_id.clone(),
      station_id: station_id.clone(),
      bytes_sec: base.bytes_sec,
      chunk_count: base.chunk_count,
      chunk_duration_ms: base.chunk_duration_ms,
      chunk_len: base.chunk_len,
      duration_ms: base.duration_ms,
      filename: base.filename.clone(),
      len: base.len,
      sha256: base.sha256.clone(),
      metadata: base.metadata.clone(),
      order,
      created_at: now,
    };

    AudioFile::insert(file).await?;

    let filter = doc! { AudioChunk::KEY_AUDIO_FILE_ID: &base.id };
    let chunks: Vec<AudioChunk> = AudioChunk::cl()
      .find(filter, None)
      .await?
      .try_collect()
      .await?;

    info!("{} - duplicating {} chunks", station_id, chunks.len());

    for base in chunks {
      let chunk = AudioChunk {
        id: AudioChunk::uid(),
        audio_file_id: file_id.clone(),
        station_id: station_id.clone(),
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

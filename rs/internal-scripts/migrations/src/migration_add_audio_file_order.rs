// use anyhow::Context;
// use config::Config;
// use db::audio_file::Metadata;
// use log::*;
// use serde::{Deserialize, Serialize};
// use serde_util::{as_f64, DateTime};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  anyhow::bail!("already migrated");
  // shared_init(String::from("./openstream.toml")).await?;
  // migrate().await?;
  // Ok(())
}

// async fn shared_init(config: String) -> Result<Config, anyhow::Error> {
//   use owo_colors::*;

//   logger::init();
//   let _ = dotenv::dotenv();

//   let canonical_config_path = std::fs::canonicalize(config.as_str())
//     .with_context(|| format!("error loading config file from {}", config.yellow()))?;

//   info!(
//     "loading config file from {}",
//     canonical_config_path.to_string_lossy().yellow()
//   );

//   let config = config::load(config).with_context(|| {
//     format!(
//       "error loading config file from {}",
//       canonical_config_path.to_string_lossy().yellow(),
//     )
//   })?;

//   debug!("config loaded: resolved config: {:#?}", config);

//   let client_options = mongodb::options::ClientOptions::parse(config.mongodb.url.as_str())
//     .await
//     .context("failed to parse mongodb connection string")?;

//   info!("mongodb config hosts: {:?}", client_options.hosts);
//   info!(
//     "mongodb client compressors: {:?}",
//     client_options.compressors
//   );

//   let client = mongodb::Client::with_options(client_options.clone())
//     .context("failed to create mongodb client")?;

//   if client.default_database().is_none() {
//     anyhow::bail!("no database specified in config, under [mongodb] url");
//   }

//   info!("mongodb client created");

//   db::init(client, config.mongodb.storage_db_name.clone());

//   info!("ensuring mongodb collections...");
//   db::ensure_collections()
//     .await
//     .context("error ensuring mongodb collections and indexes")?;

//   Ok(config)
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// #[macros::keys]
// pub struct PrevAudioFile {
//   #[serde(rename = "_id")]
//   pub id: String,
//   pub station_id: String,
//   pub sha256: String,

//   #[serde(with = "as_f64")]
//   pub len: u64,

//   pub duration_ms: f64,

//   #[serde(with = "as_f64")]
//   pub bytes_sec: usize,

//   #[serde(with = "as_f64")]
//   pub chunk_count: usize,

//   #[serde(with = "as_f64")]
//   pub chunk_len: usize,

//   pub chunk_duration_ms: f64,

//   pub filename: String,

//   pub metadata: Metadata,

//   pub created_at: DateTime,
// }

// async fn migrate() -> Result<(), mongodb::error::Error> {
//   use db::models::audio_file::AudioFile;
//   use db::models::increment_station_audio_file_order::IncrementStationAudioFileOrder;
//   use db::Incrementer;
//   use db::Model;
//   use mongodb::bson::doc;

//   let count = db::run_transaction!(session => {
//     let cl = db::models::audio_file::AudioFile::cl_as::<PrevAudioFile>();

//     let mut count: u64 = 0;

//     let mut cursor = tx_try!(cl.find_with_session(None, None, &mut session).await);
//     while let Some(result) = cursor.next(&mut session).await {
//       let file = tx_try!(result);
//       info!("migrating file {} => {} for station {}", count, file.id, file.station_id);
//       let order = tx_try!(IncrementStationAudioFileOrder::next_with_session(&file.station_id, &mut session).await);
//       let filter = doc!{ db::KEY_ID: &file.id };
//       let update = doc!{ "$set": { AudioFile::KEY_ORDER: order } };
//       let r = tx_try!(cl.update_one_with_session(filter, update, None, &mut session).await);
//       assert_eq!(r.matched_count, 1);
//       assert_eq!(r.modified_count, 1);
//       count += 1;
//     };

//     count
//   });

//   info!("{} files modified", count);
//   info!("Bye!");

//   Ok(())
// }

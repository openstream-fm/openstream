use anyhow::Context;
use db::{
  http::SocketAddr,
  station::Station,
  stream_connection::{lite::StreamConnectionLite, StreamConnection},
  Model,
};
use log::*;

fn random_os() -> Option<String> {
  use rand::seq::SliceRandom; // 0.7.2

  let vs = vec![
    Some("Linux"),
    Some("Android"),
    Some("iPhone"),
    Some("iPad"),
    Some("Windows"),
    Some("Mac OSX"),
    None,
  ];
  let option = vs.choose(&mut rand::thread_rng()).unwrap();
  option.map(String::from)
}

fn random_browser() -> Option<String> {
  use rand::seq::SliceRandom; // 0.7.2

  let vs = vec![
    Some("Chrome"),
    Some("Safari"),
    Some("Edge"),
    Some("Firefox"),
    None,
  ];
  let option = vs.choose(&mut rand::thread_rng()).unwrap();
  option.map(String::from)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  let c: usize = std::env::var("C")
    .context("env C is required")?
    .parse()
    .context("env C must be a usize")?;
  let station_id = std::env::var("S").context("env S is required")?;
  create_random_stream_connections(c, station_id).await
}

async fn create_random_stream_connections(
  c: usize,
  station_id: String,
) -> Result<(), anyhow::Error> {
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

  db::init(client, config.mongodb.storage_db_name);

  info!("ensuring mongodb collections...");
  db::ensure_collections()
    .await
    .context("error ensuring mongodb collections and indexes")?;

  let _ = match Station::get_by_id(&station_id).await? {
    None => anyhow::bail!("cannot find station with id {station_id}"),
    Some(station) => station,
  };

  for i in 0..c {
    create_random_stream_connection(c, &station_id, i).await?
  }

  info!("created {c} stream connections");

  Ok(())
}

async fn create_random_stream_connection(
  c: usize,
  station_id: &str,
  i: usize,
) -> Result<(), anyhow::Error> {
  if i % 10_000 == 0 {
    info!("creating stream connection {i} of {c}");
  }

  let ip = std::net::IpAddr::from([
    rand::random(),
    rand::random(),
    rand::random(),
    rand::random(),
  ]);

  let request = db::http::Request {
    country_code: geoip::ip_to_country_code(&ip),
    real_ip: ip,
    local_addr: SocketAddr {
      ip: std::net::IpAddr::from([0, 0, 0, 0]),
      port: 1,
    },
    remote_addr: SocketAddr {
      ip: std::net::IpAddr::from([0, 0, 0, 0]),
      port: 1,
    },
    uri: db::http::Uri {
      uri: "".into(),
      scheme: None,
      host: None,
      port: None,
      path: "".into(),
      query: None,
    },
    user_agent: user_agent::UserAgent {
      ua: None,
      category: None,
      browser_type: None,
      vendor: None,
      name: random_browser(),
      version: None,
      os: random_os(),
      os_version: None,
    },
    version: db::http::Version::HTTP_10,
    method: db::http::Method::GET,
    headers: db::http::Headers::new(),
  };

  let created_at: time::OffsetDateTime =
    time::OffsetDateTime::now_utc() - (time::Duration::DAY * 30 * rand::random::<f64>());

  let is_open = rand::random::<f64>() < (1_f64 / 30_f64);

  let duration_ms = if is_open {
    let mul: f64 = rand::random();
    Some((mul * 6000.0) as u64) // 10 min
  } else {
    None
  };

  let transfer_bytes = duration_ms.map(|s| s * 16); // 16 kbps

  let document = StreamConnection {
    id: StreamConnection::uid(),
    station_id: station_id.to_string(),
    deployment_id: String::from(""),
    is_open,
    ip: request.real_ip,
    country_code: request.country_code,
    transfer_bytes,
    duration_ms,
    last_transfer_at: created_at.into(),
    created_at: created_at.into(),
    request,
  };

  let document_lite = StreamConnectionLite::from_stream_connection_ref(&document);

  StreamConnection::insert(document).await?;
  StreamConnectionLite::insert(document_lite).await?;

  Ok(())
}

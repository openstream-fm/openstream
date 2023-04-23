use std::collections::BTreeMap;

use geoip::CountryCode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/stream-connection-stats/")]
pub struct Stats {
  pub now: StatsItem,
  pub last_24h: StatsItem,
  pub last_7d: StatsItem,
  pub last_30d: StatsItem,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/stream-connection-stats/")]
pub struct StatsItem {
  pub sessions: f64,
  #[ts(type = "Record<string, number | undefined>")]
  pub country_sessions: BTreeMap<CountryCode, f64>,
  // pub ips: f64,
  // #[ts(type = "Record<string, number | undefined>")]
  // pub country_ips: BTreeMap<CountryCode, f64>,
}

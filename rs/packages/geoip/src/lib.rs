use serde::{Deserialize, Deserializer, Serialize};
use std::net::IpAddr;
use std::str::FromStr;
use strum::*;
use ts_rs::TS;

pub fn ip_to_country_code(addr: &IpAddr) -> Option<CountryCode> {
  let entry = ip2geo::search(addr)?;
  match entry.country.trim() {
    "" => None,
    s => match CountryCode::from_str(s) {
      Ok(v) => Some(v),
      Err(_) => {
        log::warn!("failed to convert from string to CountryCode: {s}");
        None
      }
    },
  }
}

#[test]
fn it_works() {
  let addr = IpAddr::from([133u8, 132u8, 135u8, 169u8]);
  let code = ip_to_country_code(&addr).unwrap();
  eprintln!("code: {code}");
}

#[derive(
  Debug,
  Clone,
  Copy,
  Eq,
  Ord,
  PartialOrd,
  Serialize,
  Deserialize,
  TS,
  strum::Display,
  AsRefStr,
  EnumCount,
  EnumIter,
  FromRepr,
  EnumString,
  IntoStaticStr,
  deepsize::DeepSizeOf,
)]
#[ts(export, export_to = "../../../defs/")]
#[macros::keys]
// #[repr(u8)]
pub enum CountryCode {
  KM,
  NR,
  AD,
  AE,
  AF,
  AG,
  AI,
  AL,
  AM,
  AO,
  AR,
  AS,
  AT,
  AU,
  AW,
  AX,
  AZ,
  BA,
  BB,
  BD,
  BE,
  BF,
  BG,
  BH,
  BI,
  BJ,
  BL,
  BM,
  BN,
  BO,
  BQ,
  BR,
  BS,
  BT,
  BW,
  BY,
  BZ,
  CA,
  CD,
  CF,
  CG,
  CH,
  CI,
  CK,
  CL,
  CM,
  CN,
  CO,
  CR,
  CU,
  CV,
  CW,
  CY,
  CZ,
  DE,
  DJ,
  DK,
  DM,
  DO,
  DZ,
  EC,
  EE,
  EG,
  EH,
  ER,
  ES,
  ET,
  EU,
  FI,
  FJ,
  FK,
  FM,
  FO,
  FR,
  GA,
  GB,
  GD,
  GE,
  GF,
  GG,
  GH,
  GI,
  GL,
  GM,
  GN,
  GP,
  GQ,
  GR,
  GT,
  GU,
  GW,
  GY,
  HK,
  HN,
  HR,
  HT,
  HU,
  ID,
  IE,
  IL,
  IM,
  IN,
  IO,
  IQ,
  IR,
  IS,
  IT,
  JE,
  JM,
  JO,
  JP,
  KE,
  KG,
  KH,
  KI,
  KN,
  KP,
  KR,
  KW,
  KY,
  KZ,
  LA,
  LB,
  LC,
  LI,
  LK,
  LR,
  LS,
  LT,
  LU,
  LV,
  LY,
  MA,
  MC,
  MD,
  ME,
  MF,
  MG,
  MH,
  MK,
  ML,
  MM,
  MN,
  MO,
  MP,
  MQ,
  MR,
  MS,
  MT,
  MU,
  MV,
  MW,
  MX,
  MY,
  MZ,
  NA,
  NC,
  NE,
  NF,
  NG,
  NI,
  NL,
  NO,
  NP,
  NU,
  NZ,
  OM,
  PA,
  PE,
  PF,
  PG,
  PH,
  PK,
  PL,
  PM,
  PR,
  PS,
  PT,
  PW,
  PY,
  QA,
  RE,
  RO,
  RS,
  RU,
  RW,
  SA,
  SB,
  SC,
  SD,
  SE,
  SG,
  SI,
  SK,
  SL,
  SM,
  SN,
  SO,
  SR,
  SS,
  ST,
  SV,
  SX,
  SY,
  SZ,
  TC,
  TD,
  TG,
  TH,
  TJ,
  TL,
  TM,
  TN,
  TO,
  TR,
  TT,
  TV,
  TW,
  TZ,
  UA,
  UG,
  US,
  UY,
  UZ,
  VA,
  VC,
  VE,
  VG,
  VI,
  VN,
  VU,
  WF,
  WS,
  YE,
  YT,
  ZA,
  ZM,
  ZW,
  ZZ,
}

impl std::hash::Hash for CountryCode {
  fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
    hasher.write_u8(*self as u8)
  }
}

impl std::cmp::PartialEq for CountryCode {
  fn eq(&self, other: &Self) -> bool {
    *self as u8 == *other as u8
  }
}

pub fn deserialize_option<'de, D: Deserializer<'de>>(
  de: D,
) -> Result<Option<CountryCode>, D::Error> {
  let helper: Option<&str> = Deserialize::deserialize(de)?;
  match helper {
    None => Ok(None),
    Some(v) => match CountryCode::from_str(v) {
      Ok(v) => Ok(Some(v)),
      Err(_) => Ok(None),
    },
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn size_of_eq_1() {
    assert_eq!(std::mem::size_of::<CountryCode>(), 1);
  }

  #[test]
  pub fn size_of_option_eq_1() {
    assert_eq!(std::mem::size_of::<Option<CountryCode>>(), 1);
  }
}

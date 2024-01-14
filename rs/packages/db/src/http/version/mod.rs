use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[allow(non_camel_case_types)]
#[derive(
  Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize, TS, JsonSchema,
)]
#[ts(export, export_to = "../../../defs/db/http/")]
pub enum Version {
  #[serde(rename = "0.9")]
  HTTP_09,
  #[serde(rename = "1.0")]
  HTTP_10,
  #[serde(rename = "1.1")]
  HTTP_11,
  #[serde(rename = "2")]
  HTTP_2,
  #[serde(rename = "3")]
  HTTP_3,
  #[serde(rename = "other")]
  Other,
}

impl Version {
  #[inline]
  pub fn from_http(v: hyper::Version) -> Version {
    match v {
      hyper::Version::HTTP_09 => Version::HTTP_09,
      hyper::Version::HTTP_10 => Version::HTTP_10,
      hyper::Version::HTTP_11 => Version::HTTP_11,
      hyper::Version::HTTP_2 => Version::HTTP_2,
      hyper::Version::HTTP_3 => Version::HTTP_3,
      _ => Version::Other,
    }
  }

  #[inline]
  pub fn to_http(v: Version) -> hyper::Version {
    match v {
      Version::HTTP_09 => hyper::Version::HTTP_09,
      Version::HTTP_10 => hyper::Version::HTTP_10,
      Version::HTTP_11 => hyper::Version::HTTP_11,
      Version::HTTP_2 => hyper::Version::HTTP_2,
      Version::HTTP_3 => hyper::Version::HTTP_3,
      Version::Other => hyper::Version::default(),
    }
  }
}

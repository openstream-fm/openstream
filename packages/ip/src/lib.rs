use std::fmt::Display;
use std::net::{AddrParseError, IpAddr, Ipv4Addr};
use std::str::FromStr;

use reqwest::{header::ToStrError, Client};

#[derive(Debug)]
pub enum Error {
  ResponseNotOk,
  Reqwuest(reqwest::Error),
  NoIpHeader,
  ToStr(ToStrError),
  AddrParse(AddrParseError),
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ResponseNotOk => write!(f, "response status is not ok"),
      Self::Reqwuest(e) => write!(f, "error trying to connect to ip server: {}", e),
      Self::NoIpHeader => write!(f, "no x-ip header in response"),
      Self::ToStr(e) => write!(f, "x-ip header is not utf-8: {}", e),
      Self::AddrParse(e) => write!(f, "x-ip header is not a valid ip: {}", e),
    }
  }
}

impl From<reqwest::Error> for Error {
  fn from(e: reqwest::Error) -> Self {
    Self::Reqwuest(e)
  }
}

impl From<AddrParseError> for Error {
  fn from(e: AddrParseError) -> Self {
    Self::AddrParse(e)
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::Reqwuest(e) => Some(e),
      Self::ToStr(e) => Some(e),
      Self::AddrParse(e) => Some(e),
      _ => None,
    }
  }
}

pub async fn get_ip_v4() -> Result<Ipv4Addr, Error> {
  let client = Client::builder()
    .local_address(IpAddr::from([0, 0, 0, 0]))
    .build()
    .unwrap();

  let res = client.get("http://ip.openapps.ar/").send().await?;

  if !res.status().is_success() {
    return Err(Error::ResponseNotOk);
  }

  match res.headers().get("x-ip") {
    None => Err(Error::NoIpHeader),
    Some(v) => match v.to_str() {
      Err(e) => Err(Error::ToStr(e)),
      Ok(s) => {
        let ip = Ipv4Addr::from_str(s)?;
        Ok(ip)
      }
    },
  }
}

pub async fn get_ip_v4_ssl() -> Result<Ipv4Addr, Error> {
  let client = Client::builder()
    .local_address(IpAddr::from([0, 0, 0, 0]))
    .build()
    .unwrap();

  let res = client.get("https://ip.openapps.ar/").send().await?;

  if !res.status().is_success() {
    return Err(Error::ResponseNotOk);
  }

  match res.headers().get("x-ip") {
    None => Err(Error::NoIpHeader),
    Some(v) => match v.to_str() {
      Err(e) => Err(Error::ToStr(e)),
      Ok(s) => {
        let ip = Ipv4Addr::from_str(s)?;
        Ok(ip)
      }
    },
  }
}

use std::{fmt::Display, str::FromStr};

use lazy_regex::{lazy_regex, Lazy, Regex};
use serde::{de::Error, Deserialize, Deserializer, Serialize};

static MAIL_REGEX: Lazy<Regex> = lazy_regex!(
  r"^[a-z0-9]([a-z0-9\-_\.]+)?@[a-z0-9][a-z0-9_\-\.]+[a-z0-9]\.[a-z0-9\-_\.]{2,}[a-z0-9]$"
);

pub fn is_valid_email(address: &str) -> bool {
  MAIL_REGEX.is_match(address)
}

#[derive(Debug, Clone)]
pub enum ParseError {
  InvlidAddress,
}

impl Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvlidAddress => {
        write!(f, "Email address is invalid")
      }
    }
  }
}

impl std::error::Error for ParseError {}

/// Email type to use in database document structs, it does not check validity on deserialize
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DatabaseEmail(String);

impl DatabaseEmail {
  pub fn new(address: impl AsRef<str>) -> Result<Self, ParseError> {
    let addr = address.as_ref().trim().to_lowercase();
    let is_match = MAIL_REGEX.is_match(&addr);
    if !is_match {
      Err(ParseError::InvlidAddress)
    } else {
      Ok(Self(addr))
    }
  }

  pub fn new_unchecked(address: impl ToString) -> Self {
    Self(address.to_string())
  }
}

impl AsRef<str> for DatabaseEmail {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl FromStr for DatabaseEmail {
  type Err = ParseError;
  fn from_str(address: &str) -> Result<Self, Self::Err> {
    Self::new(address)
  }
}

/// Email type to use in database document structs, it does not check validity on deserialize
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct InputEmail(String);

impl InputEmail {
  pub fn new(address: impl AsRef<str>) -> Result<Self, ParseError> {
    let addr = address.as_ref().trim().to_lowercase();
    let is_match = MAIL_REGEX.is_match(&addr);
    if !is_match {
      Err(ParseError::InvlidAddress)
    } else {
      Ok(Self(addr))
    }
  }

  pub fn new_unchecked(address: impl ToString) -> Self {
    Self(address.to_string())
  }
}

impl<'de> Deserialize<'de> for InputEmail {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let address: &str = Deserialize::deserialize(d)?;
    Self::new(address).map_err(|_e| D::Error::custom("Email address is invalid"))
  }
}

impl AsRef<str> for InputEmail {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl FromStr for InputEmail {
  type Err = ParseError;
  fn from_str(address: &str) -> Result<Self, Self::Err> {
    Self::new(address)
  }
}

impl From<InputEmail> for DatabaseEmail {
  fn from(value: InputEmail) -> Self {
    DatabaseEmail(value.0)
  }
}

impl From<DatabaseEmail> for InputEmail {
  fn from(value: DatabaseEmail) -> Self {
    InputEmail(value.0)
  }
}

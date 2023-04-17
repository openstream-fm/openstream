use hyper::http::method::InvalidMethod;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Method {
  HEAD,
  GET,
  PUT,
  PATCH,
  POST,
  DELETE,
  OPTIONS,
  CONNECT,
  SOURCE,
  Other(String),
}

impl Method {
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(m: &str) -> Self {
    match m {
      "HEAD" => Self::HEAD,
      "GET" => Self::GET,
      "DELETE" => Self::DELETE,
      "PUT" => Self::PUT,
      "PATCH" => Self::PATCH,
      "POST" => Self::POST,
      "OPTIONS" => Self::OPTIONS,
      "CONNECT" => Self::CONNECT,
      "SOURCE" => Self::SOURCE,
      s => Self::Other(s.to_string()),
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      Self::HEAD => "HEAD",
      Self::GET => "GET",
      Self::DELETE => "DELETE",
      Self::PUT => "PUT",
      Self::PATCH => "PATCH",
      Self::POST => "POST",
      Self::OPTIONS => "OPTIONS",
      Self::CONNECT => "CONNECT",
      Self::SOURCE => "SOURCE",
      Self::Other(v) => v.as_str(),
    }
  }

  pub fn from_http(m: &hyper::Method) -> Self {
    Self::from_str(m.as_str())
  }

  pub fn to_http(&self) -> Result<hyper::Method, InvalidMethod> {
    hyper::Method::from_bytes(self.as_str().as_bytes())
  }
}

impl AsRef<str> for Method {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl ts_rs::TS for Method {
  const EXPORT_TO: Option<&'static str> = Some("../../../defs/db/http/Method.ts");

  /// Declaration of this type, e.g. `interface User { user_id: number, ... }`.
  /// This function will panic if the type has no declaration.
  fn decl() -> String {
    format!("type {} = {};", Self::name(), Self::inline())
  }

  /// Information about types this type depends on.
  /// This is used for resolving imports when exporting to a file.
  fn dependencies() -> Vec<ts_rs::Dependency> {
    vec![]
  }

  /// Name of this type in TypeScript.
  fn name() -> String {
    "Method".into()
  }

  /// Formats this types definition in TypeScript, e.g `{ user_id: number }`.
  /// This function will panic if the type cannot be inlined.
  fn inline() -> String {
    r#""HEAD" | "GET" | "PUT" | "PATCH" | "POST" | "DELETE" | "OPTIONS" | "CONNECT" | "SOURCE" | string"#.into()
  }

  /// Flatten an type declaration.  
  /// This function will panic if the type cannot be flattened.
  // fn inline_flattened() -> String {
  //   panic!("{} cannot be flattened", Self::name())
  // }

  /// `true` if this is a transparent type, e.g tuples or a list.  
  /// This is used for resolving imports when using the `export!` macro.
  fn transparent() -> bool {
    false
  }
}

impl Serialize for Method {
  fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    self.as_str().serialize(s)
  }
}

impl<'de> Deserialize<'de> for Method {
  fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let m = String::deserialize(d)?;
    Ok(Self::from_str(&m))
  }
}

#[cfg(test)]
mod test {
  use ts_rs::TS;

  #[test]
  fn export_method_definitions() {
    super::Method::export().unwrap();
  }
}

use prex::Request;
use serde::{Deserialize, Serialize};
use static_init::dynamic;
use woothee::parser::Parser;

#[dynamic]
static PARSER: Parser = Parser::default();

/// UserAgent is an owned value
/// it does allocate in favor of simplicity for users
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserAgent {
  pub ua: Option<String>,
  pub category: Option<String>,
  pub browser_type: Option<String>,
  pub vendor: Option<String>,
  pub name: Option<String>,
  pub version: Option<String>,
  pub os: Option<String>,
  pub os_version: Option<String>,
}

fn from_str(v: &str) -> Option<String> {
  if v == woothee::woothee::VALUE_UNKNOWN {
    None
  } else {
    Some(String::from(v))
  }
}

impl UserAgent {
  pub fn parse(agent: &str) -> Self {
    let helper = match PARSER.parse(agent) {
      None => return UserAgent::default(),
      Some(helper) => helper,
    };

    Self {
      ua: Some(String::from(agent)),
      browser_type: from_str(helper.browser_type),
      category: from_str(helper.category),
      name: from_str(helper.name),
      os: from_str(helper.os),
      os_version: from_str(&helper.os_version),
      vendor: from_str(&helper.os_version),
      version: from_str(&helper.os_version),
    }
  }
}

pub trait UserAgentExt {
  fn parse_ua(&self) -> UserAgent;
}

impl UserAgentExt for Request {
  fn parse_ua(&self) -> UserAgent {
    let v = match self.headers().get("user-agent") {
      None => return Default::default(),
      Some(v) => v,
    };

    let v = match v.to_str() {
      Err(_) => Default::default(),
      Ok(v) => v,
    };

    UserAgent::parse(v)
  }
}

#[cfg(test)]
#[test]
pub fn parse_user_agent() {
  let ch = UserAgent::parse("Mozilla/5.0 (X11; Fedora; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36");
  let ff =
    UserAgent::parse("Mozilla/5.0 (X11; Linux x86_64; rv:106.0) Gecko/20100101 Firefox/106.0");
  let ch_mac = UserAgent::parse("Mozilla/5.0 (Macintosh; Intel Mac OS X 13_0_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36");
  let edge = UserAgent::parse("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36 Edg/107.0.1418.56");
  let android = UserAgent::parse("Mozilla/5.0 (Linux; Android 13) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.5304.105 Mobile Safari/537.36");

  eprintln!("{ch:?}");
  eprintln!("{ff:?}");
  eprintln!("{ch_mac:?}");
  eprintln!("{edge:?}");
  eprintln!("{android:?}");
}

/*
macro_rules! str_enum {
  ($type:ident, $($name:ident => $value:literal)* ) => {

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum $type {
      $($name,)*
      Other(String)
    }

    impl AsRef<str> for $type {
      fn as_ref(&self) -> &str {
        match self {
          $(Self::$name => $value,)*
          Self::Other(v) => v.as_ref()
        }
      }
    }

    impl From<&str> for $type {
      fn from(v: &str) -> Self {
        $(
          if v.eq_ignore_ascii_case($value) {
            return Self::$name;
          }
        )*

        Self::Other(String::from(v))
      }
    }

    impl Serialize for $type {
      fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(s)
      }
    }

    impl<'de> Deserialize<'de> for $type {
      fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v: &str = Deserialize::deserialize(d)?;
        Ok(Self::from(v))
      }
    }
  };
}

str_enum!(Name,
  Chrome => "Chrome"
  Chromium => "Chromium"
  Edge => "Edge"
  Firefox => "Firefox"
  Safari => "Safari"
);

str_enum!(Category,
  Pc => "pc"
  Mobile => "mobile"
  Tablet => "tablet"
);

str_enum!(Vendor,
  Google => "Google"
  Mozilla => "Mozilla"
  Microsoft => "Microsoft"
  Apple => "Apple"
);

str_enum!(Os,
  Android => "Android"
  Linux => "Linux"
  Windows => "Windows"
  Osx => "Mac OSX"
  Ios => "iOS"
  Chrome => "Chrome"
  Chromium => "Chromium"
);

str_enum!(BrowserType,
  Browser => "browser"
);

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct Version(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct OsVersion(pub String);
*/

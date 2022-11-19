#![allow(clippy::manual_flatten)]
use hyper::Method;
use regex::Regex;

use crate::error::RouterBuilderError;
use crate::params::Params;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum MatchType {
  Exact,
  Scope,
}

#[derive(Clone)]
pub struct Matcher {
  pub method: Option<Method>,
  pub pattern: Option<Regex>,
  pub match_type: MatchType,
}

impl Matcher {
  pub fn all() -> Self {
    Self {
      method: None,
      pattern: None,
      match_type: MatchType::Scope,
    }
  }

  fn match_method(&self, request_method: &Method) -> bool {
    match &self.method {
      None => true,
      Some(method) => {
        method == request_method || (method == Method::GET && request_method == Method::HEAD)
      }
    }
  }

  fn match_pattern(&self, path: &str) -> Option<Params> {
    if self.pattern.is_none() {
      return Some(Params::default());
    }

    let pattern = self.pattern.as_ref().unwrap();
    if let Some(caps) = pattern.captures(path) {
      let names = pattern.capture_names();
      let mut params = Params::with_capacity(names.len());
      for name in names {
        if let Some(name) = name {
          if let Some(value) = caps.name(name) {
            params.set(name, value.as_str())
          }
        }
      }

      return Some(params);
    }

    None
  }

  fn is_param_name_char(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9')
  }

  pub fn compile_pattern_str(pat: &str) -> String {
    let path = format!("/{}", pat.trim_matches('/'));

    let mut compiled = String::new();

    let chars = path.chars().collect::<Vec<char>>();
    let mut index = 0;

    'root: loop {
      if index >= chars.len() {
        break 'root;
      }

      let ch = chars[index];
      index += 1;

      if ch != ':' {
        let mut literal = String::new();
        literal.push(ch);

        'literal: loop {
          if index >= chars.len() {
            break 'literal;
          }

          let ch = chars[index];

          if ch == ':' {
            break 'literal;
          } else {
            index += 1;
            literal.push(ch);
          }
        }

        compiled.push_str(regex::escape(literal.as_str()).as_str());
      } else {
        // start named param
        let mut name = String::new();
        let mut pattern = String::new();
        'name: for ch in chars.iter().skip(index).cloned() {
          if Self::is_param_name_char(ch) {
            index += 1;
            name.push(ch);
          } else {
            break 'name;
          }
        }

        // is not named param
        if name.is_empty() {
          compiled.push_str(regex::escape(":").as_str());
          continue 'root;
        }

        if chars.get(index) == Some(&'(') {
          index += 1;
          let mut open_paren_count = 1;
          let mut escape_count = 0;
          'pattern: for ch in chars.iter().skip(index).cloned() {
            index += 1;

            match ch {
              '(' => {
                if escape_count % 2 == 0 {
                  open_paren_count += 1;
                }
                pattern.push(ch);
              }

              ')' => {
                if escape_count % 2 == 0 {
                  open_paren_count -= 1;
                }

                if open_paren_count == 0 {
                  break 'pattern;
                } else {
                  pattern.push(ch);
                }
              }

              ch => {
                pattern.push(ch);
              }
            }

            if ch == '\\' {
              escape_count += 1;
            } else {
              escape_count = 0;
            }
          }
        }

        if pattern.is_empty() {
          pattern.push_str("[^/]+");
        }

        compiled.push_str(format!("(?P<{}>{})", name, pattern).as_str())
      }
    }

    compiled
  }

  pub fn compile_pattern(pat: &str, match_type: MatchType) -> Result<Regex, RouterBuilderError> {
    let compiled = Self::compile_pattern_str(pat);

    let re = match match_type {
      MatchType::Exact => {
        if compiled == "/" || compiled.is_empty() {
          Regex::new("^/?$")
        } else {
          Regex::new(format!("^{}/?$", compiled.trim_end_matches('/')).as_str())
        }
      }
      MatchType::Scope => {
        if compiled == "/" || compiled.is_empty() {
          Regex::new("^/?")
        } else {
          Regex::new(format!("^{}(?:$|/)", compiled.trim_end_matches('/')).as_str())
        }
      }
    };

    match re {
      Ok(re) => Ok(re),
      Err(e) => Err(RouterBuilderError::RouteRegexError {
        path: pat.to_string(),
        description: e.to_string(),
      }),
    }
  }

  pub fn r#match<'a>(&self, request_method: &Method, path: &'a str) -> Option<Params> {
    if !self.match_method(request_method) {
      return None;
    }

    self.match_pattern(path)
  }
}

#[cfg(test)]
mod tests {
  use super::Matcher;

  #[test]
  fn test_pattern_compiler() {
    let urls = vec![
      ("/", "/"),
      ("/:p1", "/(?P<p1>[^/]+)"),
      ("/:p1((.+))", "/(?P<p1>(.+))"),
      (r"/:p1(\\(.+))", r"/(?P<p1>\\(.+))"),
    ];

    for (left, right) in urls.iter() {
      assert_eq!(Matcher::compile_pattern_str(left), right.to_string());
    }
  }
}

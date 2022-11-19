use regex::Regex;
use std::time::Duration;

use hyper::header::HeaderValue;
use hyper::header::{
  ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
  ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS, ACCESS_CONTROL_MAX_AGE, ALLOW,
  ORIGIN, VARY,
};
use hyper::Method;
use hyper::StatusCode;

use crate::handler::Handler;
use crate::Next;
use crate::Request;
use crate::Response;

use async_trait::async_trait;

pub struct Cors {
  allow_origin: AllowOrigin,
  allow_methods: Option<HeaderValue>,
  allow_headers: Option<HeaderValue>,
  expose_headers: Option<HeaderValue>,
  allow_credentials: HeaderValue,
  max_age: HeaderValue,
}

fn has_vary(header: &str, value: &str) -> bool {
  let value = value.trim().to_lowercase();
  for item in header.split(',') {
    if item.trim().to_lowercase() == value {
      return true;
    }
  }
  false
}

fn add_vary(res: &mut Response, value: &'static str) {
  let vary = res.headers().get(VARY).cloned();

  if vary.is_none() {
    res
      .headers_mut()
      .insert(VARY, HeaderValue::from_static(value));
    return;
  }

  let vary = vary.unwrap();
  let vary = vary.to_str();

  if vary.is_err() {
    return;
  }

  let vary = vary.unwrap();

  if has_vary(vary, value) {
    return;
  }

  if let Ok(v) = HeaderValue::from_str(&format!("{},{}", vary, value)) {
    res.headers_mut().insert(VARY, v);
  }
}

#[async_trait]
impl Handler for Cors {
  async fn call(&self, req: Request, next: Next) -> Response {
    if req.method() == Method::OPTIONS {
      let mut res = Response::new(StatusCode::OK);
      match &self.allow_origin {
        AllowOrigin::Fixed(v) => {
          res
            .headers_mut()
            .insert(ACCESS_CONTROL_ALLOW_ORIGIN, v.clone());
        }
        AllowOrigin::Variable(v) => {
          add_vary(&mut res, "origin");
          if let Some(origin_header) = req.headers().get(ORIGIN) {
            if let Ok(origin) = origin_header.to_str() {
              if v.allow(origin) {
                res
                  .headers_mut()
                  .insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin_header.clone());
              }
            }
          }
        }
      }

      if let Some(v) = self.allow_methods.as_ref().cloned() {
        res.headers_mut().insert(ALLOW, v.clone());
        res.headers_mut().insert(ACCESS_CONTROL_ALLOW_METHODS, v);
      }

      if let Some(v) = self.allow_headers.as_ref().cloned() {
        res.headers_mut().insert(ACCESS_CONTROL_ALLOW_HEADERS, v);
      }

      if let Some(v) = self.expose_headers.as_ref().cloned() {
        res.headers_mut().insert(ACCESS_CONTROL_EXPOSE_HEADERS, v);
      }

      res.headers_mut().insert(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        self.allow_credentials.clone(),
      );

      res
        .headers_mut()
        .insert(ACCESS_CONTROL_MAX_AGE, self.max_age.clone());

      return res;
    }

    let origin = req.headers().get(ORIGIN).cloned();

    let mut res = next.run(req).await;

    match &self.allow_origin {
      AllowOrigin::Fixed(v) => {
        res
          .headers_mut()
          .insert(ACCESS_CONTROL_ALLOW_ORIGIN, v.clone());
      }
      AllowOrigin::Variable(v) => {
        add_vary(&mut res, "origin");
        if let Some(origin_header) = origin {
          if let Ok(origin_str) = origin_header.to_str() {
            if v.allow(origin_str) {
              res
                .headers_mut()
                .insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin_header.clone());
            }
          }
        }
      }
    };

    res
  }
}

pub fn cors() -> Cors {
  Cors {
    allow_origin: "*".into_allow_origin(),
    allow_methods: "GET,HEAD,POST,PUT,PATCH,DELETE".into_header_value_option(),
    allow_headers: None,
    expose_headers: None,
    allow_credentials: HeaderValue::from_static("false"),
    max_age: HeaderValue::from_static("0"),
  }
}

impl Cors {
  pub fn allow_origin<T: IntoAllowOrigin>(mut self, t: T) -> Self {
    self.allow_origin = t.into_allow_origin();
    self
  }

  pub fn allow_methods<T: IntoHeaderValueOption>(mut self, t: T) -> Self {
    self.allow_methods = t.into_header_value_option();
    self
  }

  pub fn allow_headers<T: IntoHeaderValueOption>(mut self, t: T) -> Self {
    self.allow_headers = t.into_header_value_option();
    self
  }

  pub fn expose_headers<T: IntoHeaderValueOption>(mut self, t: T) -> Self {
    self.expose_headers = t.into_header_value_option();
    self
  }

  pub fn allow_credentials(mut self, v: bool) -> Self {
    self.allow_credentials = match v {
      true => HeaderValue::from_static("true"),
      false => HeaderValue::from_static("false"),
    };
    self
  }

  pub fn max_age<T: IntoMaxAgeHeader>(mut self, t: T) -> Self {
    self.max_age = t.into_max_age_header();
    self
  }
}

pub trait IntoHeaderValueOption {
  fn into_header_value_option(self) -> Option<HeaderValue>;
}

impl IntoHeaderValueOption for () {
  fn into_header_value_option(self) -> Option<HeaderValue> {
    None
  }
}

impl IntoHeaderValueOption for HeaderValue {
  fn into_header_value_option(self) -> Option<HeaderValue> {
    Some(self)
  }
}

impl IntoHeaderValueOption for String {
  fn into_header_value_option(self) -> Option<HeaderValue> {
    Some(HeaderValue::from_str(&self).unwrap())
  }
}

impl IntoHeaderValueOption for &'static str {
  fn into_header_value_option(self) -> Option<HeaderValue> {
    Some(HeaderValue::from_static(self))
  }
}

pub trait VariableAllowOrigin: Send + Sync + 'static {
  fn allow(&self, origin: &str) -> bool;
}

impl VariableAllowOrigin for Regex {
  fn allow(&self, origin: &str) -> bool {
    self.is_match(origin)
  }
}

impl VariableAllowOrigin for Vec<String> {
  fn allow(&self, origin: &str) -> bool {
    for item in self.iter() {
      if item == origin {
        return true;
      }
    }
    false
  }
}

impl VariableAllowOrigin for Vec<&'static str> {
  fn allow(&self, origin: &str) -> bool {
    for item in self.iter() {
      if item == &origin {
        return true;
      }
    }
    false
  }
}

impl<F> VariableAllowOrigin for F
where
  F: Send + Sync + 'static + Fn(&str) -> bool,
{
  fn allow(&self, origin: &str) -> bool {
    (self)(origin)
  }
}

pub enum AllowOrigin {
  Fixed(HeaderValue),
  Variable(Box<dyn VariableAllowOrigin>),
}

pub trait IntoAllowOrigin {
  fn into_allow_origin(self) -> AllowOrigin;
}

impl IntoAllowOrigin for String {
  fn into_allow_origin(self) -> AllowOrigin {
    AllowOrigin::Fixed(
      HeaderValue::from_str(&self).expect("Invalid header value in cors.allow_origin()"),
    )
  }
}

impl IntoAllowOrigin for &'static str {
  fn into_allow_origin(self) -> AllowOrigin {
    AllowOrigin::Fixed(HeaderValue::from_static(self))
  }
}

impl IntoAllowOrigin for HeaderValue {
  fn into_allow_origin(self) -> AllowOrigin {
    AllowOrigin::Fixed(self)
  }
}

impl IntoAllowOrigin for Vec<String> {
  fn into_allow_origin(self) -> AllowOrigin {
    AllowOrigin::Variable(Box::new(self))
  }
}

impl IntoAllowOrigin for Vec<&'static str> {
  fn into_allow_origin(self) -> AllowOrigin {
    AllowOrigin::Variable(Box::new(self))
  }
}

impl IntoAllowOrigin for Regex {
  fn into_allow_origin(self) -> AllowOrigin {
    AllowOrigin::Variable(Box::new(self))
  }
}

impl<F> IntoAllowOrigin for F
where
  F: Send + Sync + 'static + Fn(&str) -> bool,
{
  fn into_allow_origin(self) -> AllowOrigin {
    AllowOrigin::Variable(Box::new(self))
  }
}

pub trait IntoMaxAgeHeader {
  fn into_max_age_header(self) -> HeaderValue;
}

impl IntoMaxAgeHeader for usize {
  fn into_max_age_header(self) -> HeaderValue {
    HeaderValue::from_str(&self.to_string()).unwrap()
  }
}

impl IntoMaxAgeHeader for Duration {
  fn into_max_age_header(self) -> HeaderValue {
    HeaderValue::from_str(&self.as_secs().to_string()).unwrap()
  }
}

impl IntoMaxAgeHeader for String {
  fn into_max_age_header(self) -> HeaderValue {
    HeaderValue::from_str(&self).unwrap()
  }
}

impl IntoMaxAgeHeader for &'static str {
  fn into_max_age_header(self) -> HeaderValue {
    HeaderValue::from_static(self)
  }
}

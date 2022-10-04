use crate::handler::Handler;
use crate::request::Request;
use crate::response::Response;
use crate::next::Next;

use std::time::Duration;
use hyper::header::HeaderValue;
use hyper::header::{
  ACCESS_CONTROL_ALLOW_ORIGIN,
  ACCESS_CONTROL_ALLOW_METHODS,
  ACCESS_CONTROL_ALLOW_CREDENTIALS,
  ACCESS_CONTROL_ALLOW_HEADERS,
  ACCESS_CONTROL_EXPOSE_HEADERS,
  ACCESS_CONTROL_MAX_AGE,
  ALLOW,
  ORIGIN,
  VARY,
};

use hyper::Method;
use hyper::StatusCode;

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
  
  let vary = res.headers().get(VARY).map(|v| v.clone());

  if vary.is_none() {
    res.headers_mut().insert(VARY, HeaderValue::from_static(value));
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


use async_trait::async_trait;

pub trait AllowOrigin: Send + Sync + 'static {
  fn allow_origin(&self, origin: &str) -> bool;
}

pub trait AllowMethods: Send + Sync + 'static {
  fn allow_methods(&self, req: &Request) -> Option<HeaderValue>;
}

pub trait AllowHeaders: Send + Sync + 'static {
  fn allow_headers(&self, req: &Request) -> Option<HeaderValue>;
}

pub trait AllowCredentials: Send + Sync + 'static {
  fn allow_credentials(&self, req: &Request) -> bool;
}

pub trait ExposeHeaders: Send + Sync + 'static {
  fn expose_headers(&self, req: &Request) -> Option<HeaderValue>;
}

pub trait MaxAge: Send + Sync + 'static {
  fn max_age(&self, req: &Request) -> HeaderValue;
}

pub struct Cors {
  allow_origin: Box<dyn AllowOrigin>,
  allow_methods: Box<dyn AllowMethods>,
  allow_headers: Box<dyn AllowHeaders>,
  allow_credentials: Box<dyn AllowCredentials>,
  expose_headers: Box<dyn ExposeHeaders>,  
  max_age: Box<dyn MaxAge>,
}

pub fn cors() -> Cors {
  Cors {
    allow_origin: Box::new(()),
    allow_methods: Box::new(()),
    allow_headers: Box::new(()),
    allow_credentials: Box::new(()),
    expose_headers: Box::new(()),
    max_age: Box::new(Duration::from_nanos(0)),
  }
}


macro_rules! setter {
  ($name:ident, $type:ident) => {
    pub fn $name<T: $type>(mut self, t: T) -> Self {
      self.$name = Box::new(t);
      self
    }
  }
}

impl Cors {
  setter!(allow_origin, AllowOrigin);
  setter!(allow_methods, AllowMethods);
  setter!(allow_headers, AllowHeaders);
  setter!(allow_credentials, AllowCredentials);
  setter!(expose_headers, ExposeHeaders);
  setter!(max_age, MaxAge);
}


#[async_trait]
impl Handler for Cors {
  
  async fn call(&self, req: Request, next: Next) -> Response {
    
    if req.method() == &Method::OPTIONS {
      let allow_origin = match req.headers().get(ORIGIN) {
        Some(origin_header) => match origin_header.to_str() {
          Ok(v) => match self.allow_origin.allow_origin(v) {
            true => origin_header.clone(),
            false => HeaderValue::from_static("")
          },
          Err(_) => HeaderValue::from_static(""),
        },
        None => HeaderValue::from_static("")
      };
      
      let allow_methods = self.allow_methods.allow_methods(&req);
      let allow_credentials = self.allow_credentials.allow_credentials(&req);
      let allow_headers = self.allow_headers.allow_headers(&req);
      let expose_headers = self.expose_headers.expose_headers(&req);
      let max_age = self.max_age.max_age(&req);
      
      let allow_credentials_header = HeaderValue::from_static(if allow_credentials { "true" } else { "false" });
      
      let mut res = Response::new(StatusCode::OK);
      
      res.headers_mut().insert(VARY, HeaderValue::from_static("origin"));
      res.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, allow_origin);
      
      if let Some(allow_methods) = allow_methods {
        res.headers_mut().insert(ACCESS_CONTROL_ALLOW_METHODS, allow_methods.clone());
        res.headers_mut().insert(ALLOW, allow_methods);
      };

      if let Some(allow_headers) = allow_headers {
        res.headers_mut().insert(ACCESS_CONTROL_ALLOW_HEADERS, allow_headers);
      }

      res.headers_mut().insert(ACCESS_CONTROL_ALLOW_CREDENTIALS, allow_credentials_header);
      
      if let Some(expose_headers) = expose_headers {
        res.headers_mut().insert(ACCESS_CONTROL_EXPOSE_HEADERS, expose_headers);
      }

      res.headers_mut().insert(ACCESS_CONTROL_MAX_AGE, max_age);
      
      return res
    }

    let allowed_origin = match req.headers().get(ORIGIN) {
      Some(origin) => match origin.to_str() {
        Ok(origin) => match self.allow_origin.allow_origin(origin) {
          true => Some(origin.to_string()),
          false => None
        }, 
        Err(_) => None
      },
      None => None
    };

    let mut res = next.run(req).await;

    if let Some(v) = allowed_origin {
      res.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_str(&v).unwrap());
    }

    add_vary(&mut res, "origin");

    res
  }
}



// max age
impl MaxAge for Duration {
  fn max_age(&self, _: &Request) -> HeaderValue {
    HeaderValue::from_str(format!("{}", self.as_secs()).as_str()).unwrap()
  }
} 

impl<F> MaxAge for F
where F: Send + Sync + 'static + Fn(&Request) -> Duration {
  fn max_age(&self, req: &Request) -> HeaderValue {
    (self)(req).max_age(req)
  }
}

macro_rules! impl_common {
  ($trait:ident, $fn:ident) => {
    
    impl $trait for HeaderValue {
      fn $fn(&self, _: &Request) -> Option<HeaderValue> {
        Some(self.clone())
      }
    }

    impl $trait for String {
      fn $fn(&self, _: &Request) -> Option<HeaderValue> {
        Some(HeaderValue::from_str(self.as_str()).unwrap())
      }
    }

    impl $trait for &'static str {
      fn $fn(&self, _: &Request) -> Option<HeaderValue> {
        Some(HeaderValue::from_static(*self))
      }
    }

    impl<F> $trait for F 
    where F: Send + Sync + 'static + Fn(&Request) -> HeaderValue {
      fn $fn(&self, req: &Request) -> Option<HeaderValue> {
        Some((self)(req))
      }
    } 

  }
}

macro_rules! impl_common_with_vec {
  
  ($trait:ident, $fn:ident) => {
    
    impl_common!($trait, $fn);

    impl $trait for Vec<String> {
      fn $fn(&self, _: &Request) -> Option<HeaderValue> {
        Some(HeaderValue::from_str(&self.join(",")).unwrap())
      }
    }

    impl $trait for Vec<&'static str> {
      fn $fn(&self, _: &Request) -> Option<HeaderValue> {
        Some(HeaderValue::from_str(self.join(",").as_str()).unwrap())
      }
    }
  }
}

// allow origin
impl AllowOrigin for () {
  fn allow_origin(&self, _: &str) -> bool {
    true
  }
}

impl AllowOrigin for &'static str {
  fn allow_origin(&self, origin: &str) -> bool {
    *self == origin
  }
}

impl AllowOrigin for String {
  fn allow_origin(&self, origin: &str) -> bool {
    self == origin
  }
}

impl<F> AllowOrigin for F
where F: Send + Sync + 'static + Fn(&str) -> bool {
  fn allow_origin(&self, origin: &str) -> bool {
    self(origin)
  }
}

use regex::Regex;
impl AllowOrigin for Regex {
  fn allow_origin(&self, origin: &str) -> bool {
    self.is_match(origin)
  }
}

// Allow headers
impl_common_with_vec!(AllowMethods, allow_methods);
impl AllowMethods for () {
  fn allow_methods(&self, _: &Request) -> Option<HeaderValue> {
    Some(HeaderValue::from_static("GET,HEAD,POST,PUT,PATCH,DELETE"))
  }
}

impl_common_with_vec!(AllowHeaders, allow_headers);
impl AllowHeaders for () {
  fn allow_headers(&self, _: &Request) -> Option<HeaderValue> {
    None
  }
}

// expose headers
impl_common_with_vec!(ExposeHeaders, expose_headers);
impl ExposeHeaders for () {
  fn expose_headers(&self, _: &Request) -> Option<HeaderValue> {
    None
  }
}

// allow credentials
impl AllowCredentials for () {
  fn allow_credentials(&self, _: &Request) -> bool {
    false
  }
}

impl AllowCredentials for bool {
  fn allow_credentials(&self, _: &Request) -> bool {
    *self
  }
}

impl<F> AllowCredentials for F 
where F: Send + Sync + 'static + Fn(&Request) -> bool {
  fn allow_credentials(&self, req: &Request) -> bool {
    (self)(req)
  }
} 
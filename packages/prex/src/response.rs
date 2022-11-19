use hyper;
use hyper::header;
use hyper::header::HeaderValue;
use hyper::Body;
use hyper::StatusCode;

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Parts {
  pub response: hyper::Response<Body>,
  pub content_type: Option<HeaderValue>,
  pub charset: Option<HeaderValue>,
}

#[derive(Debug)]
pub struct Response {
  pub(crate) response: hyper::Response<Body>,
  content_type: Option<HeaderValue>,
  charset: Option<HeaderValue>,
}

impl Response {
  #[inline]
  pub fn into_parts(self) -> Parts {
    Parts {
      response: self.response,
      content_type: self.content_type,
      charset: self.charset,
    }
  }

  #[inline]
  pub fn from_parts(parts: Parts) -> Self {
    Self {
      response: parts.response,
      content_type: parts.content_type,
      charset: parts.charset,
    }
  }

  pub fn new(status: impl Into<StatusCode>) -> Self {
    let mut response = hyper::Response::new(Body::empty());
    *response.status_mut() = status.into();

    Self {
      response,
      content_type: None,
      charset: None,
    }
  }

  pub fn redirect(status: impl Into<StatusCode>, location: impl Into<HeaderValue>) -> Self {
    let mut response = Self::new(status.into());
    response
      .headers_mut()
      .insert(header::LOCATION, location.into());
    response
  }

  pub(crate) fn default_not_found(message: impl ToString) -> Self {
    let mut response = Response::new(StatusCode::NOT_FOUND);
    response
      .set_content_type(HeaderValue::from_static("text/plain"))
      .set_charset(HeaderValue::from_static("utf-8"));

    *response.body_mut() = Body::from(message.to_string());

    response
  }

  /// consumes this response returning only the body
  #[inline]
  pub fn into_body(self) -> Body {
    self.response.into_body()
  }

  /// takes the body if this response replacing it with Body::empty
  #[inline]
  pub fn take_body(&mut self) -> Body {
    let mut body = Body::empty();
    std::mem::swap(self.body_mut(), &mut body);
    body
  }

  #[inline]
  pub fn content_type(&self) -> Option<&HeaderValue> {
    self.content_type.as_ref()
  }

  #[inline]
  pub fn content_type_mut(&mut self) -> &mut Option<HeaderValue> {
    &mut self.content_type
  }

  #[inline]
  pub fn set_content_type(&mut self, value: impl Into<HeaderValue>) -> &mut Self {
    *self.content_type_mut() = Some(value.into());
    self
  }

  #[inline]
  pub fn charset(&self) -> Option<&HeaderValue> {
    self.charset.as_ref()
  }

  #[inline]
  pub fn charset_mut(&mut self) -> &mut Option<HeaderValue> {
    &mut self.charset
  }

  #[inline]
  pub fn set_charset(&mut self, value: impl Into<HeaderValue>) -> &mut Self {
    *self.charset_mut() = Some(value.into());
    self
  }
}

impl From<StatusCode> for Response {
  fn from(s: StatusCode) -> Self {
    Self::new(s)
  }
}

impl From<Body> for Response {
  fn from(b: Body) -> Self {
    let mut res = Self::new(StatusCode::OK);
    *res.body_mut() = b;
    res
  }
}

impl From<&str> for Response {
  fn from(s: &str) -> Self {
    let mut res = Self::new(StatusCode::OK);
    *res.body_mut() = Body::from(s.to_owned());
    res
  }
}

impl<T: Into<Response>, E: Into<Response>> From<Result<T, E>> for Response {
  fn from(r: Result<T, E>) -> Self {
    match r {
      Ok(r) => r.into(),
      Err(e) => e.into(),
    }
  }
}

impl<S: Into<StatusCode>, B: Into<Body>> From<(S, B)> for Response {
  fn from(r: (S, B)) -> Self {
    let mut res = Self::new(r.0.into());
    *res.body_mut() = r.1.into();
    res
  }
}

impl From<String> for Response {
  fn from(b: String) -> Self {
    let mut res = Self::new(StatusCode::OK);
    *res.body_mut() = Body::from(b);
    res
  }
}

impl Deref for Response {
  type Target = hyper::Response<hyper::Body>;
  fn deref(&self) -> &Self::Target {
    &self.response
  }
}

impl DerefMut for Response {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.response
  }
}

impl From<Response> for hyper::Response<Body> {
  fn from(me: Response) -> Self {
    let content_type = me.content_type().cloned();
    let charset = me.charset().cloned();
    let mut response = me.response;
    match (content_type, charset) {
      (Some(content_type), Some(charset)) => {
        if let (Ok(content_type), Ok(charset)) = (content_type.to_str(), charset.to_str()) {
          response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str(format!("{};charset={}", content_type, charset).as_str())
              .unwrap(),
          );
        }
      }

      (Some(content_type), None) => {
        response
          .headers_mut()
          .insert(header::CONTENT_TYPE, content_type);
      }
      _ => {}
    }

    response
  }
}

use hyper;
use hyper::header;
use hyper::header::HeaderValue;
use hyper::header::CONTENT_TYPE;
use hyper::http::Extensions;
use hyper::Body;
use hyper::HeaderMap;
use hyper::StatusCode;
use hyper::Version;

#[derive(Debug)]
pub struct Parts {
  pub status: StatusCode,
  pub version: Version,
  pub headers: HeaderMap,
  pub extensions: Extensions,
  pub body: Body,
}

#[derive(Debug)]
pub struct Response {
  pub(crate) status: StatusCode,
  pub(crate) version: Version,
  pub(crate) headers: HeaderMap,
  pub(crate) extensions: Extensions,
  pub(crate) body: Body,
}

impl Response {
  #[inline]
  pub fn into_parts(self) -> Parts {
    Parts {
      status: self.status,
      version: self.version,
      headers: self.headers,
      extensions: self.extensions,
      body: self.body,
    }
  }

  #[inline]
  pub fn from_parts(parts: Parts) -> Self {
    Self {
      status: parts.status,
      version: parts.version,
      headers: parts.headers,
      extensions: parts.extensions,
      body: parts.body,
    }
  }

  #[inline]
  pub fn status(&self) -> StatusCode {
    self.status
  }

  #[inline]
  pub fn status_mut(&mut self) -> &mut StatusCode {
    &mut self.status
  }

  #[inline]
  pub fn headers(&self) -> &HeaderMap {
    &self.headers
  }

  #[inline]
  pub fn headers_mut(&mut self) -> &mut HeaderMap {
    &mut self.headers
  }

  #[inline]
  pub fn extensions(&self) -> &Extensions {
    &self.extensions
  }

  #[inline]
  pub fn extensions_mut(&mut self) -> &mut Extensions {
    &mut self.extensions
  }

  #[inline]
  pub fn version(&self) -> Version {
    self.version
  }

  #[inline]
  pub fn version_mut(&mut self) -> &mut Version {
    &mut self.version
  }

  #[inline]
  pub fn body(&self) -> &Body {
    &self.body
  }

  #[inline]
  pub fn body_mut(&mut self) -> &mut Body {
    &mut self.body
  }

  /// takes the body of this request replacing it with Body::empty
  #[inline]
  pub fn take_body(&mut self) -> Body {
    let mut body = Body::empty();
    std::mem::swap(self.body_mut(), &mut body);
    body
  }

  #[inline]
  pub fn into_body(self) -> Body {
    self.body
  }

  pub fn new(status: impl Into<StatusCode>) -> Self {
    Self {
      status: status.into(),
      version: Version::default(),
      body: Body::empty(),
      extensions: Extensions::new(),
      headers: HeaderMap::new(),
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
    response.headers_mut().append(
      CONTENT_TYPE,
      HeaderValue::from_static("text/plain;charset=utf-8"),
    );

    *response.body_mut() = Body::from(message.to_string());

    response
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

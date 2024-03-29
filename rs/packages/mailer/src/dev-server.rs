pub mod error;
pub mod redactable;
pub mod render;
pub mod sample;
pub mod templates;

use std::net::SocketAddr;

use anyhow::Context;
use askama::Template;
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, StatusCode};
use prex::Response;
use redactable::Redactable;
use sample::Sample;
use templates::*;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
  let mut app = prex::prex();

  app.at("/").get(|_req, _next| async move {
    let body = r#"<!doctype html>
<html>
<head>
  <title>Template</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body>
    <h1>Templates</h1>
    <ul>
      <li><a href="/account-invitation">Account invitation</a></li>
      <li><a href="/user-recovery">User recovery</a></li>
      <li><a href="/email-verification">Email verification</a></li>
      <li><a href="/no-reply-autoreply">No reply autoreply</a></li>
    <ul>
</body>
</html>
"#;

    let mut res = Response::new(StatusCode::OK);
    res
      .headers_mut()
      .append(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    *res.body_mut() = Body::from(body);
    res
  });

  app
    .at("/account-invitation")
    .get(TemplateHandler(AccountInvitation::sample()));

  app
    .at("/user-recovery")
    .get(TemplateHandler(UserRecovery::sample()));

  app
    .at("/email-verification")
    .get(TemplateHandler(EmailVerification::sample()));

  app
    .at("/no-reply-autoreply")
    .get(TemplateHandler(NoReplyAutoreply::sample()));

  let app = app.build().context("prex build")?;

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

  println!("binding to port 3000");

  hyper::Server::try_bind(&addr)
    .context("bind")?
    .serve(app)
    .await
    .context("serve")?;

  Ok(())
}

pub struct TemplateHandler<T: std::fmt::Display + Send + Sync + 'static>(pub T);

#[async_trait::async_trait]
impl<T: Template + Redactable + Clone + Send + Sync + 'static> prex::handler::Handler
  for TemplateHandler<T>
{
  async fn call(&self, _: prex::Request, _: prex::Next) -> prex::Response {
    let render = render::render(self.0.clone()).unwrap();

    let html = format!(
      r#"<!doctype html>
<html>
<head>
    <title>Template</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0">
  <div class="page-content">
    {}
  </div>
  <hr />
  <div class="text" style="white-space:pre-wrap">{}</div>
  <hr />
  <div class="page-content">
  {}
  </div>
  <hr />
  <div class="text" style="white-space:pre-wrap">{}</div>
</body>
</html>"#,
      render.sendable.html, render.sendable.text, render.storable.html, render.storable.text,
    );

    let mut res = Response::new(StatusCode::OK);

    res
      .headers_mut()
      .append(CONTENT_TYPE, HeaderValue::from_static("text/html"));

    *res.body_mut() = Body::from(html);

    res
  }
}

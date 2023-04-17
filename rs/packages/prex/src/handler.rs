use futures::future::Future;

use crate::next::Next;
use crate::request::Request;
use crate::response::Response;

use async_trait::async_trait;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
  async fn call(&self, req: Request, next: Next) -> Response;
}

#[async_trait]
impl<F, Fut, Out> Handler for F
where
  F: Send + Sync + Clone + 'static + FnOnce(Request, Next) -> Fut,
  Fut: Send + 'static + Future<Output = Out>,
  Out: Into<Response>,
{
  async fn call(&self, req: Request, next: Next) -> Response {
    (self.clone())(req, next).await.into()
  }
}

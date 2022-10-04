use crate::matcher::Matcher;
use crate::handler::Handler;

pub struct Endpoint {
  pub(crate) matcher: Matcher,
  pub(crate) handler: Box<dyn Handler>
}
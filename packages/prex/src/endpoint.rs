use crate::handler::Handler;
use crate::matcher::Matcher;

pub struct Endpoint {
  pub(crate) matcher: Matcher,
  pub(crate) handler: Box<dyn Handler>,
}

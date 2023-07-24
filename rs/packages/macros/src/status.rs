pub trait GetStatus {
  fn status(&self) -> hyper::StatusCode;
}

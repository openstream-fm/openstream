#[tokio::main]
async fn main() {
  
  let handle1 = tokio::spawn(source::start(([0, 0, 0, 0], 20500)));
  let handle2 = tokio::spawn(source_hyper::start());
  let handle3 = tokio::spawn(stream::start());

  tokio::select! {
      r = handle1 => r.expect("source panicked").expect("source errored"),
      r = handle2 => r.expect("hyper source panicked"),
      r = handle3 => r.expect("stream panicked"),
  };
}

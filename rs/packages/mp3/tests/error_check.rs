mod common;

#[test]
fn error_check() {
  common::get_file("assets/error.mp3");
  let _meta = mp3::read_from_file("assets/error.mp3"); //.expect("File error");
}

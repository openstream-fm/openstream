pub const CHARSET: &str = "123456789abcdefghjkmnpqrstuvwxyz";

pub fn uid(len: usize) -> String {
  random_string::generate(len, CHARSET)
}

#[cfg(test)]
mod test {

  use super::CHARSET;

  #[test]
  fn charset_length() {
    assert_eq!(CHARSET.len(), 32);
  }
}

pub const CHARSET: &str = "0123456789abcdefghjkmnpqrstuvwxy";

pub fn uid(len: usize) -> String {
    random_string::generate(len, CHARSET)
}

#[cfg(test)]
mod test {

    use super::CHARSET;

    #[test]
    fn charset_length() {
        assert!(CHARSET.len() == 32);
    }
}

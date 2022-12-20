#[test]
pub fn can_parse_default_config() {
  crate::parse(include_str!("../../../../config.default.toml")).unwrap();
}

#[test]
pub fn can_parse_default_config_toml() {
  crate::parse(
    include_str!("../../../../openstream.sample.toml"),
    crate::ConfigFileFormat::Toml,
  )
  .unwrap();
}

#[test]
pub fn can_parse_default_config_json() {
  crate::parse(
    include_str!("../../../../openstream.sample.json"),
    crate::ConfigFileFormat::Json,
  )
  .unwrap();
}

#[test]
pub fn toml_and_json_default_config_are_equal() {
  let toml = crate::parse(
    include_str!("../../../../openstream.sample.toml"),
    crate::ConfigFileFormat::Toml,
  )
  .unwrap();

  let json = crate::parse(
    include_str!("../../../../openstream.sample.json"),
    crate::ConfigFileFormat::Json,
  )
  .unwrap();

  assert_eq!(toml, json);
}

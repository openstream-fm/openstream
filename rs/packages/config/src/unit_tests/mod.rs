#[test]
pub fn can_parse_default_config_toml() {
  crate::load_from_memory(Some((
    include_str!("../../../../../openstream.sample.toml"),
    crate::ConfigFileFormat::Toml,
  )))
  .unwrap();
}

#[test]
pub fn can_parse_default_config_json() {
  crate::load_from_memory(Some((
    include_str!("../../../../../openstream.sample.jsonc"),
    crate::ConfigFileFormat::Json,
  )))
  .unwrap();
}

#[test]
pub fn toml_and_json_default_config_are_equal() {
  let toml = crate::load_from_memory(Some((
    include_str!("../../../../../openstream.sample.toml"),
    crate::ConfigFileFormat::Toml,
  )))
  .unwrap();

  let json = crate::load_from_memory(Some((
    include_str!("../../../../../openstream.sample.jsonc"),
    crate::ConfigFileFormat::Json,
  )))
  .unwrap();

  assert_eq!(toml, json);
}

use schemars::{
  gen::{SchemaGenerator, SchemaSettings},
  JsonSchema,
};
use ts_rs::TS;

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
  #[error("io: {0}")]
  Io(#[from] std::io::Error),
  #[error("json: {0}")]
  Json(#[from] serde_json::Error),
  #[error("no export path")]
  NoExportPath,
}

pub fn default_settings() -> SchemaSettings {
  SchemaSettings::openapi3().with(|settings| {
    // settings.option_nullable = true;
    // settings.option_add_null_type = true;
    settings.inline_subschemas = true;
  })
}

pub fn default_generator() -> SchemaGenerator {
  default_settings().into_generator()
}

pub fn export_schema<T: JsonSchema>(target: &str) -> Result<(), ExportError> {
  let mut gen = default_generator();
  export_schema_with_generator::<T>(&mut gen, target)
}

pub fn export_schema_with_generator<T: JsonSchema>(
  gen: &mut SchemaGenerator,
  target: &str,
) -> Result<(), ExportError> {
  use std::{fs::File, io::Write};
  let schema = T::json_schema(gen);
  let contents = serde_json::to_string_pretty(&schema)?;

  let parent_dir = std::path::Path::new(target).parent().unwrap();
  std::fs::create_dir_all(parent_dir)?;

  let mut file = File::create(target)?;
  file.write_all(contents.as_bytes())?;
  Ok(())
}

fn get_target_from_ts<T: TS>() -> Option<String> {
  let path = T::EXPORT_TO?;
  let mapped = format!("{}.schema.json", path.trim_end_matches(".ts"));
  Some(mapped)
}

pub fn export_schema_from_ts_with_generator<T: JsonSchema + TS>(
  gen: &mut SchemaGenerator,
) -> Result<(), ExportError> {
  let target = get_target_from_ts::<T>();
  match target {
    None => Err(ExportError::NoExportPath),
    Some(target) => export_schema_with_generator::<T>(gen, &target),
  }
}

pub fn export_schema_from_ts<T: JsonSchema + TS>() -> Result<(), ExportError> {
  let mut gen = default_generator();
  export_schema_from_ts_with_generator::<T>(&mut gen)
}

#[macro_export]
macro_rules! export_schema_ts {
  ($ident:ident) => {
    paste::paste! {
      #[cfg(test)]
      #[test]
      #[allow(non_snake_case)]
      fn [<export_schema_ $ident>]() {
        $crate::export_schema_from_ts::<$ident>().unwrap();
      }
    }
  };
}

#[macro_export]
macro_rules! impl_schema_from {
  ($target:ident, $source:ident) => {
    impl ::schemars::JsonSchema for $target {
      fn is_referenceable() -> bool {
        $source::is_referenceable()
      }

      fn schema_id() -> ::std::borrow::Cow<'static, str> {
        $source::schema_id()
      }

      fn json_schema(gen: &mut ::schemars::gen::SchemaGenerator) -> ::schemars::schema::Schema {
        $source::json_schema(gen)
      }

      fn schema_name() -> String {
        $source::schema_name()
      }
    }
  };
}

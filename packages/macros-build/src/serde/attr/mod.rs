pub(crate) mod r#enum;
pub(crate) mod field;
pub(crate) mod r#struct;
pub(crate) mod variant;

// pub(crate) use field::*;
// pub(crate) use r#enum::*;
// pub(crate) use r#struct::*;
// pub(crate) use variant::*;

use std::convert::TryFrom;
use syn::{
  parse::{Parse, ParseStream},
  Error, Lit, Result, Token,
};

use super::util::syn_err;

#[derive(Copy, Clone, Debug)]
pub enum Inflection {
  Lower,
  Upper,
  Camel,
  Snake,
  Pascal,
  ScreamingSnake,
  Kebab,
}

impl Inflection {
  pub fn apply(self, string: &str) -> String {
    use inflector::Inflector;

    match self {
      Inflection::Lower => string.to_lowercase(),
      Inflection::Upper => string.to_uppercase(),
      Inflection::Camel => string.to_camel_case(),
      Inflection::Snake => string.to_snake_case(),
      Inflection::Pascal => string.to_pascal_case(),
      Inflection::ScreamingSnake => string.to_screaming_snake_case(),
      Inflection::Kebab => string.to_kebab_case(),
    }
  }
}

impl TryFrom<String> for Inflection {
  type Error = Error;

  fn try_from(value: String) -> Result<Self> {
    Ok(match &*value.to_lowercase().replace(['_', '-'], "") {
      "lowercase" => Self::Lower,
      "uppercase" => Self::Upper,
      "camelcase" => Self::Camel,
      "snakecase" => Self::Snake,
      "pascalcase" => Self::Pascal,
      "screamingsnakecase" => Self::ScreamingSnake,
      "kebabcase" => Self::Kebab,
      _ => syn_err!("invalid inflection: '{}'", value),
    })
  }
}

fn parse_assign_str(input: ParseStream) -> Result<String> {
  input.parse::<Token![=]>()?;
  match Lit::parse(input)? {
    Lit::Str(string) => Ok(string.value()),
    other => Err(Error::new(other.span(), "expected string")),
  }
}

fn parse_assign_inflection(input: ParseStream) -> Result<Inflection> {
  parse_assign_str(input).and_then(Inflection::try_from)
}

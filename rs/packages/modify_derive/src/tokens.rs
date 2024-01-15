use crate::fields::FieldInfo;
use proc_macro2::{self};

/// Creates a token stream applying the modifiers based on the field annotations.
pub(super) fn quote_field_modifiers(
  fields: Vec<FieldInfo>,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
  let mut modifications = vec![];
  let mut nested_validifies = vec![];

  for field_info in fields {
    let (mods, nested) = field_info.quote_validifes();
    modifications.extend(mods);
    nested_validifies.extend(nested);
  }

  (modifications, nested_validifies)
}

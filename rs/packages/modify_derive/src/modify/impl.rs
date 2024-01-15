use super::modifier::Modifier;
use crate::fields::FieldInfo;
use crate::tokens::quote_field_modifiers;
use proc_macro_error::abort;
use quote::quote;
use syn::parenthesized;

const TRIM_MODIFIER: &str = "trim";
const CUSTOM_MODIFIER: &str = "custom";
const UPPERCASE_MODIFIER: &str = "uppercase";
const LOWERCASE_MODIFIER: &str = "lowercase";
const CAPITALIZE_MODIFIER: &str = "capitalize";
const MODIFY: &str = "modify";
const NESTED: &str = "nested";

/// Impl entry point
pub fn impl_modify(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
  let ident = &input.ident;

  let field_info = FieldInfo::collect(input);

  let (modifiers, _nested_validifies) = quote_field_modifiers(field_info);

  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  quote!(
    impl #impl_generics ::modify::Modify for #ident #ty_generics #where_clause {
        fn modify(&mut self) {
            #(#modifiers)*
        }
    }
  )
}

pub fn collect_modifiers(modifiers: &mut Vec<Modifier>, field: &syn::Field) {
  for attr in &field.attrs {
    // Nest validified fields
    if attr.path().is_ident(NESTED) {
      modifiers.push(Modifier::Nested);
      continue;
    }

    if !attr.path().is_ident(MODIFY) {
      continue;
    }

    attr
      .parse_nested_meta(|meta| {
        if meta.path.is_ident(CUSTOM_MODIFIER) {
          let content;
          parenthesized!(content in meta.input);
          let path: syn::Path = content.parse()?;
          modifiers.push(Modifier::Custom { function: path });
          return Ok(());
        }

        if meta.path.is_ident(TRIM_MODIFIER) {
          modifiers.push(Modifier::Trim);
          return Ok(());
        }

        if meta.path.is_ident(LOWERCASE_MODIFIER) {
          modifiers.push(Modifier::Lowercase);
          return Ok(());
        }

        if meta.path.is_ident(UPPERCASE_MODIFIER) {
          modifiers.push(Modifier::Uppercase);
          return Ok(());
        }

        if meta.path.is_ident(CAPITALIZE_MODIFIER) {
          modifiers.push(Modifier::Capitalize);
          return Ok(());
        }

        Err(meta.error("Unrecognized modify parameter"))
      })
      .unwrap_or_else(|e| abort!(e.span(), e));
  }
}

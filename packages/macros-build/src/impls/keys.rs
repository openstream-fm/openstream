use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Fields, Item, LitStr /*Variant*/};

use crate::serde::{
  attr::{
    field::FieldAttr,
    r#enum::{EnumAttr, Tagged},
    r#struct::StructAttr,
    variant::VariantAttr,
  },
  util::to_serde_ident,
};

macro_rules! try_parse {
  ($expr:expr) => {
    match $expr {
      Ok(v) => v,
      Err(e) => return e.to_compile_error().into(),
    }
  };
}

pub fn keys(_args: TokenStream, tokens: TokenStream) -> TokenStream {
  let item: Item = parse_macro_input!(tokens);

  let mut keys: Vec<(Span, String, String)> = vec![];

  let name = match item {
    Item::Enum(ref item) => {
      let enum_attrs = try_parse!(EnumAttr::from_attrs(&item.attrs));
      let tagged = try_parse!(enum_attrs.tagged());

      match tagged {
        Tagged::Internally { tag } => {
          keys.push((item.span(), "ENUM_TAG".into(), tag.into()));
        }
        Tagged::Adjacently { tag, content } => {
          keys.push((item.span(), "ENUM_TAG".into(), tag.into()));
          keys.push((item.span(), "ENUM_CONTENT".into(), content.into()));
        }
        Tagged::Untagged { .. } => {}
        Tagged::Externally { .. } => {}
      };

      for variant in &item.variants {
        let variant_attrs = try_parse!(VariantAttr::from_attrs(&variant.attrs));
        if variant_attrs.skip {
          continue;
        }

        let variant_key = to_serde_ident(&variant.ident);
        let variant_value = match variant_attrs.rename {
          Some(name) => name,
          None => match variant_attrs.rename_all {
            None => variant_key.to_string(),
            Some(inflection) => inflection.apply(&variant_key),
          },
        };

        if matches!(tagged, Tagged::Externally) {
          keys.push((
            variant.span(),
            variant_key.trim_start_matches('_').to_string(),
            variant_value.clone(),
          ));
        }

        match &variant.fields {
          Fields::Unit => continue,
          Fields::Named(named) => {
            for field in &named.named {
              let field_attrs = try_parse!(FieldAttr::from_attrs(&field.attrs));
              let field_key = to_serde_ident(field.ident.as_ref().unwrap());
              let field_value = {
                match field_attrs.rename {
                  Some(ref name) => name.clone(),
                  None => match variant_attrs.rename_all {
                    Some(inflection) => inflection.apply(&field_key),
                    None => match enum_attrs.rename_all {
                      None => field_key.clone(),
                      Some(inflection) => inflection.apply(&field_key),
                    },
                  },
                }
              };

              let (key, value) = match tagged {
                Tagged::Untagged | Tagged::Internally { .. } => (field_key, field_value),
                Tagged::Adjacently { tag: _, content } => (
                  format!("ENUM_CONTENT_{}", field_key.trim_start_matches('_')),
                  format!("{content}.{field_value}"),
                ),
                Tagged::Externally { .. } => (
                  format!(
                    "{}_{}",
                    variant_key.trim_start_matches('_'),
                    field_key.trim_start_matches('_')
                  ),
                  format!("{variant_value}.{field_value}",),
                ),
              };

              if !keys.iter().any(|(_span, item_key, item_value)| {
                item_key.eq_ignore_ascii_case(&key) && item_value == &value
              }) {
                keys.push((field.span(), key.to_uppercase(), value));
              }
            }
          }

          Fields::Unnamed(unnamed) => {
            if unnamed.unnamed.len() > 1 {
              panic!("tuple enums with more than one value are not supported");
            }
          }
        }
      }

      &item.ident
    }

    Item::Struct(ref item) => match item.fields {
      Fields::Named(ref fields) => {
        let struct_attrs = try_parse!(StructAttr::from_attrs(&item.attrs));

        for field in &fields.named {
          let field_attrs = try_parse!(FieldAttr::from_attrs(&field.attrs));

          if field_attrs.skip || field_attrs.flatten {
            continue;
          }

          let key = to_serde_ident(field.ident.as_ref().unwrap()).to_uppercase();

          let value = {
            match field_attrs.rename {
              Some(name) => name,
              None => {
                let ident = to_serde_ident(field.ident.as_ref().unwrap());
                match struct_attrs.rename_all {
                  None => ident,
                  Some(inflection) => inflection.apply(&ident),
                }
              }
            }
          };

          keys.push((field.ident.span(), key, value));
        }

        &item.ident
      }

      Fields::Unnamed(_) => {
        panic!("tuple structs are not supported");
      }

      Fields::Unit => {
        panic!("unit structs are not supported");
      }
    },

    Item::Union(_) => {
      panic!("unions are not supported")
    }

    _ => {
      panic!("item kind not supported")
    }
  };

  let keys = keys.into_iter().map(|(span, name, value)| {
    let key = Ident::new(&format!("KEY_{name}"), span);
    let value = LitStr::new(&value, span);
    quote!( pub const #key: &'static str = #value )
  });

  let out = quote! {

    #item

    impl #name {
      #(#keys;)*
    }
  };

  // eprintln!("{}", out);

  out.into()
}

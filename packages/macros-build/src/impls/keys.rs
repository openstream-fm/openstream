use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Fields, Item, LitStr};

use crate::serde::{
  attr::{field::FieldAttr, r#struct::StructAttr},
  util::to_serde_ident,
};

pub fn keys(_args: TokenStream, tokens: TokenStream) -> TokenStream {
  let item: Item = parse_macro_input!(tokens);

  let mut keys: Vec<(Span, String, String)> = vec![];

  let name = match item {
    Item::Enum(ref _item) => {
      panic!("enums are not yet implemented")
      //item.ident
    }

    Item::Struct(ref item) => match item.fields {
      Fields::Named(ref fields) => {
        let struct_attrs = match StructAttr::from_attrs(&item.attrs) {
          Ok(v) => v,
          Err(e) => return e.to_compile_error().into(),
        };

        for field in &fields.named {
          let field_attrs = match FieldAttr::from_attrs(&field.attrs) {
            Ok(v) => v,
            Err(e) => return e.to_compile_error().into(),
          };

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

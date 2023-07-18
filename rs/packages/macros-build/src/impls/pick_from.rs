use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, Item, Path};

pub fn pick_from(args: TokenStream, tokens: TokenStream) -> TokenStream {
  let target = parse_macro_input!(args as Path);
  let item: Item = parse_macro_input!(tokens);
  let mut fields_impl = Vec::<proc_macro2::TokenStream>::new();

  let name = match &item {
    Item::Struct(item) => match &item.fields {
      Fields::Unnamed(_) => panic!("tuple structs are not supported"),
      Fields::Unit => panic!("unit structs are not supported"),
      Fields::Named(fields) => {
        for field in &fields.named {
          let ident = &field.ident;
          fields_impl.push(quote! {
            #ident: ::std::convert::From::from(src.#ident),
          });
        }
        &item.ident
      }
    },
    other => panic!("item of type {other:?} is not supported, only structs are supported"),
  };

  let out = quote! {
    #item

    impl From<#target> for #name {
      fn from(src: #target) -> Self {
        Self {
          #(#fields_impl)*
        }
      }
    }
  };

  out.into()
}

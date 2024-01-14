use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

pub fn schema_ts_export(_args: TokenStream, tokens: TokenStream) -> TokenStream {
  let item: Item = parse_macro_input!(tokens);

  let name = match &item {
    Item::Struct(item) => item.ident.clone(),
    Item::Enum(item) => item.ident.clone(),
    other => panic!("item of type {other:?} is not supported, only structs are supported"),
  };

  let out = quote! {
    #item

    ::openapi::export_schema_ts!(#name);
  };

  out.into()
}

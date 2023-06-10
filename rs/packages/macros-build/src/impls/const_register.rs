use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

pub fn const_register(_args: TokenStream, tokens: TokenStream) -> TokenStream {
  let item: Item = parse_macro_input!(tokens);

  let ident = match &item {
    Item::Const(item) => item.ident.clone(),
    _ => panic!("only const values are allowed"),
  };

  quote!(
    #item

    macros::register_const!(#ident);
  )
  .into()
}

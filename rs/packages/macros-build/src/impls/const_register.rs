use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Attribute, Item, Lit, Meta};

fn parse_doc_comments(attrs: &[Attribute]) -> String {
  let lines = attrs
    .iter()
    .filter(|attr| attr.path.is_ident("doc"))
    .filter_map(|attr| match attr.parse_meta() {
      Ok(Meta::NameValue(name_value)) if name_value.path.is_ident("doc") => Some(name_value.lit),
      _ => None,
    })
    .filter_map(|lit| match lit {
      Lit::Str(s) => Some(s.value()),
      _ => None,
    })
    .collect::<Vec<_>>();

  let trimmed: Vec<_> = lines
    .iter()
    .flat_map(|lit| lit.split('\n').collect::<Vec<_>>())
    .map(|line| line.trim().to_string())
    .collect();

  trimmed.join("\n").trim().to_string()
}

pub fn const_register(_args: TokenStream, tokens: TokenStream) -> TokenStream {
  let item: Item = parse_macro_input!(tokens);

  let ident: Ident;
  let comments: String;

  match &item {
    Item::Const(item) => {
      ident = item.ident.clone();
      comments = parse_doc_comments(&item.attrs);
    }
    _ => panic!("only const values are allowed"),
  };

  quote!(
    #item

    macros::register_const!(#ident, #comments);
  )
  .into()
}

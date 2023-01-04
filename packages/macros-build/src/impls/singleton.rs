use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput, Meta, NestedMeta};

pub fn singleton(tokens: TokenStream) -> TokenStream {
  let input = parse_macro_input!(tokens as DeriveInput);

  let name = input.ident;

  let collection = 'collection: {
    for attr in input.attrs {
      if attr.path.is_ident("singleton") {
        // eprintln!("attr.path.is_ident \"singleton\"");
        match attr.parse_meta().expect("parse meta") {
          Meta::NameValue(meta) => {
            panic!("expecting meta collection = 'name' got NameValue {meta:#?} => ")
          }

          Meta::Path(meta) => panic!("expecting meta collection = 'name' got Path => {meta:#?}"),

          Meta::List(list) => {
            // eprintln!("Meta::List");
            // eprintln!("{list:#?}");
            for item in list.nested {
              #[allow(clippy::collapsible_match)]
              if let NestedMeta::Meta(meta) = item {
                if let Meta::NameValue(name_value) = meta {
                  if name_value.path.is_ident("collection") {
                    // eprintln!("name_value.is_ident(\"collection\")");
                    break 'collection name_value.lit;
                  }
                }
              }
            }
          }
        }
      }
    }

    panic!("collection attribute is required for derive(Singleton)");
  };

  let out = quote! {
    #[async_trait::async_trait]
    impl crate::Model for #name {

      const UID_LEN: usize =  crate::SINGLETON_UID_LEN;

      const CL_NAME: &'static str = #collection;

      fn uid() -> String {
        crate::singleton_uid()
      }

      async fn ensure_collection() -> Result<(), mongodb::error::Error> {
        use crate::{Model, Singleton};
        Self::ensure_indexes().await?;
        Self::ensure_instance().await?;
        Ok(())
      }
    }

    impl crate::Singleton for #name {};
  };

  // eprintln!("{}", out);

  out.into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Fields, Item, LitStr};

pub fn status(tokens: TokenStream) -> TokenStream {
  //let target = parse_macro_input!(args as Path);
  let item: Item = parse_macro_input!(tokens);
  let mut variant_tokens = Vec::<proc_macro2::TokenStream>::new();

  let ident = match &item {
    Item::Enum(item) => {
      for variant in &item.variants {
        let mut expr = None;
        for attr in &variant.attrs {
          let meta = attr.parse_meta().expect("could not parse attr meta");
          if meta
            .path()
            .get_ident()
            .expect("could not get attr meta ident")
            == "status"
          {
            let status_expr = attr
              .parse_args::<LitStr>()
              .expect("could not parse attr status expression");
            expr = Some(
              status_expr
                .value()
                .parse::<proc_macro2::TokenStream>()
                .expect("cannot parse status attr token stream"),
            );
            break;
          }
        }

        let expr = expr.expect("could not find status attribute");

        let name = variant.ident.clone();

        let fields_pattern = match &variant.fields {
          Fields::Unit => quote! {{}},
          Fields::Unnamed(unnamed) => {
            let idents = unnamed
              .unnamed
              .iter()
              .enumerate()
              .map(|(i, field)| syn::Ident::new(&format!("p{}", i), field.span()));

            quote! {
              (#(#idents,)*)
            }
          }

          Fields::Named(named) => {
            let idents = named.named.iter().map(|member| &member.ident);
            quote! { {
              #(#idents,)*
            } }
          }
        };

        variant_tokens.push(quote! {
          Self::#name #fields_pattern => #expr
        });
      }

      item.ident.clone()
    }

    _ => panic!("only enums are supported"),
  };

  let out = quote! {
    impl ::macros::GetStatus for #ident {
      fn status(&self) -> ::hyper::StatusCode {
        #[allow(unused)]
        match self {
          #(#variant_tokens,)*
        }
      }
    }
  };

  out.into()
}

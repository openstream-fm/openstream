use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, LitStr};

#[derive(Debug, Clone, Default)]
pub struct DocComments {
    pub lines: Vec<String>,
}

impl DocComments {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut lines = vec![];
        for attr in attrs {
            if !attr.path.is_ident("doc") {
                continue;
            }
            match attr.parse_args() {
                Ok(syn::Lit::Str(lit)) => {
                    lines.push(lit.value());
                }
                _ => {}
            }
        }

        Self { lines }
    }
}

impl ToTokens for DocComments {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.lines.is_empty() {
            tokens.extend(quote!(""))
        } else {
            let lines = self
                .lines
                .iter()
                .map(|line| LitStr::new(line, Span::call_site()))
                .map(|lit| quote!(["/// ", #lit].join(""),))
                .collect::<TokenStream>();

            tokens.extend(quote! {
              ["", #(#lines) ""].join('\n')
            })
        }
    }
}

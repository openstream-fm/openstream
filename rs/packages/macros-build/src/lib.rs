pub(crate) mod impls;
pub(crate) mod serde;

use proc_macro::TokenStream;

#[proc_macro_derive(Singleton, attributes(singleton))]
pub fn singleton(tokens: TokenStream) -> TokenStream {
  impls::singleton::singleton(tokens)
}

#[proc_macro_attribute]
pub fn keys(args: TokenStream, tokens: TokenStream) -> TokenStream {
  impls::keys::keys(args, tokens)
}

#[proc_macro_attribute]
pub fn const_register(args: TokenStream, tokens: TokenStream) -> TokenStream {
  impls::const_register::const_register(args, tokens)
}

// #[proc_macro_attribute]
// pub fn validate(args: TokenStream, tokens: TokenStream) -> TokenStream {
//   todo!();
//   //impls::validate::validate(args, tokens);
// }

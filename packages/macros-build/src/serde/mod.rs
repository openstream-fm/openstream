pub mod attr;
pub mod util;

// Derives [TS](./trait.TS.html) for a struct or enum.
// Please take a look at [TS](./trait.TS.html) for documentation.
// #[proc_macro_derive(TS, attributes(ts))]
// pub fn typescript(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   match entry(input) {
//     Err(err) => err.to_compile_error(),
//     Ok(result) => result,
//   }
//   .into()
// }

// fn parse_item(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
//   let input = syn::parse::<Item>(input)?;
//   let (ident, generics) = match input {
//     Item::Struct(item) => item,
//     Item::Enum(item) => item,
//     _ => syn_err!(input.span(); "unsupported item"),
//   };
// }

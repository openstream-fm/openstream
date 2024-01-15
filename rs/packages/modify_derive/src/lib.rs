use proc_macro_error::proc_macro_error;
mod fields;
mod modify;
mod tokens;

#[proc_macro_derive(Modify, attributes(modify, nested))]
#[proc_macro_error]
pub fn derive_modify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse(input).unwrap();
  modify::r#impl::impl_modify(&ast).into()
}

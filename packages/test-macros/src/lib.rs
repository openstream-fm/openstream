use quote::quote;

#[proc_macro_attribute]
pub fn async_test(
  _attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let item_fn = syn::parse_macro_input!(item as syn::ItemFn);

  let mut new_sig = item_fn.sig.clone();
  new_sig.asyncness = None;
  let block = item_fn.block;

  quote! {
    #[test]
    #new_sig {
      ::std::thread::spawn(move || {
        ::test_util::test_runtime().block_on(async move { #block })
      }).join().unwrap()
    }
  }
  .into()
}

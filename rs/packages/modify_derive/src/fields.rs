use crate::modify::{modifier::Modifier, r#impl::collect_modifiers};
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

/// Holds the combined validations and modifiers for one field
#[derive(Debug)]
pub struct FieldInfo {
  /// The original field
  pub field: syn::Field,

  /// The field's original name if annotated with `serde(rename)``
  pub name: String,

  /// Modifier annotations
  pub modifiers: Vec<Modifier>,
}

impl FieldInfo {
  pub fn new(field: syn::Field, name: String, modifiers: Vec<Modifier>) -> Self {
    FieldInfo {
      field,
      name,
      modifiers,
    }
  }

  /// Used by both the `Validate` and `modify` implementations. Validate ignores the modifiers.
  pub fn collect(input: &syn::DeriveInput) -> Vec<Self> {
    let syn::Data::Struct(syn::DataStruct { ref fields, .. }) = input.data else {
      abort!(
        input.span(),
        "#[derive(Validate/modify)] can only be used on structs with named fields"
      )
    };

    fields
      .into_iter()
      .map(|field| {
        let field_ident = field
          .ident
          .as_ref()
          .expect("Found unnamed field")
          .to_string();

        let modifiers = collect_field_attributes(field);

        Self::new(field.clone(), field_ident, modifiers)
      })
      .collect::<Vec<_>>()
  }

  /// Returns the modification tokens as the first element and any nested validifes as the second.
  pub fn quote_validifes(&self) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut nested_validifies = vec![];
    let mut quoted_modifications = vec![];

    for modifier in self.modifiers.iter() {
      let (tokens, nested) = modifier.to_modify_tokens(self);
      quoted_modifications.push(tokens);
      if let Some(nested) = nested {
        nested_validifies.push(nested);
      }
    }

    (quoted_modifications, nested_validifies)
  }

  /// Returns `self.#ident`, unless the field is an option in which case it just
  /// returns an `#ident` as we always do a `if let` check on Option fields
  pub fn quote_modifier_param(&self) -> proc_macro2::TokenStream {
    let ident = &self.field.ident;

    if self.is_reference() {
      abort!(
        ident.span(),
        "Fields containing modifiers must contain owned data"
      )
    }

    if self.is_option() {
      quote!(#ident)
    } else {
      quote!(self.#ident)
    }
  }

  pub fn wrap_modifier_if_option(
    &self,
    tokens: proc_macro2::TokenStream,
  ) -> proc_macro2::TokenStream {
    let field_ident = &self.field.ident;

    if self.is_option() {
      let this = self.option_self_tokens_modifications();
      return quote!(
          if let #this = self.#field_ident.as_mut() {
              #tokens
          }
      );
    }

    tokens
  }

  /// Wrap the quoted output of a modification in a for loop if
  /// the field type is a collection.
  pub fn wrap_modifier_if_collection(
    &self,
    param: proc_macro2::TokenStream,
    tokens: proc_macro2::TokenStream,
    modifier: &Modifier,
  ) -> proc_macro2::TokenStream {
    if !self.is_list() {
      return tokens;
    }

    let modified = match modifier {
      Modifier::Trim => quote!(el.trim().to_string()),
      Modifier::Uppercase => quote!(el.to_uppercase()),
      Modifier::Lowercase => quote!(el.to_lowercase()),
      Modifier::Capitalize => {
        quote!(::std::format!("{}{}", &el[0..1].to_uppercase(), &el[1..]))
      }
      _ => unreachable!("modifier is never wrapped"),
    };

    quote!(
        for el in #param.iter_mut() {
            *el = #modified
        }
    )
  }

  /// Returns true if the field is an option.
  pub fn is_option(&self) -> bool {
    let syn::Type::Path(ref p) = self.field.ty else {
      return false;
    };

    p.path
      .segments
      .last()
      .is_some_and(|seg| seg.ident == "Option")
  }

  /// Returns true if the field is &'_ T, or Option<&'_ T>.
  pub fn is_reference(&self) -> bool {
    is_reference(&self.field.ty)
  }

  pub fn is_list(&self) -> bool {
    is_list(&self.field.ty)
  }

  fn option_self_tokens_modifications(&self) -> proc_macro2::TokenStream {
    let ident = &self.field.ident;
    let mut tokens = quote!(#ident);
    let mut ty = &self.field.ty;

    while let Some(typ) = try_extract_option(ty) {
      tokens = quote!(Some(#tokens));
      ty = typ;
    }
    tokens
  }
}

fn is_reference(ty: &syn::Type) -> bool {
  // Strip any `Option`s
  if let Some(ty) = try_extract_option(ty) {
    return is_reference(ty);
  }

  matches!(ty, syn::Type::Reference(_))
}

fn is_list(ty: &syn::Type) -> bool {
  if let Some(ty) = try_extract_option(ty) {
    return is_list(ty);
  }

  // We consider arrays lists
  if let syn::Type::Array(_) = ty {
    return true;
  }

  // If it's not a path, it's not a list
  let syn::Type::Path(p) = ty else {
    return false;
  };

  // Always check the last arg such as in `std::vec::Vec`
  let Some(seg) = p.path.segments.last() else {
    return false;
  };

  seg.ident == "Vec" || seg.ident == "HashSet" || seg.ident == "BTreeSet" || seg.ident == "IndexSet"
}

fn try_extract_option(ty: &syn::Type) -> Option<&syn::Type> {
  let syn::Type::Path(p) = ty else {
    return None;
  };

  // Always check the last arg such as in `std::vec::Vec`
  let seg = p.path.segments.last()?;

  if &seg.ident != "Option" {
    return None;
  }

  let syn::PathArguments::AngleBracketed(ref ab) = seg.arguments else {
    return None;
  };

  let arg = ab.args.last()?;

  match arg {
    syn::GenericArgument::Type(ty) => Some(ty),
    _ => None,
  }
}

/// Find everything we need to know about a field: its real name if it's changed from the deserialization
/// and the list of validators and modifiers to run on it
fn collect_field_attributes(field: &syn::Field) -> Vec<Modifier> {
  let mut modifiers = vec![];

  collect_modifiers(&mut modifiers, field);

  modifiers
}

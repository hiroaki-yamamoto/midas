use ::proc_macro::TokenStream;

use ::quote::quote;
use ::syn::{ExprPath, Generics, Ident, Type, WhereClause};

pub fn impl_trait(
  generics: &Generics,
  tr: &[ExprPath],
  name: &Ident,
  cmd_constraint: &WhereClause,
) -> TokenStream {
  let mut implement = TokenStream::new();
  tr.into_iter()
    .map(|tr: &ExprPath| {
      return quote! {
        impl #generics #tr #generics for #name #generics #cmd_constraint {}
      };
    })
    .map(|ts| ts.into())
    .for_each(|ts: TokenStream| implement.extend(ts));
  return implement;
}

pub fn impl_trait_with_vtype(
  generics: &Generics,
  vtype: &Type,
  tr: &[ExprPath],
  name: &Ident,
  cmd_constraint: &WhereClause,
) -> TokenStream {
  let mut implement = TokenStream::new();
  tr.into_iter()
    .map(|tr: &ExprPath| {
      return quote! {
        impl #generics #tr <T, #vtype> for #name #generics #cmd_constraint {}
      };
    })
    .map(|ts| ts.into())
    .for_each(|ts: TokenStream| implement.extend(ts));
  return implement;
}

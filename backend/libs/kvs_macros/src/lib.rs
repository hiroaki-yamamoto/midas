mod kvs_args;
mod structure;
mod trait_impl;

use ::proc_macro::TokenStream;
use ::quote::quote;
use ::syn::{parse_macro_input, parse_quote, DeriveInput, Type};

use crate::structure::make_structure;
use crate::trait_impl::{impl_trait, impl_trait_with_vtype};

use crate::kvs_args::KVSArgs;

/// Derives LastCheck trait.
#[proc_macro_derive(LastCheck, attributes(value_type))]
pub fn impl_last_check(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;
  let vtype: Type = input.attrs[0].parse_args().unwrap();
  let generics = input.generics;
  let cmd_constraint = parse_quote! {
    where #generics: ::kvs::redis::Commands + Send + Sync
  };

  let mut traits_impl = impl_trait(
    &generics,
    &[
      parse_quote!(::kvs::traits::last_checked::Base),
      parse_quote!(::kvs::traits::lats_checked::Expiration),
      parse_quote!(::kvs::traits::last_checked::Remove),
    ],
    &name,
    &cmd_constraint,
  );

  traits_impl.extend(impl_trait_with_vtype(
    &generics,
    &vtype,
    &[
      parse_quote!(::kvs::traits::last_checked::Get),
      parse_quote!(::kvs::traits::last_checked::Set),
      parse_quote!(::kvs::traits::last_checked::LastCheckStore),
      parse_quote!(::kvs::traits::last_checked::ListOp),
    ],
    &name,
    &cmd_constraint,
  ));
  return traits_impl;
}

/// Constructs KVS structure.
///
/// Arguments:
/// 1. Visibility
/// 2. Name
/// 3. Value type
/// 4. Channel name
#[proc_macro]
pub fn kvs(input: TokenStream) -> TokenStream {
  let parsed: KVSArgs = parse_macro_input!(input as KVSArgs);
  let vis = parsed.vis;
  let name = parsed.name;
  let value_type = parsed.vtype;
  let ch_name = parsed.ch_name;
  let (mut structure, generics, cmd_constraint) = make_structure(&vis, &name);
  let ch_impl = TokenStream::from(quote! {
    impl #generics ::kvs::traits::normal::ChannelName for #name <T>
    #cmd_constraint
    {
      fn channel_name(
        &self,
        key: impl AsRef<str> + ::std::fmt::Display
      ) -> String where
      {
        return format!(#ch_name, key);
      }
    }
  });
  let first_traits = impl_trait(
    &generics,
    &[
      parse_quote!(::kvs::traits::normal::Exist),
      parse_quote!(::kvs::traits::normal::Expiration),
      parse_quote!(::kvs::traits::normal::Lock),
      parse_quote!(::kvs::traits::normal::Remove),
    ],
    &name,
    &cmd_constraint,
  );
  let second_traits = impl_trait_with_vtype(
    &generics,
    &value_type,
    &[
      parse_quote!(::kvs::traits::normal::Get),
      parse_quote!(::kvs::traits::normal::Set),
      parse_quote!(::kvs::traits::normal::Store),
      parse_quote!(::kvs::traits::normal::ListOp),
    ],
    &name,
    &cmd_constraint,
  );
  structure.extend(ch_impl);
  structure.extend(first_traits);
  structure.extend(second_traits);
  return structure;
}

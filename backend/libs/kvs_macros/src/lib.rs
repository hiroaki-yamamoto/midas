mod kvs_args;
mod structure;
mod trait_impl;

use ::proc_macro::TokenStream;
use ::syn::{parse_macro_input, parse_quote};

use crate::structure::Expanded;

use crate::kvs_args::KVSArgs;

/// Declare KVS structure with exchange and symbol as "key".
///
/// Arguments:
/// 1. Visibility
/// 2. Name
/// 3. Value type
/// 4. Channel name
#[proc_macro]
pub fn symbol_kvs(input: TokenStream) -> TokenStream {
  let parsed: KVSArgs = parse_macro_input!(input as KVSArgs);
  let expand = Expanded::new(parsed)
    .impl_symbol_channel_name()
    .impl_trait(&[
      parse_quote!(::kvs::traits::symbol::Incr),
      parse_quote!(::kvs::traits::symbol::Remove),
    ])
    .impl_trait_with_vtype(&[
      parse_quote!(::kvs::traits::symbol::Get),
      parse_quote!(::kvs::traits::symbol::Set),
    ]);
  return expand.build();
}

/// Declare KVS structure with last checked date.
///
/// Arguments:
/// 1. Visibility
/// 2. Name
/// 3. Value type
/// 4. Channel name
#[proc_macro]
pub fn last_check_kvs(input: TokenStream) -> TokenStream {
  let parsed: KVSArgs = parse_macro_input!(input as KVSArgs);
  let expand = Expanded::new(parsed)
    .impl_normal_channel_name()
    .impl_trait(&[
      parse_quote!(::kvs::traits::normal::Exist),
      parse_quote!(::kvs::traits::normal::Expiration),
      parse_quote!(::kvs::traits::normal::Lock),
      parse_quote!(::kvs::traits::normal::Remove),
      parse_quote!(::kvs::traits::last_checked::Base),
      parse_quote!(::kvs::traits::last_checked::Expiration),
      parse_quote!(::kvs::traits::last_checked::Remove),
    ])
    .impl_trait_with_vtype(&[
      parse_quote!(::kvs::traits::normal::Get),
      parse_quote!(::kvs::traits::normal::Set),
      parse_quote!(::kvs::traits::normal::Store),
      parse_quote!(::kvs::traits::normal::ListOp),
      parse_quote!(::kvs::traits::last_checked::Get),
      parse_quote!(::kvs::traits::last_checked::Set),
      parse_quote!(::kvs::traits::last_checked::ListOp),
      parse_quote!(::kvs::traits::last_checked::LastCheckStore),
    ]);

  return expand.build();
}

/// Declare KVS structure.
///
/// Arguments:
/// 1. Visibility
/// 2. Name
/// 3. Value type
/// 4. Channel name
#[proc_macro]
pub fn kvs(input: TokenStream) -> TokenStream {
  let parsed: KVSArgs = parse_macro_input!(input as KVSArgs);
  let expand = Expanded::new(parsed)
    .impl_normal_channel_name()
    .impl_trait(&[
      parse_quote!(::kvs::traits::normal::Exist),
      parse_quote!(::kvs::traits::normal::Expiration),
      parse_quote!(::kvs::traits::normal::Lock),
      parse_quote!(::kvs::traits::normal::Remove),
    ])
    .impl_trait_with_vtype(&[
      parse_quote!(::kvs::traits::normal::Get),
      parse_quote!(::kvs::traits::normal::Set),
      parse_quote!(::kvs::traits::normal::Store),
      parse_quote!(::kvs::traits::normal::ListOp),
    ]);
  return expand.build();
}

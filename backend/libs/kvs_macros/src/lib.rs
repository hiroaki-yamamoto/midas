mod kvs_args;

use ::proc_macro::TokenStream;
use ::quote::quote;
use ::syn::{parse_macro_input, parse_quote, ExprPath, WhereClause};

use crate::kvs_args::KVSArgs;

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
  let cmd_constraint: WhereClause = parse_quote! {
    where T: ::kvs::redis::Commands + Send + Sync
  };
  let mut ret = TokenStream::from(quote! {
    #vis struct #name <T> #cmd_constraint,
    {
      con: ::kvs::Connection <T>,
    }

    impl <T> kvs::traits::normal::Base <T> for #name <T>
    #cmd_constraint
    {
      fn commands(&self) -> ::tokio::sync::MutexGuard <T> {
        return self.con.clone();
      }
    }

    impl <T> ::kvs::traits::normal::ChannelName for #name <T>
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
  let first_traits: Vec<TokenStream> = [
    parse_quote!(::kvs::traits::normal::Exist),
    parse_quote!(::kvs::traits::normal::Expiration),
    parse_quote!(::kvs::traits::normal::Lock),
    parse_quote!(::kvs::traits::normal::Remove),
  ]
  .into_iter()
  .map(|tr: ExprPath| {
    return quote! {
      impl <T> #tr <T> for #name <T> #cmd_constraint {}
    };
  })
  .map(|ts| ts.into())
  .collect();
  let second_traits: Vec<TokenStream> = [
    parse_quote!(::kvs::traits::normal::Get),
    parse_quote!(::kvs::traits::normal::Set),
    parse_quote!(::kvs::traits::normal::Store),
    parse_quote!(::kvs::traits::normal::ListOp),
  ]
  .into_iter()
  .map(|tr: ExprPath| {
    return quote! {
      impl <T> #tr <T, #value_type> for #name <T> #cmd_constraint {}
    };
  })
  .map(|ts| ts.into())
  .collect();
  ret.extend(first_traits);
  ret.extend(second_traits);
  return ret;
}

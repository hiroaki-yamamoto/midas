mod kvs_args;

use ::proc_macro::TokenStream;
use ::quote::quote;
use ::syn::{parse_macro_input, DeriveInput};

use crate::kvs_args::KVSArgs;

#[proc_macro_derive(NormalChannelName, attributes(ch_name))]
pub fn impl_normal_channel_name(input: TokenStream) -> TokenStream {
  let parsed = parse_macro_input!(input as DeriveInput);
  let ch_name = parsed.attrs[0].parse_args::<syn::ExprLit>().unwrap();
  let name = parsed.ident;
  let generics = parsed.generics;
  let where_clause = generics.where_clause.clone();

  let expand = quote! {
    impl #generics kvs::traits::normal::ChannelName for #name #generics #where_clause
    {
      fn channel_name(
        &self,
        key: impl AsRef<str> + ::std::fmt::Display
      ) -> String where
      {
        return format!(#ch_name, key);
      }
    }
  };
  return TokenStream::from(expand);
}

#[proc_macro]
pub fn kvs(input: TokenStream) -> TokenStream {
  let parsed: KVSArgs = parse_macro_input!(input as KVSArgs);
  let vis = parsed.vis;
  let name = parsed.name;
  // let value_type = parsed.vtype;
  let ch_name = parsed.ch_name;
  let ret = quote! {
    #[derive(kvs_macros::NormalBase, kvs_macros::NormalChannelName)]
    #[ch_name(#ch_name)]
    #vis struct #name<T>
    where
      T: ::kvs::redis::Commands + Send + Sync,
    {
      con: ::kvs::Connection<T>,
    }

    impl<T> kvs::traits::normal::Base<T> for #name where T: ::kvs::redis::Commands + Send + Sync
    {
      fn commands(&self) -> ::tokio::sync::MutexGuard<T> {
        return self.con.clone();
      }
    }
  };
  return TokenStream::from(ret);
}

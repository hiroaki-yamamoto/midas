use ::proc_macro::TokenStream;

use ::syn::{
  parse_quote, ExprPath, Generics, Ident, LitStr, Type, Visibility, WhereClause,
};

use crate::kvs_args::KVSArgs;
use crate::trait_impl::{impl_trait, impl_trait_with_vtype};

use ::quote::quote;

pub struct Expanded {
  implement: TokenStream,
  generics: Generics,
  cmd_constraint: WhereClause,
  vtype: Type,
  ch_name: LitStr,
  name: Ident,
}

impl Expanded {
  pub fn new(args: KVSArgs) -> Self {
    let vis = args.vis;
    let name = args.name;
    let value_type = args.vtype;
    let ch_name = args.ch_name;
    let (imple, generics, cmd_constraint) = Self::make_structure(&vis, &name);
    return Self {
      implement: imple,
      vtype: value_type,
      generics,
      cmd_constraint,
      ch_name,
      name,
    };
  }

  pub fn build(self) -> TokenStream {
    return self.implement;
  }

  pub fn impl_symbol_channel_name(self) -> Self {
    let generics = self.generics.clone();
    let cmd_constraint = self.cmd_constraint.clone();
    let ch_name = self.ch_name.clone();
    let name = self.name.clone();
    let mut structure = self.implement;
    let imple = TokenStream::from(quote! {
      impl #generics ::kvs::traits::symbol::ChannelName for #name <T>
      #cmd_constraint
      {
        fn channel_name(
          &self,
          exchange: impl AsRef<str> + ::std::fmt::Display,
          symbol: impl AsRef<str> + ::std::fmt::Display
        ) -> String
        {
          return format!(#ch_name, exchange, symbol);
        }
      }
    });
    structure.extend(imple);
    return Self {
      implement: structure,
      ..self
    };
  }

  pub fn impl_normal_channel_name(self) -> Self {
    let generics = self.generics.clone();
    let cmd_constraint = self.cmd_constraint.clone();
    let ch_name = self.ch_name.clone();
    let name = self.name.clone();
    let mut structure = self.implement;
    let imple = TokenStream::from(quote! {
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
    structure.extend(imple);
    return Self {
      implement: structure,
      ..self
    };
  }

  pub fn impl_trait(self, traits: &[ExprPath]) -> Self {
    let mut implement = self.implement;
    implement.extend(impl_trait(
      &self.generics,
      traits,
      &self.name,
      &self.cmd_constraint,
    ));
    return Self { implement, ..self };
  }

  pub fn impl_trait_with_vtype(self, traits: &[ExprPath]) -> Self {
    let mut implement = self.implement;
    implement.extend(impl_trait_with_vtype(
      &self.generics,
      &self.vtype,
      traits,
      &self.name,
      &self.cmd_constraint,
    ));
    return Self { implement, ..self };
  }

  fn make_structure(
    vis: &Visibility,
    name: &Ident,
  ) -> (TokenStream, Generics, WhereClause) {
    let generics = parse_quote! {
      <T>
    };
    let cmd_constraint: WhereClause = parse_quote! {
      where T: ::kvs::redis::Commands + Send + Sync
    };

    let implement = TokenStream::from(quote! {
      #vis struct #name #generics #cmd_constraint,
      {
        con: ::kvs::Connection #generics,
      }

      impl #generics kvs::traits::normal::Base #generics for #name #generics
      #cmd_constraint
      {
        fn commands(&self) -> ::std::sync::Arc<::tokio::sync::Mutex #generics> {
          return self.con.clone();
        }
      }
    });

    return (implement, generics, cmd_constraint);
  }
}

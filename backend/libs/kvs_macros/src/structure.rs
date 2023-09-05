use ::proc_macro::TokenStream;

use ::syn::{parse_quote, Generics, Ident, Visibility, WhereClause};

use ::quote::quote;

pub fn make_structure(
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
      fn commands(&self) -> ::tokio::sync::MutexGuard #generics {
        return self.con.clone();
      }
    }
  });

  return (implement, generics, cmd_constraint);
}

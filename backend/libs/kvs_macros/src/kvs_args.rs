use ::syn::parse::{Parse, ParseStream};
use ::syn::token::Comma;
use ::syn::{Ident, LitStr, Type, Visibility};

pub struct KVSArgs {
  pub vis: Visibility,
  pub name: Ident,
  pub vtype: Type,
  pub ch_name: LitStr,
}

impl Parse for KVSArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let vis: Visibility = input.parse()?;
    let _: Comma = input.parse()?;
    let name: Ident = input.parse()?;
    let _: Comma = input.parse()?;
    let vtype: Type = input.parse()?;
    let _: Comma = input.parse()?;
    let ch_name: LitStr = input.parse()?;
    return Ok(Self {
      vis,
      name,
      vtype,
      ch_name,
    });
  }
}

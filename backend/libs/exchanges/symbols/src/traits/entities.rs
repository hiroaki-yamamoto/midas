use ::rpc::symbols::SymbolInfo;

pub trait Symbol: Into<SymbolInfo> {
  fn symbol(&self) -> String;
}

use super::symbol_info::SymbolInfo;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolList {
  pub symbols: Vec<Box<SymbolInfo>>,
}

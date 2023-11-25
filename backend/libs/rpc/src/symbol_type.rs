
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SymbolType {
  Crypto,
  Stock,
}

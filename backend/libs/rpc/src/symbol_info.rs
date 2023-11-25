
#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
  pub base: String,
  pub base_commission_precision: i64,
  pub base_precision: i64,
  pub quote: String,
  pub quote_commission_precision: i64,
  pub quote_precision: i64,
  pub status: String,
  pub symbol: String,
}

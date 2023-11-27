
#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookticker {
  pub ask_price: String,
  pub ask_qty: String,
  pub bid_price: String,
  pub bid_qty: String,
  pub id: String,
  pub symbol: String,
}

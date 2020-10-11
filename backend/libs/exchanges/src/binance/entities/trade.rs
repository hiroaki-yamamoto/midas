use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TradeSubRequestInner {
  pub id: u32,
  pub params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "method")]
pub(crate) enum TradeSubRequest {
  #[serde(rename = "SUBSCRIBE")]
  Subscribe(TradeSubRequestInner),
  #[serde(rename = "UNSUBSCRIBE")]
  Unsubscribe(TradeSubRequestInner),
}

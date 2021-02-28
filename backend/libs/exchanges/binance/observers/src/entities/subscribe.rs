use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeRequestInner {
  pub id: u64,
  pub params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "method")]
pub enum SubscribeRequest {
  #[serde(rename = "SUBSCRIBE")]
  Subscribe(SubscribeRequestInner),
  #[serde(rename = "UNSUBSCRIBE")]
  Unsubscribe(SubscribeRequestInner),
}

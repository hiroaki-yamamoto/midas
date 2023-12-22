use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeRequestInner {
  pub id: String,
  pub params: Vec<String>,
}

impl SubscribeRequestInner {
  pub fn into_subscribe(&self) -> SubscribeRequest {
    return SubscribeRequest::Subscribe(self.clone());
  }

  pub fn into_unsubscribe(&self) -> SubscribeRequest {
    return SubscribeRequest::Unsubscribe(self.clone());
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "method")]
pub enum SubscribeRequest {
  #[serde(rename = "SUBSCRIBE")]
  Subscribe(SubscribeRequestInner),
  #[serde(rename = "UNSUBSCRIBE")]
  Unsubscribe(SubscribeRequestInner),
}

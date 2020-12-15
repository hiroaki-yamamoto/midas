use ::mongodb::bson::oid::ObjectId;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIKey {
  pub id: ObjectId,
  pub exchange: String,
  pub label: String,
  pub pub_key: String,
  pub prv_key: String,
}

impl APIKey {
  pub fn new(
    id: ObjectId,
    exchange: String,
    label: String,
    pub_key: String,
    prv_key: String,
  ) -> Self {
    return Self {
      id,
      exchange,
      label,
      pub_key,
      prv_key,
    };
  }
}

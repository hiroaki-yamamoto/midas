use ::mongodb::bson::oid::{ObjectId, Result};
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIKey<T> {
  pub id: T,
  pub exchange: String,
  pub label: String,
  pub pub_key: String,
  pub prv_key: String,
}

impl<T> APIKey<T> {
  pub fn new(
    id: T,
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

impl From<APIKey<ObjectId>> for APIKey<String> {
  fn from(value: APIKey<ObjectId>) -> Self {
    return Self {
      id: value.id.to_hex(),
      exchange: value.exchange,
      label: value.label,
      pub_key: value.pub_key,
      prv_key: value.prv_key,
    };
  }
}

impl From<APIKey<String>> for Result<APIKey<ObjectId>> {
  fn from(value: APIKey<String>) -> Self {
    return Ok(APIKey {
      id: ObjectId::with_string(&value.id)?,
      exchange: value.exchange,
      label: value.label,
      pub_key: value.pub_key,
      prv_key: value.prv_key,
    });
  }
}

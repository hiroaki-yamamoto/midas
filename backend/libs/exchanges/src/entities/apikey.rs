use ::mongodb::bson::oid::ObjectId;
use ::num::traits::FromPrimitive;
use ::serde::{Deserialize, Serialize};

use ::rpc::entities::Exchanges;
use ::rpc::keychain::ApiKey as RPCAPIKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIKey {
  #[serde(default)]
  pub id: ObjectId,
  #[serde(default)]
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

impl From<APIKey> for Result<RPCAPIKey, String> {
  fn from(value: APIKey) -> Self {
    return Ok(RPCAPIKey {
      id: value.id.to_hex(),
      exchange: value.exchange.parse::<Exchanges>()?.into(),
      label: value.label,
      pub_key: value.pub_key,
      prv_key: value.prv_key,
    });
  }
}

impl From<RPCAPIKey> for APIKey {
  fn from(value: RPCAPIKey) -> Self {
    return APIKey {
      id: ObjectId::with_string(&value.id).unwrap_or(ObjectId::new()),
      exchange: FromPrimitive::from_i32(value.exchange)
        .map(|exc: Exchanges| exc.as_string())
        .unwrap_or(String::default()),
      label: value.label,
      pub_key: value.pub_key,
      prv_key: value.prv_key,
    };
  }
}

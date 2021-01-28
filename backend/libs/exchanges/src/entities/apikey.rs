use ::mongodb::bson::oid::ObjectId;
use ::num::traits::FromPrimitive;
use ::serde::{Deserialize, Serialize};

use ::rpc::entities::Exchanges;
use ::rpc::keychain::ApiKey as RPCAPIKey;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct APIKey {
  #[serde(default, rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(default)]
  pub exchange: String,
  pub label: String,
  pub pub_key: String,
  pub prv_key: String,
}

impl From<APIKey> for Result<RPCAPIKey, String> {
  fn from(value: APIKey) -> Self {
    return Ok(RPCAPIKey {
      id: value.id.map(|oid| oid.to_hex()).unwrap_or(String::from("")),
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
      id: ObjectId::with_string(&value.id).ok(),
      exchange: FromPrimitive::from_i32(value.exchange)
        .map(|exc: Exchanges| exc.as_string())
        .unwrap_or(String::default()),
      label: value.label,
      pub_key: value.pub_key,
      prv_key: value.prv_key,
    };
  }
}

use ::bson::oid::ObjectId;
use ::num_traits::FromPrimitive;
use ::serde::{Deserialize, Serialize};

use ::rpc::entities::Exchanges;
use ::rpc::keychain::ApiKey as RPCAPIKey;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct APIKeyInner {
  #[serde(default, rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub label: String,
  pub pub_key: String,
  pub prv_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "exchange", rename_all = "camelCase")]
pub enum APIKey {
  Binance(APIKeyInner),
  Unknown(APIKeyInner),
}

impl APIKey {
  pub fn inner(&self) -> &APIKeyInner {
    match self {
      APIKey::Binance(inner) => inner,
      APIKey::Unknown(inner) => inner,
    }
  }
  pub fn inner_mut(&mut self) -> &mut APIKeyInner {
    match self {
      APIKey::Binance(inner) => inner,
      APIKey::Unknown(inner) => inner,
    }
  }
}

impl From<APIKey> for Exchanges {
  fn from(v: APIKey) -> Self {
    match v {
      APIKey::Binance(_) => Self::Binance,
      APIKey::Unknown(_) => Self::Unknown,
    }
  }
}

impl From<APIKey> for Result<RPCAPIKey, String> {
  fn from(value: APIKey) -> Self {
    let inner = value.inner().clone();
    let exchange: Exchanges = value.into();
    return Ok(RPCAPIKey {
      id: inner.id.map(|oid| oid.to_hex()).unwrap_or(String::from("")),
      exchange: exchange.into(),
      label: inner.label,
      pub_key: inner.pub_key,
      prv_key: inner.prv_key,
    });
  }
}

impl From<RPCAPIKey> for APIKey {
  fn from(value: RPCAPIKey) -> Self {
    let exchange: Exchanges =
      FromPrimitive::from_i32(value.exchange).unwrap_or(Exchanges::Unknown);
    let exchange: APIKey = match exchange {
      Exchanges::Binance => APIKey::Binance(APIKeyInner {
        id: ObjectId::with_string(&value.id).ok(),
        label: value.label,
        pub_key: value.pub_key,
        prv_key: value.prv_key,
      }),
      Exchanges::Unknown => APIKey::Unknown(APIKeyInner {
        id: ObjectId::with_string(&value.id).ok(),
        label: value.label,
        pub_key: value.pub_key,
        prv_key: value.prv_key,
      }),
    };
    return exchange;
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum APIKeyEvent {
  Add(APIKey),
  Remove(APIKey),
}

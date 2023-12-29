use ::std::convert::TryFrom;

use ::bson::oid::ObjectId;
use ::serde::{Deserialize, Serialize};

use ::errors::ParseError;

use ::rpc::api_key::ApiKey as RPCAPIKey;
use ::rpc::exchanges::Exchanges;

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
}

impl APIKey {
  pub fn inner(&self) -> &APIKeyInner {
    match self {
      APIKey::Binance(inner) => inner,
    }
  }
  pub fn inner_mut(&mut self) -> &mut APIKeyInner {
    match self {
      APIKey::Binance(inner) => inner,
    }
  }
}

impl From<APIKey> for Exchanges {
  fn from(v: APIKey) -> Self {
    match v {
      APIKey::Binance(_) => Self::Binance,
    }
  }
}

impl From<APIKey> for Result<RPCAPIKey, String> {
  fn from(value: APIKey) -> Self {
    let inner = value.inner().clone();
    let exchange: Exchanges = value.into();
    return Ok(RPCAPIKey {
      id: inner.id.map(|oid| oid.to_hex()),
      exchange: exchange.into(),
      label: inner.label,
      pub_key: inner.pub_key,
      prv_key: inner.prv_key,
    });
  }
}

impl TryFrom<RPCAPIKey> for APIKey {
  type Error = ParseError;
  fn try_from(value: RPCAPIKey) -> Result<Self, Self::Error> {
    let exchange: APIKey = match value.exchange.as_ref() {
      Exchanges::Binance => APIKey::Binance(APIKeyInner {
        id: value.id.map(|id| ObjectId::parse_str(id).ok()).flatten(),
        label: value.label,
        pub_key: value.pub_key,
        prv_key: value.prv_key,
      }),
    };
    return Ok(exchange);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum APIKeyEvent {
  Add(APIKey),
  Remove(APIKey),
}

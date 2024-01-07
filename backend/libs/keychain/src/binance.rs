use ::errors::KeyChainResult;
use ::reqwest::header::{HeaderName, HeaderValue};
use ::rpc::exchanges::Exchanges;
use ::types::chrono::Utc;

use crate::entities::APIKey;
use crate::interfaces::{IHeaderSigner, IQueryStringSigner};

pub struct APIKeySigner;

impl APIKeySigner {
  pub fn new() -> Self {
    return Self {};
  }
}

impl IQueryStringSigner for APIKeySigner {
  fn append_sign(&self, key: &APIKey, qs: &str) -> String {
    let now = Utc::now().timestamp_millis();
    let qs = format!("{}&timestamp={}", qs, now);
    let sign = key.sign(Exchanges::Binance, &qs);
    return format!("{}&signature={}", qs, sign);
  }
}

impl IHeaderSigner for APIKeySigner {
  fn append_sign(
    &self,
    key: &APIKey,
    header: &mut reqwest::header::HeaderMap,
  ) -> KeyChainResult<()> {
    let name: HeaderName = "x-mbx-apikey".parse()?;
    let value: HeaderValue = key.inner().pub_key.parse()?;
    header.append(name, value);
    return Ok(());
  }
}

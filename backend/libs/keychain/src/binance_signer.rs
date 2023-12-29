use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::bytes::Bytes;
use ::mongodb::bson::oid::ObjectId;
use ::ring::hmac;

use ::errors::{ObjectNotFound, SignerResult};
use ::rpc::exchanges::Exchanges;

use crate::interfaces::{IKeyChain, ISigner};

pub struct Signer {
  keychain: Arc<dyn IKeyChain + Send + Sync>,
}

impl Signer {
  pub fn new(keychain: Arc<dyn IKeyChain + Send + Sync>) -> Self {
    return Self { keychain };
  }
}

#[async_trait]
impl ISigner for Signer {
  async fn sign(
    &self,
    api_key_id: ObjectId,
    body: String,
  ) -> SignerResult<String> {
    let key = self
      .keychain
      .get(Exchanges::Binance, api_key_id)
      .await?
      .ok_or(ObjectNotFound::new("API KeyPair".to_string()))?;
    let key = key.inner();
    let prv_key = hmac::Key::new(hmac::HMAC_SHA256, key.prv_key.as_bytes());
    let tag = hmac::sign(&prv_key, body.as_bytes());
    let signature = Bytes::copy_from_slice(tag.as_ref());
    return Ok(format!("{:x}", signature));
  }
}

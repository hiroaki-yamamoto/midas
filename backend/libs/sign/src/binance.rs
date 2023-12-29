use ::bytes::Bytes;
use ::ring::hmac;

use ::entities::APIKeyInner;

use crate::interface::ISigner;

#[derive(Debug)]
pub struct Signer {
  prv_key: hmac::Key,
}

impl Signer {
  pub fn new(api_key: APIKeyInner) -> Self {
    return Self {
      prv_key: hmac::Key::new(hmac::HMAC_SHA256, api_key.prv_key.as_bytes()),
    };
  }
}

impl ISigner for Signer {
  fn sign(&self, body: String) -> String {
    let tag = hmac::sign(&self.prv_key, body.as_bytes());
    let signature = Bytes::copy_from_slice(tag.as_ref());
    return format!("{:x}", signature);
  }
}

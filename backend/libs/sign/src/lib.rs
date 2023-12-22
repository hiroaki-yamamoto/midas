use ::bytes::Bytes;
use ::ring::hmac;

pub trait Sign {
  fn get_secret_key(&self, prv_key: String) -> hmac::Key;
  fn sign(&self, body: String, prv_key: String) -> String {
    let secret = self.get_secret_key(prv_key);
    let tag = hmac::sign(&secret, body.as_bytes());
    let signature = Bytes::copy_from_slice(tag.as_ref());
    return format!("{:x}", signature);
  }
}

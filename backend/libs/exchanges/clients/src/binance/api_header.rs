use ::std::convert::TryFrom;

use ::async_trait::async_trait;
use ::mongodb::bson::oid::ObjectId;
use ::reqwest::header::{HeaderMap, HeaderName};
use ::rpc::entities::Exchanges;

use ::entities::APIKeyInner;
use ::errors::{APIHeaderResult, ObjectNotFound};
use ::keychain::KeyChain;

#[async_trait]
pub trait FindKey {
  fn get_keychain(&self) -> &KeyChain;
  async fn get_api_key(&self, id: ObjectId) -> APIHeaderResult<APIKeyInner> {
    let key = self.get_keychain().get(Exchanges::Binance, id).await?;
    let key = key.ok_or(ObjectNotFound::new("API KeyPair".to_string()))?;
    return Ok(key.inner().clone());
  }
}

pub trait APIHeader {
  fn get_pub_header(
    &self,
    api_key: &APIKeyInner,
  ) -> APIHeaderResult<HeaderMap> {
    return self.pub_header_from_str(&api_key.pub_key);
  }

  fn pub_header_from_str<T>(&self, pub_api: T) -> APIHeaderResult<HeaderMap>
  where
    T: AsRef<str>,
  {
    let mut header = HeaderMap::new();
    header.insert(
      HeaderName::try_from("x-mbx-apikey")?,
      pub_api.as_ref().parse()?,
    );
    return Ok(header);
  }
}

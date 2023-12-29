use ::std::convert::TryFrom;

use ::reqwest::header::{HeaderMap, HeaderName};

use ::entities::APIKeyInner;
use ::errors::APIHeaderResult;

pub trait APIHeader {
  fn get_pub_header(
    &self,
    api_key: &APIKeyInner,
  ) -> APIHeaderResult<HeaderMap> {
    return self.pub_header_from_str(&api_key.pub_key);
  }

  fn pub_header_from_str(&self, pub_api: &str) -> APIHeaderResult<HeaderMap> {
    let mut header = HeaderMap::new();
    header.insert(HeaderName::try_from("x-mbx-apikey")?, pub_api.parse()?);
    return Ok(header);
  }
}

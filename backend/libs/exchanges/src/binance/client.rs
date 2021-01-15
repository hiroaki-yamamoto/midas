use ::reqwest::{Client as Req, header};

use ::types::GenericResult;

pub trait PubClient {
  fn get_client(&self, pub_key: String) -> GenericResult<Req> {
    let def_header = header::HeaderMap::new();
    def_header.insert(
      header::HeaderName::from_static("x-mbx-apikey"),
      header::HeaderValue::from_static(pub_key.as_str())
    );
    return Ok(Req::builder().default_headers(def_header).build()?);
  }
}

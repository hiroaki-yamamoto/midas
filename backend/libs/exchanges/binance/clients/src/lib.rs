pub mod constants;

pub use ::reqwest;
use ::reqwest::{header, Client as Req};

use ::types::ThreadSafeResult;

pub trait PubClient {
  fn get_client<T>(&self, pub_key: T) -> ThreadSafeResult<Req>
  where
    T: AsRef<str>,
  {
    let mut def_header = header::HeaderMap::new();
    def_header.insert(
      header::HeaderName::from_static("x-mbx-apikey"),
      header::HeaderValue::from_str(pub_key.as_ref())?,
    );
    return Ok(Req::builder().default_headers(def_header).build()?);
  }
}

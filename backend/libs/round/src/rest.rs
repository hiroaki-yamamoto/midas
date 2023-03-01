use ::std::time::Duration;

use ::serde::Serialize;

use ::errors::{HTTPResult, MaximumAttemptExceeded};
use ::log::warn;
use ::reqwest::{
  header::HeaderMap, Client, Method, Response as Resp, Result as ReqRes, Url,
};

macro_rules! method {
  ($method_name: ident, $method: path) => {
    pub async fn $method_name<T>(
      &mut self,
      headers: Option<HeaderMap>,
      query: Option<T>,
    ) -> ::errors::HTTPResult<Resp>
    where
      T: ::serde::Serialize,
    {
      return self.request($method, headers, query).await;
    }
  };
}

#[derive(Debug, Clone)]
pub struct RestClient {
  urls: Vec<Url>,
  state: usize,
  cli: Client,
}

impl RestClient {
  pub fn new(
    urls: Vec<Url>,
    con_timeout: Duration,
    req_timeout: Duration,
  ) -> ReqRes<Self> {
    return Ok(Self {
      urls,
      cli: Client::builder()
        .connect_timeout(con_timeout)
        .timeout(req_timeout)
        .build()?,
      state: 0,
    });
  }

  async fn request<T>(
    &mut self,
    method: Method,
    headers: Option<HeaderMap>,
    query: Option<T>,
  ) -> HTTPResult<Resp>
  where
    T: Serialize,
  {
    for c in 0..self.urls.len() {
      let index = (self.state + c) % self.urls.len();
      let url = self.urls[index].clone();
      let mut req = self.cli.request(method.clone(), url.clone());
      if let Some(query) = &query {
        req = req.query(query);
      }
      if let Some(headers) = headers.as_ref() {
        req = req.headers(headers.clone());
      }
      let resp = req.send().await;
      if let Err(e) = &resp {
        if e.is_timeout() {
          warn!("Connection Timeout (Trying the next): {}", url);
          continue;
        }
      }
      self.state = index;
      return Ok(resp?);
    }
    return Err(MaximumAttemptExceeded {}.into());
  }

  method!(get, Method::GET);
  method!(post, Method::POST);
  method!(delete, Method::DELETE);
  method!(put, Method::PUT);

  pub fn get_current_url(&self) -> &Url {
    return &self.urls[self.state];
  }
}

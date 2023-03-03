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

#[derive(Debug)]
pub struct RestClient {
  state: usize,
  urls: Vec<Url>,
  cli: Client,
}

impl Clone for RestClient {
  fn clone(&self) -> Self {
    return Self {
      urls: self.urls.clone(),
      state: (self.state + 1) % self.urls.len(),
      cli: self.cli.clone(),
    };
  }
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
      return Ok(resp?.error_for_status()?);
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

  pub fn get_state(&self) -> usize {
    return self.state;
  }

  pub fn set_state(&mut self, state: usize) {
    self.state = state % self.urls.len();
  }
}

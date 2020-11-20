mod errors;

use ::warp::{Filter, Reply};

use self::errors::CSRFCheckFailed;
pub use ::http::Method;

#[derive(Debug, Clone)]
pub struct CSRFOption {
  cookie_name: &'static str,
  header_name: &'static str,
}

impl Default for CSRFOption {
  fn default() -> Self {
    return Self {
      cookie_name: "XSRF-TOKEN",
      header_name: "X-XSRF-TOKEN",
    };
  }
}

impl CSRFOption {
  pub fn builder() -> Self {
    return Self::default();
  }
  pub fn cookie_name(mut self, cookie_name: &'static str) -> Self {
    self.cookie_name = cookie_name;
    return self;
  }
  pub fn header_name(mut self, header_name: &'static str) -> Self {
    self.header_name = header_name;
    return self;
  }
}

#[derive(Debug, Clone)]
pub struct CSRF {
  opt: CSRFOption,
}

impl CSRF {
  pub fn new(opt: CSRFOption) -> Self {
    return Self { opt };
  }
  pub fn protect(
    &self,
  ) -> impl Filter<Extract = (), Error = ::warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static {
    return ::warp::any()
      .and(::warp::filters::cookie::optional(self.opt.cookie_name))
      .and(::warp::filters::header::optional(self.opt.header_name))
      .and_then(
        |cookie: Option<String>, header: Option<String>| async move {
          if cookie.is_none() || header.is_none() {
            return Err(::warp::reject::custom(CSRFCheckFailed::new(
              "Either cookie or header is none.".to_string(),
              format!("{:?}", cookie),
              format!("{:?}", header),
            )));
          }
          let (cookie, header) = (cookie.unwrap(), header.unwrap());
          if cookie == header {
            return Ok(());
          }
          return Err(::warp::reject::custom(CSRFCheckFailed::new(
            "CSRF Token Mismatch".to_string(),
            cookie,
            header,
          )));
        },
      )
      .untuple_one();
  }

  pub fn generate_cookie<F, Resp>(
    &self,
    filter: F,
  ) -> impl Filter<Extract = (impl Reply,), Error = ::std::convert::Infallible>
       + Clone
       + Send
       + Sync
       + 'static
  where
    F: Filter<Extract = (Resp,), Error = ::std::convert::Infallible>
      + Clone
      + Send
      + Sync
      + 'static,
    Resp: Reply,
  {
    return ::warp::any().and(filter).map(|reply: Resp| {});
  }
}

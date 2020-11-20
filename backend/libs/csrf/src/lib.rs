mod errors;

use ::warp::{Filter, Reply};

use self::errors::CSRFCheckFailed;
pub use ::http::Method;

#[derive(Debug, Clone)]
pub struct CSRFOption {
  cookie_name: String,
  header_name: String,
  verify_methods: Vec<Method>,
}

impl Default for CSRFOption {
  fn default() -> Self {
    return Self {
      cookie_name: "XSRF-TOKEN".to_string(),
      header_name: "X-XSRF-TOKEN".to_string(),
      verify_methods: vec![
        Method::POST,
        Method::DELETE,
        Method::PUT,
        Method::PATCH,
      ],
    };
  }
}

impl CSRFOption {
  pub fn builder() -> Self {
    return Self::default();
  }
  pub fn cookie_name(mut self, cookie_name: String) -> Self {
    self.cookie_name = cookie_name;
    return self;
  }
  pub fn header_name(mut self, header_name: String) -> Self {
    self.header_name = header_name;
    return self;
  }
  pub fn verify_methods(mut self, verify_methods: Vec<Method>) -> Self {
    self.verify_methods = verify_methods;
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
  pub fn protect<F, R1>(
    &self,
    filter: F,
  ) -> impl Filter<Extract = (R1,), Error = ::warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static
  where
    F: Filter<Extract = (R1,), Error = ::std::convert::Infallible>
      + Clone
      + Send
      + Sync
      + 'static,
    F::Extract: Reply,
  {
    let cookie_name = self.opt.cookie_name;
    let header_name = self.opt.header_name;
    let methods = self.opt.verify_methods;

    return ::warp::method()
      .and(::warp::filters::cookie::optional(
        &cookie_name,
      ))
      .and(::warp::filters::header::optional(
        &header_name,
      ))
      .and_then(
        |method: Method, cookie: Option<String>, header: Option<String>| async move {
          let verify_methods = methods.clone();
          if !methods.contains(&method) {
            return Ok(());
          }
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
      .untuple_one()
      .and(filter);
  }
}

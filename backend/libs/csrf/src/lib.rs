mod errors;

use ::cookie::CookieBuilder;
use ::hyper::header::SET_COOKIE;
use ::rand::distributions::Alphanumeric;
use ::rand::{thread_rng, Rng};
use ::time::Duration as TimeDuration;
use ::warp::http::Method;
use ::warp::reply;
use ::warp::{Filter, Rejection, Reply};

pub use self::errors::CSRFCheckFailed;

#[derive(Debug, Clone)]
pub struct CSRFOption {
  cookie_name: &'static str,
  header_name: &'static str,
  verify_methods: Vec<Method>,
}

impl Default for CSRFOption {
  fn default() -> Self {
    return Self {
      cookie_name: "XSRF-TOKEN",
      header_name: "X-XSRF-TOKEN",
      verify_methods: vec![
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
      ],
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
  pub fn verify_methods(mut self, methods: Vec<Method>) -> Self {
    self.verify_methods = methods;
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
    let verify_methods = self.opt.verify_methods.clone();
    return ::warp::method()
      .and(::warp::filters::cookie::optional(self.opt.cookie_name))
      .and(::warp::filters::header::optional(self.opt.header_name))
      .map(
        move |method: Method,
              cookie: Option<String>,
              header: Option<String>| {
          if !verify_methods.contains(&method) {
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
      .and_then(|res: Result<(), Rejection>| async { return res })
      .untuple_one();
  }

  pub fn generate_cookie<F, Resp>(
    &self,
    methods: Vec<Method>,
    filter: F,
  ) -> impl Filter<Extract = (impl Reply,), Error = ::warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static
  where
    F: Filter<Extract = (Resp,), Error = ::warp::Rejection>
      + Clone
      + Send
      + Sync
      + 'static,
    Resp: Reply,
  {
    let cookie_name = self.opt.cookie_name.clone();
    return ::warp::method()
      .and(::warp::cookie::optional(&cookie_name))
      .and(filter)
      .map(
        move |method: Method, req_cookie: Option<String>, resp: Resp| {
          if !methods.contains(&method) {
            return resp.into_response();
          }
          let value: String =
            thread_rng().sample_iter(&Alphanumeric).take(50).collect();
          let cookie = CookieBuilder::new(cookie_name, value)
            .max_age(TimeDuration::new(3600, 0))
            .secure(true)
            .finish();
          match req_cookie {
            None => {
              return reply::with_header(
                resp,
                SET_COOKIE.as_str(),
                cookie.to_string(),
              )
              .into_response();
            }
            Some(_) => return resp.into_response(),
          };
        },
      );
  }
}

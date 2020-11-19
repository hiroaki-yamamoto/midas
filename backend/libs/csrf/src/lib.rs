mod errors;

use ::warp::wrap_fn;
use ::warp::{Filter, Reply};

use self::errors::CSRFCheckFailed;
pub use ::http::Method;

#[derive(Debug)]
pub struct CSRFOption<'t> {
  cookie_name: &'t str,
  header_name: &'t str,
  verify_methods: Vec<Method>,
}

impl<'t> Default for CSRFOption<'t> {
  fn default() -> Self {
    return Self {
      cookie_name: "XSRF-TOKEN",
      header_name: "X-XSRF-TOKEN",
      verify_methods: vec![
        Method::POST,
        Method::DELETE,
        Method::PUT,
        Method::PATCH,
      ],
    };
  }
}

impl<'t> CSRFOption<'t> {
  pub fn builder() -> Self {
    return Self::default();
  }
  pub fn cookie_name(mut self, cookie_name: &'t str) -> Self {
    self.cookie_name = cookie_name;
    return self;
  }
  pub fn header_name(mut self, header_name: &'t str) -> Self {
    self.header_name = header_name;
    return self;
  }
  pub fn verify_methods(mut self, verify_methods: Vec<Method>) -> Self {
    self.verify_methods = verify_methods;
    return self;
  }
}

fn protect_csrf<F, S, T, R>(option: CSRFOption) -> impl Fn(F) -> S
where
  F: Filter<Extract = (T,), Error = std::convert::Infallible>
    + Clone
    + Send
    + Sync
    + 'static,
  F::Extract: Reply,
  S: Filter<Extract = (R,)> + Clone + Send + Sync + 'static,
  S::Extract: Reply,
{
  let mut csrf_filter = ::warp::method()
    .and(::warp::filters::cookie::optional(option.cookie_name))
    .and(::warp::filters::header::optional(option.header_name))
    .and_then(
      |method: Method, cookie: Option<String>, header: Option<String>| async {
        if !option.verify_methods.contains(&method) {
          return Ok((method, cookie, header));
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
          return Ok((method, Some(cookie), Some(header)));
        }
        return Err(::warp::reject::custom(CSRFCheckFailed::new(
          "CSRF Token Mismatch".to_string(),
          cookie,
          header,
        )));
      },
    )
    .untuple_one();
  return move |filter: F| {
    return csrf_filter.and(filter).map(
      move |method: Method,
            cookie: Option<String>,
            header: Option<String>,
            reply: F::Extract| {
        return filter;
      },
    );
  };
}

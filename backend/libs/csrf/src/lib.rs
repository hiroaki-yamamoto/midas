mod errors;
use ::std::str::FromStr;

use ::warp::wrap_fn;
use ::warp::{Filter, Reply};

use self::errors::MethodParseError;

pub enum Methods {
  DELETE,
  GET,
  HEAD,
  OPTIONS,
  PATCH,
  POST,
  PUT,
}

impl FromStr for Methods {
  type Err = MethodParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    return Ok(match s.to_lowercase().as_str() {
      "post" => Self::POST,
      "delete" => Self::DELETE,
      "put" => Self::PUT,
      "patch" => Self::PATCH,
      "get" => Self::GET,
      "head" => Self::HEAD,
      "options" => Self::OPTIONS,
      method => return Err(MethodParseError::new(method.to_string())),
    });
  }
}

#[derive(Debug)]
pub struct CSRFOption<'t> {
  cookie_name: &'t str,
  header_name: &'t str,
  verify_methods: Vec<&'t str>,
}

impl<'t> Default for CSRFOption<'t> {
  fn default() -> Self {
    return Self {
      cookie_name: "XSRF-TOKEN",
      header_name: "X-XSRF-TOKEN",
      verify_methods: vec!["POST", "DELETE", "PUT", "PATCH"],
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
  pub fn verify_methods(mut self, verify_methods: Vec<&'t str>) -> Self {
    self.verify_methods = verify_methods;
    return self;
  }
}

fn protect_csrf<A, F, S, T, R>(
  option: CSRFOption,
) -> Result<A, MethodParseError>
where
  A: Fn(F) -> S,
  F: Filter<Extract = (T,), Error = std::convert::Infallible>
    + Clone
    + Send
    + Sync
    + 'static,
  S: Filter<Extract = (U,)> + Clone + Send + Sync + 'static,
  R: Reply,
{
  let mut filter = ::warp::method().map(|method| {
    if !option.verify_methods.contains(method) {
      return;
    }
  });
  return mvoe || {
    return filter;
  };
}

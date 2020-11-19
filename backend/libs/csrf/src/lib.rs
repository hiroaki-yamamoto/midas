use ::warp::wrap_fn;

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

fn protect_csrf() {}

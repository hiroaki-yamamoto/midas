use ::std::fmt::{Display, Error, Formatter, Result as FmtResult};

use ::serde::{Deserialize, Serialize};
use ::serde_json::to_string;
use ::warp::reject::Reject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSRFCheckFailed {
  pub reason: String,
  pub cookie_value: String,
  pub header_value: String,
}

impl CSRFCheckFailed {
  pub fn new(
    reason: String,
    cookie_value: String,
    header_value: String,
  ) -> Self {
    return Self {
      reason,
      cookie_value,
      header_value,
    };
  }
}

impl Display for CSRFCheckFailed {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let json = to_string(self).map_err(|_| Error {})?;
    return write!(f, "{}", json);
  }
}

impl Reject for CSRFCheckFailed {}

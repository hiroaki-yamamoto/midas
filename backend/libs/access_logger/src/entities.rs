use ::std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use warp::log::Info;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
  pub remote_addr: Option<String>,
  pub method: String,
  pub path: String,
  pub version: String,
  pub status: u16,
  pub referer: Option<String>,
  pub user_agent: Option<String>,
  pub elapsed: String,
  pub host: Option<String>,
  pub headers: String,
}

impl From<Info<'_>> for Log {
  fn from(value: Info<'_>) -> Self {
    return Self {
      remote_addr: value.remote_addr().map(|addr| format!("{}", addr)),
      method: value.method().to_string(),
      path: value.path().to_string(),
      version: format!("{:?}", value.version()),
      status: value.status().as_u16(),
      referer: value.referer().map(|v| v.to_string()),
      user_agent: value.user_agent().map(|v| v.to_string()),
      elapsed: format!("{:?}", value.elapsed()),
      host: value.host().map(|v| v.to_string()),
      headers: format!("{:?}", value.request_headers()),
    };
  }
}

impl Display for Log {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let serialized = to_string(&self).unwrap_or(format!("{:?}", self));
    return write!(f, "{}", serialized);
  }
}

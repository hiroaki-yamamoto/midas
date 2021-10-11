use ::err_derive::Error;
use ::url::Url;

#[derive(Debug, Clone, Error)]
#[error(
  display = "Status Failue (code: {}, text: {}, url: {:?})",
  code,
  text,
  url
)]
pub struct StatusFailure {
  pub url: Option<Url>,
  pub code: u16,
  pub text: String,
}

impl StatusFailure {
  pub fn new(url: Option<Url>, code: u16, text: String) -> Self {
    return Self { url, code, text };
  }
}

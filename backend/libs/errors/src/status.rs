use ::err_derive::Error;
use ::url::Url;

#[derive(Debug, Clone, Error)]
#[error(
  display = "Status Failue (code: {}, text: {}, url: {})",
  code,
  text,
  url
)]
pub struct StatusFailure {
  pub url: Url,
  pub code: u16,
  pub text: String,
}

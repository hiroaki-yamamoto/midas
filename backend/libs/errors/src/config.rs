use ::std::io::Error as IOErr;

use ::err_derive::Error;
use ::reqwest::Error as ReqwestErr;
use ::serde_yaml::Error as YamlErr;

use crate::MaximumAttemptExceeded;

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error(display = "{}", _0)]
  MaximumAttemptExceeded(#[source] MaximumAttemptExceeded),
  #[error(display = "IOErr: {}", _0)]
  IOErr(#[source] IOErr),
  #[error(display = "YAML Decode/Encode Error: {}", _0)]
  YamlErr(#[source] YamlErr),
  #[error(display = "Requwest Err: {}", _0)]
  ReqwestErr(#[source] ReqwestErr),
}

pub type ConfigResult<T> = Result<T, ConfigError>;

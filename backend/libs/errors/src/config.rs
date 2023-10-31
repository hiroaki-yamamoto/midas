use ::std::io::Error as IOErr;

use ::async_nats::ConnectError;
use ::err_derive::Error;
use ::redis::RedisError;
use ::reqwest::Error as ReqwestErr;
use ::serde_yaml::Error as YamlErr;

use crate::MaximumAttemptExceeded;

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error(display = "{}", _0)]
  MaximumAttemptExceeded(#[source] MaximumAttemptExceeded),
  #[error(display = "KVS Error: {}", _0)]
  RedisError(#[source] RedisError),
  #[error(display = "IOErr: {}", _0)]
  IOErr(#[source] IOErr),
  #[error(display = "YAML Decode/Encode Error: {}", _0)]
  YamlErr(#[source] YamlErr),
  #[error(display = "Requwest Err: {}", _0)]
  ReqwestErr(#[source] ReqwestErr),
  #[error(display = "NATS Connection Err: {}", _0)]
  ConnectError(#[source] ConnectError),
}

pub type ConfigResult<T> = Result<T, ConfigError>;

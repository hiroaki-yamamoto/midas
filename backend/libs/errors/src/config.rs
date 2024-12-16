use ::std::io::Error as IOErr;

use ::async_nats::ConnectError;
use ::redis::RedisError;
use ::reqwest::Error as ReqwestErr;
use ::serde_yaml::Error as YamlErr;
use ::thiserror::Error;

use crate::MaximumAttemptExceeded;

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("{}", _0)]
  MaximumAttemptExceeded(#[from] MaximumAttemptExceeded),
  #[error("KVS Error: {}", _0)]
  RedisError(#[from] RedisError),
  #[error("IOErr: {}", _0)]
  IOErr(#[from] IOErr),
  #[error("YAML Decode/Encode Error: {}", _0)]
  YamlErr(#[from] YamlErr),
  #[error("Requwest Err: {}", _0)]
  ReqwestErr(#[from] ReqwestErr),
  #[error("NATS Connection Err: {}", _0)]
  ConnectError(#[from] ConnectError),
}

pub type ConfigResult<T> = Result<T, ConfigError>;

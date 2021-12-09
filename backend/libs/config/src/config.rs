use ::std::error::Error;
use ::std::fs::{read_to_string, File};
use ::std::io::Read;
use ::std::time::Duration;

use ::reqwest::{Certificate, Client};
use ::serde::Deserialize;
use ::serde_yaml::Result as YaMLResult;
use ::slog::Logger;
use ::slog_builder::{build_debug, build_json};

use ::types::{GenericResult, ThreadSafeResult};

use ::errors::MaximumAttemptExceeded;
use ::redis::Connection;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TLS {
  #[serde(rename = "privateKey")]
  pub prv_key: String,
  pub cert: String,
  pub ca: String,
  pub root: String,
}

#[derive(Debug, Deserialize)]
pub struct ServiceAddresses {
  pub historical: String,
  pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub host: String,
  #[serde(rename = "dbURL")]
  pub db_url: String,
  #[serde(rename = "brokerURL")]
  pub broker_url: String,
  #[serde(rename = "redisURL")]
  pub redis_url: String,
  #[serde(default)]
  pub debug: bool,
  pub tls: TLS,
}

impl Config {
  pub fn redis(&self, logger: &Logger) -> ThreadSafeResult<Connection> {
    let cli = ::redis::Client::open(self.redis_url.clone())?;
    for _ in 0..10 {
      match cli.get_connection_with_timeout(Duration::from_secs(1)) {
        Ok(o) => return Ok(o),
        Err(e) => {
          ::slog::warn!(
            logger,
            "Failed to estanblish the connection to redis. Retrying.\
            (Reason: {:?})",
            e
          );
          continue;
        }
      }
    }
    return Err(Box::new(MaximumAttemptExceeded::default()));
  }
  pub fn from_stream<T>(st: T) -> YaMLResult<Self>
  where
    T: Read,
  {
    return ::serde_yaml::from_reader::<_, Self>(st);
  }

  pub fn from_fpath(path: Option<String>) -> GenericResult<Self> {
    let path = match path {
      None => String::from(super::constants::DEFAULT_CONFIG_PATH),
      Some(p) => p,
    };
    let f = File::open(path)?;
    return Ok(Self::from_stream(f)?);
  }

  pub fn build_slog(&self) -> Logger {
    return match self.debug {
      true => build_debug(),
      false => build_json(),
    };
  }

  pub fn build_rest_client(&self) -> GenericResult<Client> {
    let ca = Certificate::from_pem(read_to_string(&self.tls.ca)?.as_bytes())?;
    return Ok(Client::builder().add_root_certificate(ca).build()?);
  }
}

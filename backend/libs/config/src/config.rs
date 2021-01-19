use ::std::error::Error;
use ::std::fs::{read_to_string, File};
use ::std::io::Read;

use ::reqwest::{Certificate, Client, Identity};
use ::serde::Deserialize;
use ::slog::Logger;
use ::slog_atomic::AtomicSwitchCtrl;
use ::slog_builder::{build_debug, build_json};

use ::types::GenericResult;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TLS {
  #[serde(rename = "privateKey")]
  pub prv_key: String,
  pub cert: String,
  pub ca: String,
}

impl TLS {
  pub fn build_tls(&self) -> GenericResult<(Identity, Certificate)> {
    return Ok((
      Identity::from_pem(read_to_string(&self.prv_key)?.as_bytes())?,
      Certificate::from_pem(read_to_string(&self.cert)?.as_bytes())?,
    ));
  }
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
  #[serde(default)]
  pub debug: bool,
  pub tls: TLS,
}

impl Config {
  pub fn from_stream<T>(st: T) -> Result<Self, Box<dyn Error>>
  where
    T: Read,
  {
    let cfg: Self = ::serde_yaml::from_reader(st)?;
    return Ok(cfg);
  }

  pub fn from_fpath(path: Option<String>) -> Result<Self, Box<dyn Error>> {
    let path = match path {
      None => String::from(super::constants::DEFAULT_CONFIG_PATH),
      Some(p) => p,
    };
    let f = File::open(path)?;
    return Self::from_stream(f);
  }

  pub fn build_slog(&self) -> Logger {
    return match self.debug {
      true => build_debug(),
      false => build_json(),
    };
  }

  pub fn build_rest_client(&self) -> GenericResult<Client> {
    let (prv_key, root_cert) = self.tls.build_tls()?;
    return Ok(
      Client::builder()
        .add_root_certificate(root_cert)
        .identity(prv_key)
        .build()?,
    );
  }
}

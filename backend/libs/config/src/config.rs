use ::std::error::Error;
use ::std::fs::{read_to_string, File};
use ::std::io::Read;

use ::serde::Deserialize;
use ::tonic::transport::{Identity, ServerTlsConfig};

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
  pub fn load_server(&self) -> GenericResult<ServerTlsConfig> {
    let prv_key = read_to_string(&self.prv_key)?;
    let cert = read_to_string(&self.cert)?;
    let tlscfg =
      ServerTlsConfig::new().identity(Identity::from_pem(cert, prv_key));
    return Ok(tlscfg);
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
  pub service_addresses: ServiceAddresses,
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
}

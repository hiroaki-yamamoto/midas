use ::std::fs::File;
use ::std::error::Error;
use ::std::io::Read;
use ::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  #[serde(rename = "dbURL")]
  pub db_url: String,
  #[serde(rename = "brokerURL")]
  pub broker_url: String,
  #[serde(default)]
  pub debug: bool,
}

impl Config {
  pub fn from_stream<T>(st:T) -> Result<Self, Box<dyn Error>> where T: Read {
    let cfg: Self = ::serde_yaml::from_reader(st)?;
    return Ok(cfg);
  }

  pub fn from_fpath(path: String) -> Result<Self, Box<dyn Error>> {
    let f = File::open(path)?;
    return Self::from_stream(f);
  }
}

use ::std::fs::{read_to_string, File};
use ::std::io::Read;
use ::std::time::Duration;

use ::log::warn;
use ::mongodb::error::Result as DBResult;
use ::mongodb::{
  options::ClientOptions as DBCliOpt, Client as DBCli, Database as DB,
};
use ::reqwest::{Certificate, Client};
use ::serde::de::Error as SerdeError;
use ::serde::{Deserialize, Deserializer};
use ::serde_yaml::Result as YaMLResult;
use ::tokio::io::stdout;
use ::tokio::time::{sleep, timeout};
use structured_logger::{async_json::new_writer, Builder};

use ::errors::{ConfigError, ConfigResult, MaximumAttemptExceeded};
use ::kvs::redis::aio::MultiplexedConnection as RedisConnection;
use ::kvs::redis::Client as RedisClient;
use ::subscribe::nats::connect as nats_connect;
use ::subscribe::nats::Client as Nats;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TLS {
  #[serde(rename = "privateKey")]
  pub prv_key: String,
  pub cert: String,
  pub ca: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeObserverInitNodeNumbers {
  pub binance: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub host: String,
  #[serde(rename = "dbURL")]
  pub db_url: String,
  #[serde(rename = "brokerURL")]
  pub broker_url: String,
  #[serde(rename = "transpilerURL")]
  pub transpiler_url: String,
  #[serde(rename = "redisURL", deserialize_with = "Config::redis_client")]
  pub redis: RedisClient,
  pub tls: TLS,
}

impl Config {
  fn redis_client<'de, D>(de: D) -> Result<RedisClient, D::Error>
  where
    D: Deserializer<'de>,
  {
    let url: String = Deserialize::deserialize(de)?;
    return ::redis::Client::open(url).map_err(|e| SerdeError::custom(e));
  }

  pub async fn redis(&self) -> ConfigResult<RedisConnection> {
    for _ in 0..60 {
      match timeout(
        Duration::from_secs(1),
        self.redis.get_multiplexed_async_connection(),
      )
      .await
      {
        Ok(o) => return Ok(o?),
        Err(e) => {
          warn!(
            error: err = e;
            "Failed to estanblish the connection to redis. Retrying.",
          );
        }
      }
    }
    return Err(MaximumAttemptExceeded::default().into());
  }

  pub async fn db(&self) -> DBResult<DB> {
    let opt = DBCliOpt::parse(self.db_url.to_owned()).await?;
    let cli = DBCli::with_options(opt)?;
    return Ok(cli.database("midas"));
  }
  pub fn from_stream<T>(st: T) -> YaMLResult<Self>
  where
    T: Read,
  {
    return ::serde_yaml::from_reader::<_, Self>(st);
  }

  pub fn from_fpath(path: Option<String>) -> ConfigResult<Self> {
    let path = match path {
      None => String::from(super::constants::DEFAULT_CONFIG_PATH),
      Some(p) => p,
    };
    let f = File::open(path)?;
    return Ok(Self::from_stream(f)?);
  }

  pub fn init_logger(&self) {
    Builder::new()
      .with_default_writer(new_writer(stdout()))
      .init();
  }

  pub fn build_rest_client(&self) -> ConfigResult<Client> {
    let ca = Certificate::from_pem(read_to_string(&self.tls.ca)?.as_bytes())?;
    return Ok(Client::builder().add_root_certificate(ca).build()?);
  }

  pub async fn nats_cli(&self) -> ConfigResult<Nats> {
    let mut err = None;
    for count in 1..=30 {
      match nats_connect(&self.broker_url).await {
        Ok(broker) => return Ok(broker),
        Err(e) => {
          warn!(
            "Failed to establish the connection to nats: {}
              Retrying in 1 sec ({}/30)",
            e, count
          );
          err = Some(e);
        }
      }
      sleep(Duration::from_secs(1)).await;
    }
    return Err(
      err
        .map(|e| ConfigError::from(e))
        .unwrap_or(MaximumAttemptExceeded::default().into())
        .into(),
    );
  }
}

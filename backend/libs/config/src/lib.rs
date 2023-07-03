mod cmdargs;
mod config;
mod constants;

use std::future::Future;
use std::net::SocketAddr;

use ::clap::Parser;
use ::libc::{SIGINT, SIGTERM};

pub use self::constants::{
  CHAN_BUF_SIZE, DEFAULT_CONFIG_PATH, DEFAULT_RECONNECT_INTERVAL,
  NUM_OBJECTS_TO_FETCH,
};

pub use self::cmdargs::CmdArgs;
pub use self::config::Config;

use ::mongodb::Database;
use ::nats::jetstream::JetStream as Broker;
use ::tokio::signal::unix as signal;

pub async fn init<S, T>(func: T)
where
  T: Fn(Config, signal::Signal, Database, Broker, SocketAddr) -> S,
  S: Future<Output = ()>,
{
  let sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  cfg.init_logger();
  let db = cfg.db().await.unwrap();
  let broker = cfg.nats_cli().unwrap();
  let host: SocketAddr = cfg.host.parse().unwrap();
  func(cfg, sig, db, broker, host).await;
}

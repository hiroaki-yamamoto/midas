mod cmdargs;
mod config;
mod constants;

use std::future::Future;
use std::net::SocketAddr;

use ::clap::Parser;
use ::libc::{SIGINT, SIGTERM};
use ::tokio::join;

pub use self::constants::{
  CHAN_BUF_SIZE, DEFAULT_CONFIG_PATH, DEFAULT_RECONNECT_INTERVAL,
  NUM_OBJECTS_TO_FETCH,
};

pub use self::cmdargs::CmdArgs;
pub use self::config::Config;

use ::mongodb::Database;
use ::subscribe::natsJS::context::Context as NatsJS;
use ::tokio::signal::unix as signal;

pub async fn init<S, T>(func: T)
where
  T: FnOnce(Config, signal::Signal, Database, NatsJS, SocketAddr) -> S,
  S: Future<Output = ()>,
{
  let sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  cfg.init_logger();
  let (db, broker) = join!(cfg.db(), cfg.nats_cli());
  let db = db.unwrap();
  let broker = broker.unwrap();
  let host: SocketAddr = cfg.host.parse().unwrap();
  func(cfg, sig, db, broker, host).await;
}

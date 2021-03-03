use ::clap::Clap;
use ::futures::future::{select, Either};
use ::libc::{SIGINT, SIGTERM};
use ::nats::asynk::connect as new_broker;
use ::tokio::signal::unix as signal;

use ::binance_notification as binance;
use ::config::{CmdArgs, Config};
use ::notification::UserStream as UserStreamTrait;

#[::tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let config = Config::from_fpath(Some(args.config)).unwrap();
  let logger = config.build_slog();
  let broker = new_broker(config.broker_url.as_str()).await.unwrap();
  let binance = binance::UserStream::new(broker, logger);
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let sig = Box::pin(sig.recv());
  let jobs = binance.start();
  match select(jobs, sig).await {
    Either::Left((v, _)) => v,
    Either::Right(_) => Ok(()),
  }
  .unwrap();
}

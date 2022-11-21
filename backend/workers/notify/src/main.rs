use ::clap::Parser;
use ::futures::future::{select, Either};
use ::libc::{SIGINT, SIGTERM};
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::notification::binance;
use ::notification::traits::UserStream as UserStreamTrait;

#[::tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let config = Config::from_fpath(Some(args.config)).unwrap();
  config.init_logger();
  let broker = config.nats_cli().unwrap();
  let binance = binance::UserStream::new(broker);
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

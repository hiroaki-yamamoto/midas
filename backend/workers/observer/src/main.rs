use ::clap::Clap;
use ::futures::future::{select, Either};
use ::libc::{SIGINT, SIGTERM};
use ::nats::asynk::connect as new_broker;
use ::slog::o;
use ::tokio::signal::unix as signal;

use ::config::{Config, DEFAULT_CONFIG_PATH};
use ::exchanges::{binance, TradeObserver};
use ::rpc::entities::Exchanges;
use ::slog_builder::{build_debug, build_json};

#[derive(Debug, Clap)]
#[clap(author = "Hiroaki Yamamoto")]
struct CmdArgs {
  #[clap(short, long)]
  pub exchange: Exchanges,
  #[clap(short, long, default_value = DEFAULT_CONFIG_PATH)]
  pub config: String,
}

#[::tokio::main]
async fn main() {
  let cmd_args: CmdArgs = CmdArgs::parse();
  let config = Config::from_fpath(Some(cmd_args.config)).unwrap();

  let (logger, _) = match config.debug {
    true => build_debug(),
    false => build_json(),
  };
  let broker = new_broker(&config.broker_url).await.unwrap();
  let exchange: Box<dyn TradeObserver> = match cmd_args.exchange {
    Exchanges::Binance => Box::new(binance::TradeObserver::new(
      broker,
      logger.new(o!("scope" => "Trade Observer")),
    )),
  };
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let sig = Box::pin(sig.recv());
  match select(exchange.start(), sig).await {
    Either::Left((v, _)) => v,
    Either::Right(_) => Ok(()),
  }
  .unwrap();
}

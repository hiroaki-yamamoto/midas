use ::clap::Parser;
use ::futures::future::{select, Either};
use ::libc::{SIGINT, SIGTERM};
use ::tokio::signal::unix as signal;

use ::config::{Config, DEFAULT_CONFIG_PATH};
use ::observers::binance;
use ::observers::traits::TradeObserver as TradeObserverTrait;
use ::rpc::entities::Exchanges;

#[derive(Debug, Parser)]
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

  let broker = config.nats_cli().unwrap();
  let db = config.db().await.unwrap();
  config.init_logger();
  let exchange: Box<dyn TradeObserverTrait> = match cmd_args.exchange {
    Exchanges::Binance => {
      Box::new(binance::TradeObserver::new(Some(db), broker).await)
    }
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

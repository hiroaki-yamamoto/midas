use ::clap::Parser;
use ::libc::{SIGINT, SIGTERM};
use ::log::error;
use ::tokio::signal::unix as signal;

use ::config::{Config, DEFAULT_CONFIG_PATH};
use ::observers::binance;
use ::observers::traits::ITradeObserver as TradeObserverTrait;
use ::rpc::exchanges::Exchanges;

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
  config.init_logger();

  let broker = config.nats_cli().await.unwrap();
  let db = config.db().await.unwrap();
  let mut exchange: Box<dyn TradeObserverTrait> = match cmd_args.exchange {
    Exchanges::Binance => {
      Box::new(binance::TradeObserver::new(&broker, &db).await.unwrap())
    }
  };
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  if let Err(e) = exchange.start(&mut sig).await {
    error!(error: err = e; "An Error Occurred.");
  }
}

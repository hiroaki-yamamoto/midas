use ::clap::Parser;
use ::libc::{SIGINT, SIGTERM};
use ::log::{as_error, error};
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
  config.init_logger();

  let broker = config.nats_cli().await.unwrap();
  let redis = config.redis().unwrap();
  let exchange: Box<dyn TradeObserverTrait> = match cmd_args.exchange {
    Exchanges::Binance => {
      Box::new(binance::TradeObserver::new(&broker, redis).await.unwrap())
    }
  };
  let sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  if let Err(e) = exchange.start(sig.into()).await {
    error!(error = as_error!(e); "An Error Occurred.");
  }
}

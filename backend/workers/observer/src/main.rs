use ::clap::Clap;
use ::nats::asynk::connect as new_broker;
use ::slog::o;
use ::tonic::transport::Channel;

use ::config::{Config, DEFAULT_CONFIG_PATH};
use ::exchanges::{binance, TradeObserver};
use ::rpc::entities::Exchanges;
use ::rpc::symbol::symbol_client::SymbolClient;
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
  let tls = config.tls.load_client().unwrap();
  let channel =
    Channel::from_shared(config.service_addresses.symbol.into_bytes())
      .unwrap()
      .tls_config(tls)
      .unwrap()
      .connect()
      .await
      .unwrap();
  let mut symbol_client = SymbolClient::new(channel);
  let exchange: Box<dyn TradeObserver> = match cmd_args.exchange {
    Exchanges::Binance => Box::new(
      binance::TradeObserver::new(
        broker,
        logger.new(o!("scope" => "Trade Observer")),
        &mut symbol_client,
      )
      .await
      .unwrap(),
    ),
  };
  let _ = exchange.start().await.unwrap();
}

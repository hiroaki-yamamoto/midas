use ::clap::Parser;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::nats::connect;
use ::slog::{info, warn};
use ::std::collections::HashMap;
use ::subscribe::PubSub;
use ::tokio::select;
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::rpc::entities::Exchanges;

use ::history::binance::fetcher::HistoryFetcher as BinanceHistoryFetcher;
use ::history::binance::pubsub::HistChartPubSub;
use ::history::traits::HistoryFetcher;

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  info!(logger, "Kline fetch worker");
  let broker = connect(&cfg.broker_url).unwrap();
  let db =
    DBCli::with_options(MongoDBCliOpt::parse(&cfg.db_url).await.unwrap())
      .unwrap()
      .database("midas");

  let pubsub = HistChartPubSub::new(broker);
  let mut sub = pubsub.subscribe().unwrap();
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();

  let mut reg: HashMap<Exchanges, &dyn HistoryFetcher> = HashMap::new();
  let binance_fetcher =
    BinanceHistoryFetcher::new(None, logger.clone()).unwrap();
  reg.insert(Exchanges::Binance, &binance_fetcher as &dyn HistoryFetcher);

  loop {
    select! {
      Some((req, _)) = sub.next() => {
        match reg.get(&req.exchange) {
          Some(fetcher) => {
            let kline = fetcher.fetch(&req).await;
          },
          None => {
            warn!(
              logger,
              "Unknown Exchange Type: {}",
              req.exchange.as_string()
            );
          }
        }
      },
      _ = sig.recv() => {
        break;
      },
    }
  }
}

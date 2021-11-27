use ::clap::Parser;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::nats::connect;
use ::slog::{info, warn};
use ::subscribe::PubSub;
use ::tokio::select;
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::rpc::entities::Exchanges;

use ::binance_histories::fetcher::HistoryFetcher as BinanceHistoryFetcher;
use ::binance_histories::pubsub::HistChartPubSub;
use ::history::HistoryFetcher;

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
  let binance_fetcher =
    BinanceHistoryFetcher::new(None, logger.clone()).unwrap();
  loop {
    select! {
      Some((req, _)) = sub.next() => {
        match req.exchange {
          Exchanges::Binance => {
            let kline = binance_fetcher.fetch(&req).await;
          }
          _ => {
            warn!(
              logger,
              "Unknown Exchange Type: {}",
              req.exchange.as_string()
            );
            continue;
          }
        }
      },
      _ = sig.recv() => {
        break;
      },
    }
  }
}

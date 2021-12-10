use ::std::collections::HashMap;

use ::clap::Parser;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::nats::connect;
use ::slog::{error, info};
use ::subscribe::PubSub;
use ::tokio::select;
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::rpc::entities::Exchanges;

use ::history::binance::fetcher::HistoryFetcher;
use ::history::binance::writer::HistoryWriter;
use ::history::entities::FetchStatusChanged;
use ::history::kvs::CurrentSyncProgressStore;
use ::history::pubsub::{FetchStatusEventPubSub, HistChartPubSub};
use ::history::traits::{
  HistoryFetcher as HistoryFetcherTrait, HistoryWriter as HistoryWriterTrait,
};

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
  let redis = cfg.redis(&logger).unwrap();
  let mut cur_prog_kvs = CurrentSyncProgressStore::new(redis);

  let pubsub = HistChartPubSub::new(broker.clone());
  let mut sub = pubsub.queue_subscribe("histChart.fetcher").unwrap();
  let change_event_pub = FetchStatusEventPubSub::new(broker);
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();

  let mut reg: HashMap<Exchanges, Box<dyn HistoryFetcherTrait>> =
    HashMap::new();

  let fetcher = HistoryFetcher::new(None, logger.clone()).unwrap();
  let writer = HistoryWriter::new(&db);
  reg.insert(Exchanges::Binance, Box::new(fetcher));

  loop {
    select! {
      Some((req, _)) = sub.next() => {
        let klines = match reg.get(&req.exchange) {
          Some(fetcher) => {
            match fetcher.fetch(&req).await {
              Err(e) => {
                error!(logger, "Failed to fetch klines: {}", e);
                continue;
              },
              Ok(k) => k
            }
          },
          None => {
            error!(logger, "Unknown Exchange: {}", req.exchange.as_string());
            continue;
          }
        };
        if let Err(e) = writer.write(klines).await {
          error!(logger, "Failed to write the klines: {}", e);
          continue;
        }
        if let Err(e) = cur_prog_kvs.incr(
          req.exchange.as_string(),
          req.symbol.clone(), 1
        ) {
          error!(logger, "Failed to report the progress: {}", e);
        };
        if let Err(e) = change_event_pub.publish(&FetchStatusChanged{
          exchange: req.exchange,
          symbol: req.symbol,
        }) {
          error!(logger, "Failed to broadcast progress changed event: {}", e);
        };
      },
      _ = sig.recv() => {
        break;
      },
    }
  }
}

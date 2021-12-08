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

use ::history::binance::fetcher::HistoryFetcher;
use ::history::binance::pubsub::HistChartPubSub;
use ::history::binance::writer::HistoryWriter;
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

  let pubsub = HistChartPubSub::new(broker);
  let mut sub = pubsub.subscribe().unwrap();
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();

  let fetcher = HistoryFetcher::new(None, logger.clone()).unwrap();
  let writer = HistoryWriter::new(&db);

  loop {
    select! {
      Some((req, _)) = sub.next() => {
        let klines = match fetcher.fetch(&req).await {
          Err(e) => {
            error!(logger, "Failed to fetch klines: {}", e);
            continue;
          },
          Ok(k) => k
        };
        if let Err(e) = writer.write(klines).await {
          error!(logger, "Failed to write the klines: {}", e);
          continue;
        }
      },
      _ = sig.recv() => {
        break;
      },
    }
  }
}

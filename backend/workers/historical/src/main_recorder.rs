use ::clap::Clap;
use ::futures::future::{join_all, select};
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::nats::asynk::connect;
use ::slog::{info, o};
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::exchanges::binance::HistoryRecorder as BinanceHistoryRecorder;
use ::exchanges::HistoryRecorder;

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let (logger, _) = cfg.build_slog();
  info!(logger, "Kline fetch worker");
  let broker = connect(&cfg.broker_url).await.unwrap();
  let db =
    DBCli::with_options(MongoDBCliOpt::parse(&cfg.db_url).await.unwrap())
      .unwrap()
      .database("midas");
  let fetchers: Vec<Box<dyn HistoryRecorder>> = vec![Box::new(
    BinanceHistoryRecorder::new(
      db,
      logger.new(o!("scope" => "history_fetcher")),
      broker,
    )
    .await,
  )];
  let fetchers = fetchers.iter().map(|fetcher| fetcher.spawn());
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let sig = Box::pin(sig.recv());
  select(join_all(fetchers), sig).await;
}

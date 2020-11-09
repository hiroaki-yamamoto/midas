use ::clap::Clap;
use ::futures::future::{join_all, select};
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::nats::asynk::connect;
use ::slog::{info, o};
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::exchanges::binance::{
  HistoryFetcher as BinanceHistoryFetcher,
  SymbolFetcher as BinanceSymbolFetcher,
};
use ::exchanges::HistoryFetcher;
use ::types::GenericResult;

#[tokio::main]
async fn main() -> GenericResult<()> {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config))?;
  let (logger, _) = cfg.build_slog();
  info!(logger, "Kline fetch worker");
  let broker = connect(&cfg.broker_url).await?;
  let db = DBCli::with_options(MongoDBCliOpt::parse(&cfg.db_url).await?)?
    .database("midas");
  let fetchers: Vec<Box<dyn HistoryFetcher>> = vec![Box::new(
    BinanceHistoryFetcher::new(
      None,
      logger.clone(),
      broker.clone(),
      BinanceSymbolFetcher::new(
        logger.new(o!("scope" => "SymbolFetcher")),
        broker,
        db,
      )
      .await,
    )
    .await?,
  )];
  let fetchers = fetchers.iter().map(|fetcher| fetcher.spawn());
  let mut sig = signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT))?;
  let sig = Box::pin(sig.recv());
  select(join_all(fetchers), sig).await;
  return Ok(());
}

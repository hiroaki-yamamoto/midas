use ::clap::Clap;
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::nats::asynk::connect;
use ::slog::{info, Logger};

use ::config::{CmdArgs, Config};
use ::exchanges::binance::HistoryFetcher as BinanceHistoryFetcher;
use ::exchanges::HistoryFetcher;
use ::slog_builder::{build_debug, build_json};
use ::types::GenericResult;

#[tokio::main]
async fn main() -> GenericResult<()> {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config))?;
  let logger: Logger;
  if cfg.debug {
    let (debug_logger, _) = build_debug();
    logger = debug_logger;
  } else {
    let (prd_logger, _) = build_json();
    logger = prd_logger;
  }
  info!(logger, "Kline fetch worker");
  let broker = connect(&cfg.broker_url).await;
  let db = DBCli::with_options(MongoDBCliOpt::parse(&cfg.db_url).await?)?
    .database("midas");
  // let fetchers: Vec<Box<dyn HistoryFetcher>> =
  //   vec![BinanceHistoryFetcher::new(
  //     None,
  //     logger: Logger,
  //     broker: Connection,
  //     symbol_fetcher: SymbolFetcher,
  //   )];
  return Ok(());
}

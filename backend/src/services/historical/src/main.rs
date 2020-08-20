mod config;
mod manager;
mod server;

use crate::config::Config;
use ::clap::Clap;
use ::slog::info;
use ::slog::Logger;
use ::slog_atomic::AtomicSwitchCtrl;
use ::slog_builder::{build_debug, build_json};
use ::std::error::Error;

use ::mongodb::options::ClientOptions as MongoDBCliOpt;

#[derive(Clap)]
#[clap(author = "Hiroaki Yamamoto")]
struct CmdArgs {
  #[clap(short, long, default_value = "/etc/midas/historical.yml")]
  config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(args.config)?;
  let logger: Logger;
  let ctrl: AtomicSwitchCtrl;
  if cfg.debug {
    let (debug_logger, debug_ctrl) = build_debug();
    logger = debug_logger;
    ctrl = debug_ctrl;
  } else {
    let (prd_logger, prd_ctrl) = build_json();
    logger = prd_logger;
    ctrl = prd_ctrl;
  }
  info!(logger, "test");
  let broker = ::nats::connect(&cfg.broker_url)?;
  let db = MongoDBCliOpt::parse(&cfg.db_url).await?;
  return Ok(());
}

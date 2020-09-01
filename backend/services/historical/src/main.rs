mod config;
mod entities;
mod manager;
mod service;

use crate::config::Config;
use crate::service::Service;
use ::clap::Clap;
use ::rpc::historical::hist_chart_server::HistChartServer;
use ::slog::info;
use ::slog::Logger;
use ::slog_builder::{build_debug, build_json};
use ::std::error::Error;
use ::std::net::SocketAddr;
use ::tonic::transport::Server as RPCServer;

use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;

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
  if cfg.debug {
    let (debug_logger, _) = build_debug();
    logger = debug_logger;
  } else {
    let (prd_logger, _) = build_json();
    logger = prd_logger;
  }
  info!(logger, "Kline History Fetcher");
  let broker = ::nats::connect(&cfg.broker_url)?;
  let db = DBCli::with_options(MongoDBCliOpt::parse(&cfg.db_url).await?)?
    .database("midas");
  let host: SocketAddr = cfg.host.parse()?;
  let svc = Service::new(&logger, &db, broker)?;
  let svc = HistChartServer::new(svc);
  info!(logger, "Opened history fetcher RPC server on {}", host);
  RPCServer::builder().add_service(svc).serve(host).await?;
  return Ok(());
}

mod entities;
mod manager;
mod service;

use ::std::error::Error;
use ::std::net::SocketAddr;

use ::clap::Clap;
use ::futures::FutureExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::slog::info;
use ::slog::Logger;
use ::slog_builder::{build_debug, build_json};
use ::tokio::signal::unix as signal;
use ::tonic::transport::Server as RPCServer;

use ::config::{CmdArgs, Config};
use ::rpc::historical::hist_chart_server::HistChartServer;

use crate::service::Service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut sig = signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT))?;
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
  info!(logger, "Historical Kline Service");
  let broker = ::nats::connect(&cfg.broker_url)?;
  let db = DBCli::with_options(MongoDBCliOpt::parse(&cfg.db_url).await?)?
    .database("midas");
  let host: SocketAddr = cfg.host.parse()?;
  let svc = Service::new(&logger, &db, broker)?;
  let svc = HistChartServer::new(svc);
  info!(logger, "Opened the server on {}", host);
  RPCServer::builder()
    .add_service(svc)
    .serve_with_shutdown(host, sig.recv().then(|_| async { () }))
    .await?;
  return Ok(());
}

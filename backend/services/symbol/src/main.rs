mod service;

use ::std::net::SocketAddr;

use ::clap::Clap;
use ::mongodb::{options::ClientOptions as DBCliOpt, Client as DBCli};
use ::slog::{info, o, Logger};
use ::tonic::transport::Server as RPCServer;

use ::config::{CmdArgs, Config};
use ::rpc::symbol::symbol_server::SymbolServer;
use ::slog_builder::{build_debug, build_json};
use ::types::GenericResult;

use self::service::Service;

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
  info!(logger, "Kline History Fetcher");
  let db =
    DBCli::with_options(DBCliOpt::parse(&cfg.db_url).await?)?.database("midas");
  let host: SocketAddr = cfg.host.parse()?;
  let svc = Service::new(&db, logger.new(o!("scope" => "SymbolService")));
  let svc = SymbolServer::new(svc);
  info!(logger, "Opened history fetcher RPC server on {}", host);
  RPCServer::builder().add_service(svc).serve(host).await?;
  return Ok(());
}

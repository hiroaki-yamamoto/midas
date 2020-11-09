mod service;

use ::std::net::SocketAddr;

use ::clap::Clap;
use ::futures::FutureExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::{options::ClientOptions as DBCliOpt, Client as DBCli};
use ::nats::asynk::connect as connect_broker;
use ::slog::{info, o};
use ::tokio::signal::unix as signal;
use ::tonic::transport::Server as RPCServer;

use ::config::{CmdArgs, Config};
use ::rpc::symbol::symbol_server::SymbolServer;
use ::types::GenericResult;

use self::service::Service;

#[tokio::main]
async fn main() -> GenericResult<()> {
  let mut sig = signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT))?;
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config))?;
  let (logger, _) = cfg.build_slog();
  info!(logger, "Symbol Service");
  let db =
    DBCli::with_options(DBCliOpt::parse(&cfg.db_url).await?)?.database("midas");
  let broker = connect_broker(&cfg.broker_url).await?;
  let host: SocketAddr = cfg.host.parse()?;
  let svc =
    Service::new(&db, broker, logger.new(o!("scope" => "SymbolService"))).await;
  let svc = SymbolServer::new(svc);
  info!(logger, "Opened the server on {}", host);
  RPCServer::builder()
    .tls_config(cfg.tls.load_server()?)?
    .add_service(svc)
    .serve_with_shutdown(host, sig.recv().then(|_| async { () }))
    .await?;
  return Ok(());
}

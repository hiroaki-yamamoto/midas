mod service;

use ::std::net::SocketAddr;

use ::clap::Clap;
use ::futures::FutureExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::{options::ClientOptions as DBCliOpt, Client as DBCli};
use ::nats::asynk::connect as connect_broker;
use ::slog::{info, o};
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::types::GenericResult;

use self::service::Service;

#[tokio::main]
async fn main() -> GenericResult<()> {
  let mut sig = signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT))?;
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config))?;
  let (logger, _) = cfg.build_slog();
  let db =
    DBCli::with_options(DBCliOpt::parse(&cfg.db_url).await?)?.database("midas");
  let broker = connect_broker(&cfg.broker_url).await?;
  let host: SocketAddr = cfg.host.parse()?;
  let svc =
    Service::new(&db, broker, logger.new(o!("scope" => "SymbolService"))).await;

  info!(logger, "Opened REST server on {}", host);
  let (_, svr) = ::warp::serve(svc.route())
    .tls()
    .cert_path(&cfg.tls.cert)
    .key_path(&cfg.tls.prv_key)
    .bind_with_graceful_shutdown(host, async move {
      sig.recv().await;
    });
  let svr = svr.then(|_| async {
    ::slog::warn!(logger, "REST Server is shutting down! Bye! Bye!");
  });
  svr.await;
  return Ok(());
}

mod service;

use ::std::net::SocketAddr;

use ::clap::Clap;
use ::futures::FutureExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::{options::ClientOptions as DBCliOpt, Client as DBCli};
use ::nats::asynk::connect as connect_broker;
use ::slog::{info, o};
use ::tokio::signal::unix as signal;
use ::warp::Filter;

use ::config::{CmdArgs, Config};
use ::csrf::{CSRFOption, CSRF};

use self::service::Service;

#[tokio::main]
async fn main() {
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  let db = DBCli::with_options(DBCliOpt::parse(&cfg.db_url).await.unwrap())
    .unwrap()
    .database("midas");
  let broker = connect_broker(&cfg.broker_url).await.unwrap();
  let host: SocketAddr = cfg.host.parse().unwrap();
  let svc =
    Service::new(&db, broker, logger.new(o!("scope" => "SymbolService"))).await;
  let csrf = CSRF::new(CSRFOption::builder());
  let router = csrf.protect().and(svc.route().await);

  info!(logger, "Opened REST server on {}", host);
  let (_, svr) = ::warp::serve(router)
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
  return;
}

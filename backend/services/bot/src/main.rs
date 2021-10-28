mod routing;

use ::std::net::SocketAddr;

use ::clap::Parser;
use ::futures::FutureExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::slog::info;
use ::tokio::signal::unix as signal;
use ::warp::Filter;

use ::config::{CmdArgs, Config};
use ::csrf::{CSRFOption, CSRF};
use ::rpc::rejection_handler::handle_rejection;

use self::routing::construct;

#[tokio::main]
async fn main() {
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  let db =
    DBCli::with_options(MongoDBCliOpt::parse(&cfg.db_url).await.unwrap())
      .unwrap()
      .database("midas");
  let http_cli = cfg.build_rest_client().unwrap();
  let host: SocketAddr = cfg.host.parse().unwrap();
  let csrf = CSRF::new(CSRFOption::builder());
  let route = construct(&db, http_cli);
  let route = csrf.protect().and(route).recover(handle_rejection);

  info!(logger, "Opened REST server on {}", host);
  let (_, svr) = ::warp::serve(route)
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
}

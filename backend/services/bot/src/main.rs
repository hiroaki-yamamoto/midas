mod routing;

use ::std::net::SocketAddr;

use ::clap::Parser;
use ::futures::FutureExt;
use ::libc::{SIGINT, SIGTERM};
use ::log::{info, warn};
use ::tokio::signal::unix as signal;
use ::warp::Filter;

use ::access_logger::log;
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
  cfg.init_logger();
  let access_logger = log();
  let db = cfg.db().await.unwrap();
  let http_cli = cfg.build_rest_client().unwrap();
  let host: SocketAddr = cfg.host.parse().unwrap();
  let csrf = CSRF::new(CSRFOption::builder());
  let route = construct(&db, http_cli, &cfg.transpiler_url);
  let route = csrf
    .protect()
    .and(route)
    .with(access_logger)
    .recover(handle_rejection);

  info!("Opened REST server on {}", host);
  let (_, svr) = ::warp::serve(route)
    .tls()
    .cert_path(&cfg.tls.cert)
    .key_path(&cfg.tls.prv_key)
    .bind_with_graceful_shutdown(host, async move {
      sig.recv().await;
    });
  let svr = svr.then(|_| async {
    warn!("REST Server is shutting down! Bye! Bye!");
  });
  svr.await;
}

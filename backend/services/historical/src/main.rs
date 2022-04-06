mod service;

use ::std::net::SocketAddr;

use ::clap::Parser;
use ::futures::FutureExt;
use ::libc::{SIGINT, SIGTERM};
use ::probe::probe;
use ::slog::{info, warn};
use ::tokio::signal::unix as signal;
use ::warp::Filter;

use ::config::{CmdArgs, Config};
use ::csrf::{CSRFOption, CSRF};
use ::rpc::rejection_handler::handle_rejection;

use crate::service::Service;

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  info!(logger, "Historical Kline Service");
  let broker = ::nats::connect(&cfg.broker_url).unwrap();
  let redis = cfg.redis;
  let host: SocketAddr = cfg.host.parse().unwrap();
  let svc = Service::new(&broker, &redis).await.unwrap();
  let csrf = CSRF::new(CSRFOption::builder());
  let route = csrf
    .protect()
    .and(svc.route())
    .or(probe())
    .recover(handle_rejection);

  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let host = host.clone();
  info!(logger, "Opened REST server on {}", host);
  let (_, ws_svr) = ::warp::serve(route)
    .tls()
    .cert_path(&cfg.tls.cert)
    .key_path(&cfg.tls.prv_key)
    .bind_with_graceful_shutdown(host, async move {
      sig.recv().await;
    });
  let svr = ws_svr.then(|_| async {
    warn!(logger, "REST Server is shutting down! Bye! Bye!");
  });
  svr.await;
}

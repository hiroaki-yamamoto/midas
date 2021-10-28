use ::std::net::SocketAddr;

use ::clap::Parser;
use ::futures::FutureExt;
use ::libc::{SIGINT, SIGTERM};
use ::slog::info;
use ::tokio::signal::unix as signal;
use ::warp::reply;
use ::warp::Filter;

use ::config::{CmdArgs, Config};
use ::csrf::{CSRFOption, CSRF};

#[tokio::main]
async fn main() {
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  let host: SocketAddr = cfg.host.parse().unwrap();
  let csrf = CSRF::new(CSRFOption::builder());
  let route = ::warp::get()
    .or(::warp::head())
    .or(::warp::options())
    .and(::warp::path("csrf"))
    .map(|_| reply::reply());
  let route = csrf.generate_cookie(route);

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

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
use ::slog::{info, warn};
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};

use crate::service::Service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config))?;
  let (logger, _) = cfg.build_slog();
  info!(logger, "Historical Kline Service");
  let broker = ::nats::asynk::connect(&cfg.broker_url).await?;
  let db = DBCli::with_options(MongoDBCliOpt::parse(&cfg.db_url).await?)?
    .database("midas");
  let host: SocketAddr = cfg.host.parse()?;
  let svc = Service::new(&logger, &db, broker.clone()).await?;
  let route = svc.route();

  let mut sig = signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT))?;
  let host = host.clone();
  info!(logger, "Opened Websocket server on {}", host);
  let (_, ws_svr) = ::warp::serve(route)
    .tls()
    .cert_path(&cfg.tls.cert)
    .key_path(&cfg.tls.prv_key)
    .bind_with_graceful_shutdown(host, async move {
      sig
        .recv()
        .then(|_| async {
          let _ = svc.graceful_shutdown().await;
        })
        .await;
    });
  let svr = ws_svr.then(|_| async {
    warn!(logger, "Websocket Server is shutting down! Bye! Bye!");
  });
  svr.await;
  return Ok(());
}

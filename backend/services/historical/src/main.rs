mod entities;
mod manager;
mod service;

use ::std::error::Error;
use ::std::net::SocketAddr;

use ::clap::Clap;
use ::futures::{join, FutureExt};
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::slog::{info, warn};
use ::tokio::signal::unix as signal;
use ::tonic::transport::Server as RPCServer;

use ::config::{CmdArgs, Config};
use ::rpc::historical::hist_chart_server::HistChartServer;

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
  let ws_route = svc.get_websocket_route();

  let mut sig = signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT))?;
  let mut ws_sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT))?;
  let mut ws_host = host.clone();
  ws_host.set_port(ws_host.port() + 1);
  info!(logger, "Opened Websocket server on {}", ws_host);
  let (_, ws_svr) = ::warp::serve(ws_route)
    .tls()
    .cert_path(&cfg.tls.cert)
    .key_path(&cfg.tls.prv_key)
    .bind_with_graceful_shutdown(ws_host, async move {
      ws_sig.recv().await;
    });
  let ws_svr = ws_svr.then(|_| async {
    warn!(logger, "Websocket Server is shutting down! Bye! Bye!");
  });

  let svc_clone = svc.clone();
  let svc = HistChartServer::new(svc);
  info!(logger, "Opened GRPC server on {}", host);
  let rpc_svr = RPCServer::builder()
    .tls_config(cfg.tls.load_server()?)?
    .add_service(svc)
    .serve_with_shutdown(host, async move {
      sig
        .recv()
        .then(|_| async move {
          let _ = svc_clone.graceful_shutdown().await;
        })
        .await;
    })
    .then(|_| async {
      warn!(logger, "GRPC Server is shutting down! Bye! Bye!");
    });
  let _ = join!(ws_svr, rpc_svr);
  return Ok(());
}

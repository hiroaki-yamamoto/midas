mod entities;
mod manager;
mod service;

use ::std::error::Error;
use ::std::net::SocketAddr;

use ::clap::Clap;
use ::futures::{join, FutureExt, StreamExt};
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::options::ClientOptions as MongoDBCliOpt;
use ::mongodb::Client as DBCli;
use ::serde_json::to_string as jsonify;
use ::slog::info;
use ::slog::Logger;
use ::slog_builder::{build_debug, build_json};
use ::tokio::signal::unix as signal;
use ::tokio::sync::mpsc;
use ::tonic::transport::Server as RPCServer;
use ::tonic::Request;
use ::warp::ws::{Message, WebSocket, Ws};
use ::warp::Filter;

use ::config::{CmdArgs, Config};
use ::rpc::historical::hist_chart_server::{HistChart, HistChartServer};
use ::types::Status;

use crate::service::Service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
  info!(logger, "Historical Kline Service");
  let broker = ::nats::connect(&cfg.broker_url)?;
  let db = DBCli::with_options(MongoDBCliOpt::parse(&cfg.db_url).await?)?
    .database("midas");
  let host: SocketAddr = cfg.host.parse()?;
  let svc = Service::new(&logger, &db, broker.clone())?;
  let ws_svc = svc.clone();
  let ws_route =
    ::warp::path("subscribe")
      .and(::warp::ws())
      .map(move |ws: Ws| {
        let ws_svc = ws_svc.clone();
        return ws.on_upgrade(|sock: WebSocket| async move {
          let subsc = ws_svc.subscribe(Request::new(())).await;
          let (ret_tx, _) = sock.split();
          let (tx, rx) = mpsc::unbounded_channel();
          let _ = rx.forward(ret_tx);
          match subsc {
            Err(e) => {
              let _ = tx.send(Ok(Message::close_with(
                1011 as u16,
                format!(
                  "Got an error while trying to subscribe the channel: {}",
                  e
                ),
              )));
            }
            Ok(resp) => {
              let mut stream = resp.into_inner();
              while let Some(v) = stream.next().await {
                match v {
                  Err(e) => {
                    let st = Status::from_tonic_status(&e);
                    let _ = tx.send(Ok(Message::text(jsonify(&st).unwrap_or(
                      String::from("Failed to serialize the error"),
                    ))));
                  }
                  Ok(d) => {
                    let _ = tx.send(Ok(Message::text(jsonify(&d).unwrap_or(
                      String::from("Failed to serialize the progress data."),
                    ))));
                  }
                }
              }
            }
          };
        });
      });

  let mut sig = signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT))?;
  let mut ws_sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT))?;
  let mut ws_host = host.clone();
  ws_host.set_port(ws_host.port() + 1);
  info!(logger, "Opened Websocket server on {}", ws_host);
  let (_, ws_svr) =
    ::warp::serve(ws_route).bind_with_graceful_shutdown(ws_host, async move {
      ws_sig.recv().await;
    });

  let svc = HistChartServer::new(svc);
  info!(logger, "Opened GRPC server on {}", host);
  let rpc_svr = RPCServer::builder()
    .tls_config(cfg.tls.load()?)?
    .add_service(svc)
    .serve_with_shutdown(host, sig.recv().then(|_| async { () }));
  let _ = join!(ws_svr, rpc_svr);
  return Ok(());
}

use ::std::net::SocketAddr;

use ::clap::Clap;
use ::futures::{FutureExt, SinkExt, StreamExt};
use ::libc::{SIGINT, SIGTERM};
use ::nats::asynk::{connect as broker_con, Connection as NatsCon};
use ::serde_json::to_string as jsonify;
use ::slog::{o, Logger};
use ::tokio::signal::unix as signal;
use ::types::Status;
use ::warp::ws::Message;
use ::warp::Filter;

use ::config::{CmdArgs, Config};
use ::csrf::{CSRFOption, CSRF};
use ::exchanges::{binance, TradeObserver};
use ::rpc::entities::Exchanges;

fn get_exchange(
  exchange: Exchanges,
  broker: NatsCon,
  logger: Logger,
) -> impl TradeObserver {
  return match exchange {
    Exchanges::Binance => binance::TradeObserver::new(broker, logger),
  };
}

#[::tokio::main]
async fn main() {
  let cmd: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(cmd.config)).unwrap();
  let broker = broker_con(cfg.broker_url.as_str()).await.unwrap();
  let (logger, _) = cfg.build_slog();
  let route_logger = logger.clone();
  let csrf = CSRF::new(CSRFOption::builder());
  let route = csrf
    .protect()
    .and(::warp::ws())
    .and(warp::path::param())
    .map(move |ws: ::warp::ws::Ws, exchange: String| {
      let exchange: Result<Exchanges, String> = exchange.parse();
      let broker = broker.clone();
      let logger = route_logger.new(o! {
        "scope" => "Trade Observer Service"
      });
      return ws.on_upgrade(|mut socket: ::warp::ws::WebSocket| async move {
        let exchange = match exchange {
          Err(e) => {
            let _ = socket.send(Message::close_with(1003 as u16, e)).await;
            let _ = socket.close().await;
            return;
          }
          Ok(v) => {
            get_exchange(v, broker, logger.new(o!("exchange" => v.as_string())))
          }
        };
        let mut sub = match exchange.subscribe().await {
          Ok(sub) => sub,
          Err(e) => {
            let _ = socket
              .send(Message::close_with(1001 as u16, format!("{}", e)))
              .await;
            let _ = socket.close().await;
            return;
          }
        };
        while let Some(best_price) = sub.next().await {
          let _ = socket
            .send(Message::text(jsonify(&best_price).unwrap_or_else(|e| {
              return jsonify(&Status::new_int(0, format!("{}", e).as_str()))
                .unwrap_or_else(|e| format!("{}", e));
            })))
            .await;
          let _ = socket.flush().await;
        }
      });
    });
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let host: SocketAddr = cfg.host.parse().unwrap();
  let (_, ws_svr) = ::warp::serve(route)
    .tls()
    .cert_path(&cfg.tls.cert)
    .key_path(&cfg.tls.prv_key)
    .bind_with_graceful_shutdown(host, async move {
      sig.recv().await;
    });
  ::slog::info!(
    &logger,
    "Starting Trade Observer Websocket Server";
    o!("addr" => host.to_string()),
  );
  let ws_svr = ws_svr.then(|_| async {
    ::slog::warn!(
      &logger,
      "Trade Observer Websocket Server is shutting down! Bye! Bye!"
    );
  });
  ws_svr.await;
}

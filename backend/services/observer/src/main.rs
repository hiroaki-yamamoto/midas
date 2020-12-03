use ::std::collections::HashMap;
use ::std::net::SocketAddr;
use ::std::time::Duration;

use ::clap::Clap;
use ::futures::{FutureExt, SinkExt, StreamExt};
use ::libc::{SIGINT, SIGTERM};
use ::nats::asynk::{connect as broker_con, Connection as NatsCon};
use ::prost::Message as ProstMsg;
use ::rpc::entities::Status;
use ::slog::{o, Logger};
use ::tokio::select;
use ::tokio::signal::unix as signal;
use ::tokio::time::interval;
use ::warp::ws::Message;
use ::warp::{Filter, Reply};

use ::config::{CmdArgs, Config};
use ::csrf::{CSRFOption, CSRF};
use ::exchanges::{binance, TradeObserver};
use ::rpc::bookticker::{BookTicker, BookTickers};
use ::rpc::entities::Exchanges;

async fn get_exchange(
  exchange: Exchanges,
  broker: NatsCon,
  logger: Logger,
) -> impl TradeObserver {
  return match exchange {
    Exchanges::Binance => {
      binance::TradeObserver::new(None, broker, logger).await
    }
  };
}

fn handle_websocket(
  exchange: impl TradeObserver + Send + Sync + 'static,
  ws: ::warp::ws::Ws,
) -> impl Reply {
  return ws.on_upgrade(|mut socket: ::warp::ws::WebSocket| async move {
    let mut pairs: BookTickers = BookTickers {
      book_ticker_map: HashMap::new(),
    };
    let mut publish_interval = interval(Duration::from_millis(50));
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
    let mut changed = false;
    loop {
      select! {
        Some(best_price) = sub.next() => {
          let best_price: BookTicker = best_price.into();
          pairs.book_ticker_map.insert(best_price.symbol.clone(), best_price);
          changed = true;
        },
        _ = publish_interval.tick() => {
          if changed {
            let mut buf: Vec<u8> = Vec::new();
            let _ = pairs.encode(&mut buf).unwrap_or_else(|e| {
              buf.clear();
              Status::new_int(0, format!("{}", e).as_str())
                .encode(&mut buf)
                .unwrap_or_else(|e| {
                  buf.clear();
                  buf.extend(format!("{}", e).as_bytes());
                });
            });
            let _ = socket.send(Message::binary(buf)).await;
            let _ = socket.flush().await;
            changed = false;
          }
        }
        else => {break;}
      }
    }
  });
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
    .and(warp::path::param())
    .map(move |exchange: String| {
      return (exchange, broker.clone(), route_logger.clone());
    })
    .untuple_one()
    .and_then(
      |exchange: String, broker: NatsCon, logger: Logger| async move {
        let exchange: Result<Exchanges, String> = exchange.parse();
        let logger = logger.new(o! {
          "scope" => "Trade Observer Service"
        });
        match exchange {
          Err(_) => return Err(::warp::reject::not_found()),
          Ok(exchange) => {
            return Ok(get_exchange(exchange, broker, logger).await)
          }
        };
      },
    )
    .and(::warp::ws())
    .map(handle_websocket);
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

use ::std::collections::HashMap;
use ::std::net::SocketAddr;
use ::std::time::Duration;

use ::clap::Parser;
use ::futures::{FutureExt, SinkExt, StreamExt};
use ::libc::{SIGINT, SIGTERM};
use ::nats::{connect as broker_con, Connection as NatsCon};
use ::rpc::entities::Status;
use ::serde_json::to_string;
use ::slog::{o, Logger};
use ::tokio::select;
use ::tokio::signal::unix as signal;
use ::tokio::time::interval;
use ::warp::ws::Message;
use ::warp::{Filter, Reply};

use ::binance_observers::{self as binance, TradeObserverTrait};
use ::config::{CmdArgs, Config};
use ::csrf::{CSRFOption, CSRF};
use ::rpc::bookticker::BookTicker;
use ::rpc::entities::Exchanges;

async fn get_exchange(
  exchange: Exchanges,
  broker: NatsCon,
  logger: Logger,
) -> Option<impl TradeObserverTrait> {
  return match exchange {
    Exchanges::Binance => {
      Some(binance::TradeObserver::new(None, broker, logger).await)
    }
  };
}

fn handle_websocket(
  exchange: impl TradeObserverTrait + Send + Sync + 'static,
  ws: ::warp::ws::Ws,
) -> impl Reply {
  return ws.on_upgrade(|mut socket: ::warp::ws::WebSocket| async move {
    let mut book_tickers: HashMap<String, BookTicker> = HashMap::new();
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
    let mut needs_flush = false;
    loop {
      select! {
        Some(best_price) = sub.next() => {
          let best_price: BookTicker = best_price.into();
          book_tickers.insert(best_price.symbol.to_owned(), best_price);
          needs_flush = true;
        },
        _ = publish_interval.tick() => {
          if needs_flush {
            let msg: String = to_string(&book_tickers).unwrap_or_else(|e| {
              return to_string(&Status::new_int(0, format!("{}", e).as_str()))
                .unwrap_or_else(
                  |e| format!("Failed to encode the bookticker data: {}", e)
                );
            });
            book_tickers.clear();
            let _ = socket.send(Message::text(msg)).await;
            let _ = socket.flush().await;
            needs_flush = false;
          }
        }
        Some(msg) = socket.next() => {
          let msg = msg.unwrap_or(::warp::filters::ws::Message::close());
          if msg.is_close() {
            break;
          }
          continue;
        },
        else => {
          break;
        }
      }
    }
    let _ = socket.close().await;
  });
}

#[::tokio::main]
async fn main() {
  let cmd: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(cmd.config)).unwrap();
  let broker = broker_con(cfg.broker_url.as_str()).unwrap();
  let logger = cfg.build_slog();
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
        let exchange: Exchanges =
          exchange.parse().map_err(|_| ::warp::reject::not_found())?;
        let observer = match exchange {
          Exchanges::Binance => get_exchange(exchange, broker, logger).await,
        };
        return match observer {
          None => Err(::warp::reject::not_found()),
          Some(o) => Ok(o),
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

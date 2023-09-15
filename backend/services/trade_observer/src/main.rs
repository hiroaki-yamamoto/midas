use ::std::collections::HashMap;
use ::std::time::Duration;

use ::futures::{FutureExt, SinkExt, StreamExt};
use ::log::{info, warn};
use ::rpc::entities::Status;
use ::serde_json::to_string;
use ::tokio::select;
use ::tokio::time::interval;
use ::warp::ws::Message;
use ::warp::{Filter, Reply};

use ::access_logger::log;
use ::config::init;
use ::csrf::{CSRFOption, CSRF};
use ::errors::CreateStreamResult;
use ::observers::binance;
use ::observers::traits::TradeSubscriber as TradeSubscriberTrait;
use ::rpc::bookticker::BookTicker;
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;

async fn get_exchange(
  exchange: Exchanges,
  broker: &Nats,
) -> CreateStreamResult<Option<impl TradeSubscriberTrait>> {
  return Ok(match exchange {
    Exchanges::Binance => Some(binance::TradeSubscriber::new(broker).await?),
  });
}

fn handle_websocket(
  exchange: impl TradeSubscriberTrait + Send + Sync + 'static,
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
              return to_string(&Status::new_int(0, &format!("{}", e)))
                .unwrap_or_else(
                  |e| format!("Failed to encode the bookticker data: {}", e)
                );
            });
            book_tickers.clear();
            let _ = socket.send(Message::text(msg)).await;
            let _ = socket.flush().await;
            needs_flush = false;
          }
        },
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
  init(|cfg, mut sig, _, broker, host| async move {
    let csrf = CSRF::new(CSRFOption::builder());
    let route = csrf
      .protect()
      .and(warp::path::param())
      .map(move |exchange: String| {
        return (exchange, broker.clone());
      })
      .untuple_one()
      .and_then(|exchange: String, broker: Nats| async move {
        let exchange: Exchanges = Exchanges::from_str_name(&exchange)
          .ok_or(::warp::reject::not_found())?;
        let observer = match exchange {
          Exchanges::Binance => get_exchange(exchange, &broker).await,
        }
        .unwrap();
        return match observer {
          None => Err(::warp::reject::not_found()),
          Some(o) => Ok(o),
        };
      })
      .and(::warp::ws())
      .map(handle_websocket)
      .with(log());
    let (_, ws_svr) = ::warp::serve(route)
      .tls()
      .cert_path(&cfg.tls.cert)
      .key_path(&cfg.tls.prv_key)
      .bind_with_graceful_shutdown(host, async move {
        sig.recv().await;
      });
    info!(
      addr = host.to_string();
      "Starting Trade Observer Websocket Server",
    );
    let ws_svr = ws_svr.then(|_| async {
      warn!("Trade Observer Websocket Server is shutting down! Bye! Bye!");
    });
    ws_svr.await;
  })
  .await;
}

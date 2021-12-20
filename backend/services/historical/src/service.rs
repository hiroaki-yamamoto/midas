use ::std::fmt::Debug;

use ::futures::{SinkExt, StreamExt};
use ::mongodb::Database;
use ::nats::Connection as NatsCon;
use ::serde_json::to_string as jsonify;
use ::slog::Logger;
use ::tokio::select;
use ::warp::filters::BoxedFilter;
use ::warp::ws::{Message, WebSocket, Ws};
use ::warp::{Filter, Reply};
use subscribe::PubSub;

use ::history::binance::fetcher as binance_hist;
use ::history::pubsub::FetchStatusEventPubSub;
use ::types::GenericResult;

#[derive(Debug, Clone)]
pub struct Service {
  logger: Logger,
  binance: binance_hist::HistoryFetcher,
  status: FetchStatusEventPubSub,
}

impl Service {
  pub async fn new(
    log: &Logger,
    db: &Database,
    nats: NatsCon,
  ) -> GenericResult<Self> {
    let binance = binance_hist::HistoryFetcher::new(None, log.clone())?;
    let ret = Self {
      logger: log.clone(),
      binance,
      status: FetchStatusEventPubSub::new(nats.clone()),
    };
    return Ok(ret);
  }

  pub fn route(&self) -> BoxedFilter<(impl Reply,)> {
    return self.websocket().boxed();
  }

  fn websocket(&self) -> BoxedFilter<(impl Reply,)> {
    let ws_svc = self.clone();
    return ::warp::path("subscribe")
      .and(::warp::ws())
      .map(move |ws: Ws| {
        let ws_svc = ws_svc.clone();
        return ws.on_upgrade(|mut sock: WebSocket| async move {
          let subsc = ws_svc.status.subscribe();
          match subsc {
            Err(e) => {
              let msg = format!(
                "Got an error while trying to subscribe the channel: {}",
                e
              );
              let _ = sock.send(Message::close_with(1011 as u16, msg)).await;
              let _ = sock.flush().await;
            }
            Ok(mut resp) => loop {
              select! {
                Some((item, _)) = resp.next() => {
                  match item {
                    KlineFetchStatus::ProgressChanged {
                      exchange: _,
                      previous: _,
                      current,
                    } => {
                      let payload = jsonify(&current).unwrap_or(String::from(
                        "Failed to serialize the progress data.",
                      ));
                      let payload = Message::text(payload);
                      let _ = sock.send(payload).await;
                    },
                    KlineFetchStatus::Stop => {
                      break;
                    },
                    _ => {}
                  }
                  let _ = sock.flush().await;
                },
                Some(msg) = sock.next() => {
                  if msg.is_err() {
                    continue;
                  }
                  let msg = msg.unwrap();
                  if msg.is_close() {
                    break;
                  }

                },
              }
            },
          };
          let _ = sock.close().await;
        });
      })
      .boxed();
  }

  fn handle_req(&self, msg: &Message) {}
}

use ::std::fmt::Debug;

use ::futures::{SinkExt, StreamExt};
use ::http::StatusCode;
use ::nats::jetstream::JetStream as NatsJS;
use ::serde_json::{from_slice as parse_json, to_string as jsonify};
use ::subscribe::PubSub;
use ::tokio::select;
use ::warp::filters::BoxedFilter;
use ::warp::reject::custom as reject_custom;
use ::warp::ws::{Message, WebSocket, Ws};
use ::warp::{Filter, Reply};

use ::entities::HistoryFetchRequest as HistFetchReq;
use ::history::kvs::{redis, CurrentSyncProgressStore, NumObjectsToFetchStore};
use ::history::pubsub::{FetchStatusEventPubSub, HistChartDateSplitPubSub};
use ::history::traits::Store;
use ::rpc::entities::Status;
use ::rpc::historical::{
  HistoryFetchRequest as RPCHistFetchReq, Progress, StatusCheckRequest,
};

#[derive(Debug, Clone)]
pub struct Service {
  redis_cli: redis::Client,
  status: FetchStatusEventPubSub,
  splitter: HistChartDateSplitPubSub,
}

impl Service {
  pub async fn new(nats: &NatsJS, redis_cli: &redis::Client) -> Self {
    let ret = Self {
      status: FetchStatusEventPubSub::new(nats.clone()),
      splitter: HistChartDateSplitPubSub::new(nats.clone()),
      redis_cli: redis_cli.clone(),
    };
    return ret;
  }

  pub fn route(&self) -> BoxedFilter<(impl Reply,)> {
    return self.websocket();
  }

  fn websocket(&self) -> BoxedFilter<(impl Reply,)> {
    let me = self.clone();
    return ::warp::path("subscribe")
      .map(move || {
        return me.clone();
      })
      .and_then(|me: Self| async move {
        let size = me.redis_cli.clone()
          .get_connection()
          .map(|con| NumObjectsToFetchStore::new(con))
          .map_err(|err| {
            Status::new(StatusCode::SERVICE_UNAVAILABLE, format!("(Size) {}", err))
          });
        let size = match size {
          Err(e) => return Err(reject_custom(e)),
          Ok(v) => v
        };
        let cur = me.redis_cli
          .get_connection()
          .map(|con| CurrentSyncProgressStore::new(con))
          .map_err(|err| {
            Status::new(StatusCode::SERVICE_UNAVAILABLE, format!("(Current) {}", err))
          });
        let cur = match cur {
          Err(e) => return Err(reject_custom(e)),
          Ok(v) => v
        };
        return Ok((me, size, cur))
      })
      .untuple_one()
      .and(::warp::ws())
      .map(move |
        me: Self,
        mut size: NumObjectsToFetchStore<redis::Connection>,
        mut cur: CurrentSyncProgressStore<redis::Connection>,
        ws: Ws
      | {
        return ws.on_upgrade(|mut sock: WebSocket| async move {
          let subsc = me.status.queue_subscribe("histServiceFetchStatus");
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
                  let size = size.get(
                    item.exchange.as_string(),
                    &item.symbol
                  ).unwrap_or(0);
                  let cur = cur.get(
                    item.exchange.as_string(), &item.symbol
                  ).unwrap_or(0);
                  let prog = Progress {
                    exchange: item.exchange as i32,
                    symbol: item.symbol.clone(),
                    size,
                    cur
                  };
                  let payload = jsonify(&prog).unwrap_or(String::from(
                    "Failed to serialize the progress data.",
                  ));
                  let payload = Message::text(payload);
                  let _ = sock.send(payload).await;
                  let _ = sock.flush().await;
                },
                Some(Ok(msg)) = sock.next() => {
                  if msg.is_close() {
                    break;
                  }
                  if let Ok(req) = parse_json::<RPCHistFetchReq>(msg.as_bytes()) {
                    let req: HistFetchReq = req.into();
                    match me.splitter.publish(&req) {
                      Ok(_) => { println!("Published Sync Start and End Date"); }
                      Err(e) => { println!("Publishing Sync Date Failed: {:?}", e); }
                    }
                  } else if let Ok(req) = parse_json::<StatusCheckRequest>(msg.as_bytes()) {
                    let exchange = req.exchange().as_string();
                    let size = size.get(&exchange, &req.symbol).unwrap_or(0);
                    let cur = cur.get(&exchange, &req.symbol).unwrap_or(0);
                    let prog = Progress {
                      exchange: req.exchange,
                      symbol: req.symbol,
                      size,
                      cur
                    };
                    let payload = jsonify(&prog).unwrap_or(String::from(
                      "Failed to serialize the progress data.",
                    ));
                    let payload = Message::text(payload);
                    let _ = sock.send(payload).await;
                    let _ = sock.flush().await;
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
}

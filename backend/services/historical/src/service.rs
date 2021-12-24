use ::std::fmt::Debug;

use ::futures::{SinkExt, StreamExt};
use ::http::StatusCode;
use ::mongodb::Database;
use ::nats::Connection as NatsCon;
use ::serde_json::{from_slice as parse_json, to_string as jsonify};
use ::slog::Logger;
use ::subscribe::PubSub;
use ::tokio::select;
use ::warp::filters::BoxedFilter;
use ::warp::reject::custom as reject_custom;
use ::warp::ws::{Message, WebSocket, Ws};
use ::warp::{Filter, Reply};

use ::entities::HistoryFetchRequest as HistFetchReq;
use ::history::binance::fetcher as binance_hist;
use ::history::kvs::{redis, CurrentSyncProgressStore, NumObjectsToFetchStore};
use ::history::pubsub::{FetchStatusEventPubSub, RawHistChartPubSub};
use ::history::traits::Store;
use ::rpc::entities::Status;
use ::rpc::historical::{HistoryFetchRequest as RPCHistFetchReq, Progress};
use ::types::GenericResult;

#[derive(Debug, Clone)]
pub struct Service {
  logger: Logger,
  binance: binance_hist::HistoryFetcher,
  redis_cli: redis::Client,
  status: FetchStatusEventPubSub,
  splitter: RawHistChartPubSub,
}

impl Service {
  pub async fn new(
    log: &Logger,
    db: &Database,
    nats: &NatsCon,
    redis_cli: &redis::Client,
  ) -> GenericResult<Self> {
    let binance = binance_hist::HistoryFetcher::new(None, log.clone())?;
    let ret = Self {
      logger: log.clone(),
      binance,
      status: FetchStatusEventPubSub::new(nats.clone()),
      splitter: RawHistChartPubSub::new(nats.clone()),
      redis_cli: redis_cli.clone(),
    };
    return Ok(ret);
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
        let ws_svc = me.clone();
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
                  let size = size.get(
                    item.exchange.as_string(),
                    &item.symbol
                  ).unwrap_or(0);
                  let cur = cur.get(
                    item.exchange.as_string(), &item.symbol
                  ).unwrap_or(0);
                  let prog = Progress {size, cur};
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
                    let _ = ws_svc.splitter.publish(&req);
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

use ::std::sync::Arc;

use ::futures::stream::BoxStream;
use ::futures::{SinkExt, StreamExt};
use ::http::StatusCode;
use ::mongodb::Database;
use ::serde_json::{from_slice as parse_json, to_string as jsonify};
use ::subscribe::PubSub;
use ::tokio::{join, select};
use ::warp::filters::BoxedFilter;
use ::warp::reject::{custom as cus_rej, Rejection};
use ::warp::ws::{Message, WebSocket, Ws};
use ::warp::{Filter, Reply};
use kvs::redis::aio::MultiplexedConnection;

use ::entities::HistoryFetchRequest as HistFetchReq;
use ::errors::KVSResult;
use ::history::kvs::{CUR_SYNC_PROG_KVS_BUILDER, NUM_TO_FETCH_KVS_BUILDER};
use ::history::pubsub::{FetchStatusEventPubSub, HistChartDateSplitPubSub};
use ::kvs::redis;
use ::kvs::traits::symbol::Get;
use ::rpc::exchanges::Exchanges;
use ::rpc::history_fetch_request::HistoryFetchRequest as RPCHistFetchReq;
use ::rpc::progress::Progress;
use ::rpc::status::Status;
use ::rpc::status_check_request::StatusCheckRequest;
use ::subscribe::nats::Client as Nats;
use ::symbols::binance::recorder::SymbolWriter as BinanceSymbolWriter;
use ::symbols::traits::SymbolReader as SymbolReaderTrait;
use ::symbols::types::ListSymbolStream;

use crate::errors::ServiceResult;

type ProgressKVS =
  Arc<dyn Get<Commands = MultiplexedConnection, Value = i64> + Send + Sync>;

#[derive(Debug, Clone)]
pub struct Service {
  num_obj_kvs_get: ProgressKVS,
  sync_prog_kvs_get: ProgressKVS,
  status: FetchStatusEventPubSub,
  splitter: HistChartDateSplitPubSub,
  db: Database,
}

impl Service {
  pub async fn new(
    nats: &Nats,
    redis_cli: &redis::Client,
    db: &Database,
  ) -> ServiceResult<Self> {
    let (status, splitter) = join!(
      FetchStatusEventPubSub::new(nats),
      HistChartDateSplitPubSub::new(nats)
    );
    let redis_con: KVSResult<MultiplexedConnection> = redis_cli
      .get_multiplexed_async_connection()
      .await
      .map_err(|err| err.into());
    let redis_con = redis_con?;
    let num_obj_kvs_get =
      Arc::new(NUM_TO_FETCH_KVS_BUILDER.build(redis_con.clone()));
    let sync_prog_kvs_get =
      Arc::new(CUR_SYNC_PROG_KVS_BUILDER.build(redis_con));
    let ret = Self {
      status: status?,
      splitter: splitter?,
      db: db.clone(),
      num_obj_kvs_get,
      sync_prog_kvs_get,
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
        let writer = BinanceSymbolWriter::new(&me.db).await;
        let symbols = writer.list_trading().await.map_err(|err| {
          return cus_rej(Status::new(
            StatusCode::SERVICE_UNAVAILABLE,
            &format!("(DB, Symbol): {}", err)
          ));
        })?;
        let size = me.num_obj_kvs_get.clone();
        let cur = me.sync_prog_kvs_get.clone();
        return Ok::<_, Rejection>((me, size, cur, symbols));
      })
      .and_then(|(me, size, cur, symbol): (
        Self,
        ProgressKVS, ProgressKVS,
        ListSymbolStream,
      )| async move {
        let (clos_size, clos_cur) = (size.clone(), cur.clone());
        let prog = symbol.map(move |symbol| {
          return (symbol.symbol, clos_size.clone(), clos_cur.clone());
        }).filter_map(|(sym, size, cur)| async move {
          let exchange_name: Arc<String> = Exchanges::Binance.as_str_name().to_lowercase().into();
          let sym: Arc<String> = sym.into();
          let size = size.get(exchange_name.clone(), sym.clone());
          let cur = cur.get(exchange_name.clone(), sym.clone());
          let (size, cur) = join!(size, cur);
          if let Some(size) = size.ok() {
            if let Some(cur) = cur.ok() {
              return Some((size, cur, sym));
            }
          }
          return None;
        }).map(|(size, cur, symbol): (i64, i64, Arc<String>)| {
          return Progress {
            exchange: Box::new(Exchanges::Binance),
            symbol: symbol.as_ref().clone(),
            size,
            cur
          };
        }).filter_map(|prog: Progress| async move {
          return jsonify(&prog).ok();
        }).map(|payload: String| {
          return Message::text(payload);
        }).boxed();
        return Ok::<_, Rejection>((me, size, cur, prog));
      })
      .untuple_one()
      .and(::warp::ws())
      .map(|
        me: Self,
        size: ProgressKVS,
        cur: ProgressKVS,
        mut start_prog: BoxStream<'static, Message>,
        ws: Ws
      | {
        return ws.on_upgrade(|mut sock: WebSocket| async move {
          while let Some(payload) = start_prog.next().await {
            let _ = sock.feed(payload).await;
          }
          let _ = sock.flush();
          let subsc = me.status.pull_subscribe("historyService").await;
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
                  let exchange: Arc<String> = item
                    .exchange
                    .as_str_name()
                    .to_lowercase()
                    .into();
                  let symbol: Arc<String> = item.symbol.into();
                  let size = size.get(exchange.clone(), symbol.clone()).await.unwrap_or(0);
                  let cur = cur.get(exchange.clone(), symbol.clone()).await.unwrap_or(0);
                  let prog = Progress {
                    exchange: item.exchange as i32,
                    symbol: symbol.as_ref().clone(),
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
                    match me.splitter.publish(&req).await {
                      Ok(_) => { println!("Published Sync Start and End Date"); }
                      Err(e) => { println!("Publishing Sync Date Failed: {:?}", e); }
                    }
                  } else if let Ok(req) = parse_json::<StatusCheckRequest>(msg.as_bytes()) {
                    let exchange = req.exchange().as_str_name().to_lowercase();
                    let exchange = Arc::new(exchange);
                    let symbol = Arc::new(req.symbol.clone());
                    let size = size.get(exchange.clone(), symbol.clone()).await.unwrap_or(0);
                    let cur = cur.get(exchange.clone(), symbol.clone()).await.unwrap_or(0);
                    let prog = Progress {
                      exchange: req.exchange().into(),
                      symbol: symbol.as_ref().clone(),
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

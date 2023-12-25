use ::std::sync::Arc;

use ::futures::stream::BoxStream;
use ::futures::{SinkExt, StreamExt};
use ::mongodb::Database;
use ::serde_json::{from_slice as parse_json, to_string as jsonify};
use ::tokio::select;
use ::warp::filters::BoxedFilter;
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
use ::rpc::status_check_request::StatusCheckRequest;
use ::subscribe::nats::Client as Nats;
use ::symbols::binance::recorder::SymbolWriter as BinanceSymbolWriter;

use crate::context::Context;
use crate::errors::ServiceResult;
use crate::services::SocketResponseService;

type ProgressKVS =
  Arc<dyn Get<Commands = MultiplexedConnection, Value = i64> + Send + Sync>;

#[derive(Clone)]
pub struct Service {
  context: Arc<Context>,
}

impl Service {
  pub async fn new(
    nats: &Nats,
    redis_cli: &redis::Client,
    db: &Database,
  ) -> ServiceResult<Self> {
    let status = Arc::new(FetchStatusEventPubSub::new(nats).await?);
    let splitter = Arc::new(HistChartDateSplitPubSub::new(nats).await?);
    let redis_con: KVSResult<MultiplexedConnection> = redis_cli
      .get_multiplexed_async_connection()
      .await
      .map_err(|err| err.into());
    let redis_con = redis_con?;
    let num_obj_kvs =
      Arc::new(NUM_TO_FETCH_KVS_BUILDER.build(redis_con.clone()));
    let sync_prog_kvs = Arc::new(CUR_SYNC_PROG_KVS_BUILDER.build(redis_con));
    let symbol_reader = Arc::new(BinanceSymbolWriter::new(db).await);
    let soc_resp = Arc::new(SocketResponseService::new(
      num_obj_kvs.clone(),
      sync_prog_kvs.clone(),
    ));
    let context = Arc::new(Context::new(
      num_obj_kvs,
      sync_prog_kvs,
      status,
      splitter,
      symbol_reader,
      soc_resp,
    ));
    let ret = Self { context };
    return Ok(ret);
  }

  pub fn route(&self) -> BoxedFilter<(impl Reply,)> {
    return self.websocket();
  }

  fn websocket(&self) -> BoxedFilter<(impl Reply,)> {
    let ctx = self.context.clone();
    return ::warp::path("subscribe")
      .and_then(||async {
        return Ok((ctx.clone(), ctx.get_init_prog_stream(Exchanges::Binance).await?));
      })
      .untuple_one()
      .and(::warp::ws())
      .map(|
        ctx: Arc<Context>,
        start_prog: BoxStream<'_, Message>,
        ws: Ws
      | {
        return ws.on_upgrade(|mut sock: WebSocket| async move {
          while let Some(payload) = start_prog.next().await {
            let _ = sock.feed(payload).await;
          }
          let _ = sock.flush().await;
          let subsc = ctx.status.pull_subscribe("historyService").await;
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
                  ctx.socket_response.handle(&item, Box::pin(sock)).await;
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
                    let exchange = req.exchange.as_str().to_lowercase();
                    let exchange = Arc::new(exchange);
                    let symbol = Arc::new(req.symbol.clone());
                    let size = size.get(exchange.clone(), symbol.clone()).await.unwrap_or(0);
                    let cur = cur.get(exchange.clone(), symbol.clone()).await.unwrap_or(0);
                    let prog = Progress {
                      exchange: req.exchange,
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

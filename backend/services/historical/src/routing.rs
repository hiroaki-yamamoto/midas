use ::std::sync::Arc;

use ::futures::{SinkExt, StreamExt};
use ::kvs::redis::aio::MultiplexedConnection;
use ::log::error;
use ::mongodb::Database;
use ::tokio::select;
use ::warp::filters::BoxedFilter;
use ::warp::ws::{Message, WebSocket, Ws};
use ::warp::{Filter, Reply};

use ::errors::KVSResult;
use ::history::kvs::{CUR_SYNC_PROG_KVS_BUILDER, NUM_TO_FETCH_KVS_BUILDER};
use ::history::pubsub::{FetchStatusEventPubSub, HistChartDateSplitPubSub};
use ::kvs::redis;
use ::rpc::exchanges::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::symbols::binance::recorder::SymbolWriter as BinanceSymbolWriter;

use crate::context::Context;
use crate::errors::ServiceResult;
use crate::services::{SocketRequestService, SocketResponseService};

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
    let sock_req = Arc::new(SocketRequestService::new(splitter.clone()));
    let context = Arc::new(Context::new(
      num_obj_kvs,
      sync_prog_kvs,
      status,
      splitter,
      symbol_reader,
      soc_resp,
      sock_req,
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
      .map(move || {
        return ctx.clone();
      })
      .and(::warp::ws())
      .map(
        |ctx: Arc<Context>, ws: Ws| {
          return ws.on_upgrade(|mut sock: WebSocket| async move {
            match ctx.get_init_prog_stream(Exchanges::Binance).await {
              Ok(mut start_prog) => {
                while let Some(payload) = start_prog.next().await {
                  let _ = sock.feed(payload).await;
                  let _ = sock.flush().await;
                }
              }
              Err(err) => {
                error!(error: err = err; "Failed to get progress for initalization.");
              }
            }
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
                    if let Err(e) = ctx.socket_response.handle(&item, &mut sock).await {
                      error!(error: err = e; "Failed send the progress response");
                    }
                  },
                  Some(Ok(msg)) = sock.next() => {
                    if msg.is_close() {
                      break;
                    }
                    if let Err(e) = ctx.socket_request.handle(&msg).await {
                      error!(error: err = e; "Failed to handle the request.");
                    }
                  },
                }
              },
            };
            let _ = sock.close().await;
          });
        },
      )
      .boxed();
  }
}

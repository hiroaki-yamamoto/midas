pub mod entities;
pub(crate) mod handlers;
mod pubsub;

use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::try_join_all;
use ::futures::stream::{BoxStream, StreamExt};
use ::futures::FutureExt;
use ::log::{as_error, warn};
use ::tokio::signal::unix::Signal;
use ::tokio::sync::watch::{channel, Receiver};
use ::tokio::sync::RwLock;
use ::tokio::time::interval;
use ::tokio::{join, select, spawn};
use ::uuid::Uuid;

use ::entities::{BookTicker as CommonBookTicker, TradeObserverControlEvent};
use ::errors::{CreateStreamResult, ObserverResult};
use ::kvs::redis::Commands as RedisCommands;
use ::kvs::traits::last_checked::ListOp;
use ::kvs::Connection;
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;

use crate::kvs::ObserverNodeKVS;
use crate::pubsub::{NodeControlEventPubSub, NodeEventPubSub};
use crate::traits::{
  TradeObserver as TradeObserverTrait, TradeSubscriber as TradeSubscriberTrait,
};

use self::handlers::trade::BookTickerHandler;
use self::pubsub::BookTickerPubSub;

const SUBSCRIBE_DELAY: Duration = Duration::from_secs(1);

pub struct TradeObserver<T>
where
  T: RedisCommands + Send + Sync + 'static,
{
  node_id: Option<Uuid>,
  kvs: ObserverNodeKVS<T>,
  control_event: NodeControlEventPubSub,
  node_event: NodeEventPubSub,
  trade_handler: BookTickerHandler,
  symbol_to_add: Vec<String>,
  symbol_to_del: Vec<String>,
}

impl<T> TradeObserver<T>
where
  T: RedisCommands + Send + Sync,
{
  pub async fn new(
    broker: &Nats,
    redis_cmd: Connection<T>,
  ) -> ObserverResult<Self> {
    let control_event = NodeControlEventPubSub::new(broker).await?;
    let node_event = NodeEventPubSub::new(broker).await?;
    let trade_handler = BookTickerHandler::new(broker).await?;

    let kvs = ObserverNodeKVS::new(redis_cmd.into());
    let me = Self {
      node_id: None,
      trade_handler,
      kvs,
      control_event,
      node_event,
      symbol_to_add: Vec::new(),
      symbol_to_del: Vec::new(),
    };
    return Ok(me);
  }

  async fn handle_control_event(
    me: Arc<RwLock<Self>>,
    mut signal: Receiver<Option<()>>,
  ) -> ObserverResult<()> {
    let control_event = async {
      let me = me.read().await;
      me.control_event.clone()
    }
    .await;
    let mut control_event =
      control_event.pull_subscribe("biannceTradeObserver").await?;
    loop {
      select! {
        _ = signal.changed() => {
          break;
        }
        Some((event, _)) = control_event.next() => {
          match event {
            TradeObserverControlEvent::NodeIDAssigned(node_id) => {
              let me = me.read().await;
              warn!(
                req_node_id = node_id.to_string(),
                node_id = me.node_id.map(|id| id.to_string());
                "Received Node ID Assigned event that is not recognized.",
              );
              continue;
            }
            TradeObserverControlEvent::SymbolAdd(exchange, symbol) => {
              if exchange != Exchanges::Binance {
                continue;
              }
              let mut me = me.write().await;
              me.symbol_to_add.push(symbol);
            }
            TradeObserverControlEvent::SymbolDel(exchange, symbol) => {
              if exchange != Exchanges::Binance {
                continue;
              }
              let mut me = me.write().await;
              me.symbol_to_del.push(symbol);
            }
          }
        }
      }
    }
    return Ok(());
  }

  async fn handle_subscribe(
    me: Arc<RwLock<Self>>,
    mut signal: Receiver<Option<()>>,
  ) -> ObserverResult<()> {
    let mut interval = interval(SUBSCRIBE_DELAY);
    loop {
      select! {
        _ = signal.changed() => {
          break;
        },
        _ = interval.tick() => {
          if async {
            me.read().await.node_id.is_none()
          }.await {
            continue;
          }
          while async {
            let me = me.read().await;
            !me.symbol_to_add.is_empty()
          }.await {
            let to_add: Vec<String> = async {
              let mut me = me.write().await;
              let to_add = &mut me.symbol_to_add;
              if to_add.len() > 10 {
                to_add.drain(..10).collect()
              } else {
                to_add.drain(..).collect()
              }
            }.await;
            let mut me = me.write().await;
            if let Err(e) = me.trade_handler.subscribe(
              to_add.as_slice()
            ).await {
              warn!(error = as_error!(e); "Failed to subscribe");
              continue;
            }
            if let Err(e) = me
              .kvs
              .lpush::<usize>(&me.node_id.unwrap().to_string(), to_add, None)
              .await
            {
              warn!(
                error = as_error!(e);
                "Failed to register the symbols to the node KVS"
              );
              continue;
            }
          }
        },
      };
    }
    return Ok(());
  }

  async fn handle_unsubscribe(
    me: Arc<RwLock<Self>>,
    mut signal: Receiver<Option<()>>,
  ) -> ObserverResult<()> {
    let mut interval = interval(SUBSCRIBE_DELAY);
    loop {
      select! {
        _ = signal.changed() => {
          break;
        },
        _ = interval.tick() => {
          {
            let me = me.read().await;
            if me.node_id.is_none() {
              continue;
            }
          }
          let mut lrem_defer = vec![];
          let to_del: Vec<String> = async {
            let to_del = me.clone();
            let mut to_del = to_del.write().await;
            let to_del = &mut to_del.symbol_to_del;
            to_del.drain(..).collect()
          }.await;

          {
            let mut me = me.write().await;
            if let Err(e) = me.trade_handler.unsubscribe(to_del.as_slice()).await
            {
              warn!(error = as_error!(e); "Failed to unsubscribe");
            };
          }

          lrem_defer.extend(to_del.into_iter().map(|sym| {
            let me = me.clone();
            return async move {
              let me = me.read().await;
              me.kvs
                .lrem::<usize>(me.node_id.unwrap().to_string().as_str(), 0, sym)
                .await
            };
          }));
          if let Err(e) = try_join_all(lrem_defer).await {
            warn!(
              error = as_error!(e);
              "Failed to unregister the symbols from the node KVS"
            );
            continue;
          }
        },
      }
    }
    return Ok(());
  }
}

#[async_trait]
impl<T> TradeObserverTrait for TradeObserver<T>
where
  T: RedisCommands + Send + Sync + 'static,
{
  async fn start(self: Box<Self>, signal: &mut Signal) -> ObserverResult<()> {
    let me = Arc::new(RwLock::new(*self));
    let (signal_tx, signal_rx) = channel::<Option<()>>(None);
    let signal_defer = signal
      .recv()
      .then(|_| async {
        let _ = signal_tx.send(Some(()));
      })
      .boxed();
    let (control_handler, subscribe_handler, unsubscribe_handler) = (
      spawn(Self::handle_control_event(me.clone(), signal_rx.clone())),
      spawn(Self::handle_subscribe(me.clone(), signal_rx.clone())),
      spawn(Self::handle_unsubscribe(me.clone(), signal_rx.clone())),
    );
    let _ = join!(
      signal_defer,
      control_handler,
      subscribe_handler,
      unsubscribe_handler
    );
    return Ok(());
  }
}

#[derive(Clone, Debug)]
pub struct TradeSubscriber {
  pubsub: BookTickerPubSub,
}

impl TradeSubscriber {
  pub async fn new(broker: &Nats) -> CreateStreamResult<Self> {
    let pubsub = BookTickerPubSub::new(&broker).await?;
    return Ok(Self { pubsub });
  }
}

#[async_trait]
impl TradeSubscriberTrait for TradeSubscriber {
  async fn subscribe(&self) -> ObserverResult<BoxStream<'_, CommonBookTicker>> {
    let st = self.pubsub.pull_subscribe("binanceObserver").await?;
    let st = st.map(|(item, _)| {
      let ret: CommonBookTicker = item.into();
      return ret;
    });
    return Ok(st.boxed());
  }
}

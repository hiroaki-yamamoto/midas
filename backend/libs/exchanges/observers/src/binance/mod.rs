pub mod entities;
pub(crate) mod handlers;
mod pubsub;

use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::try_join_all;
use ::futures::stream::{BoxStream, StreamExt};
use ::futures::FutureExt;
use ::log::{as_error, as_serde, info, warn};
use ::tokio::select;
use ::tokio::signal::unix::Signal;
use ::tokio::sync::watch::{channel, Receiver};
use ::tokio::sync::RwLock;
use ::tokio::time::interval;
use ::uuid::Uuid;

use ::entities::{
  BookTicker as CommonBookTicker, TradeObserverControlEvent,
  TradeObserverNodeEvent,
};
use ::errors::{CreateStreamResult, ObserverError, ObserverResult};
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
  symbols_to_add: Vec<String>,
  symbols_to_del: Vec<String>,
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
      trade_handler: trade_handler,
      kvs,
      control_event,
      node_event,
      symbols_to_add: Vec::new(),
      symbols_to_del: Vec::new(),
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
          warn!("Received signal to stop handle_control_event");
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
              me.symbols_to_add.push(symbol);
            }
            TradeObserverControlEvent::SymbolDel(exchange, symbol) => {
              if exchange != Exchanges::Binance {
                continue;
              }
              let mut me = me.write().await;
              me.symbols_to_del.push(symbol);
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
          warn!("Received signal to stop handle_subscribe");
          break;
        },
        _ = interval.tick() => {
          {
            if me.read().await.node_id.is_none() {
              continue;
            }
          }
          let mut symbols_to_add: Vec<String> = {
            let mut me = me.write().await;
            me.symbols_to_add.drain(..).collect()
          };
          info!(symbols = as_serde!(symbols_to_add); "Start subscription");
          while !symbols_to_add.is_empty() {
            let to_add: Vec<String> = async {
              let to_add = &mut symbols_to_add;
              if to_add.len() > 10 {
                to_add.drain(..10).collect()
              } else {
                to_add.drain(..).collect()
              }
            }.await;
            {
              let mut me = me.write().await;
              let trade_handler = &mut me.trade_handler;
              if let Err(e) = trade_handler.subscribe(
                to_add.as_slice()
              ).await {
                warn!(error = as_error!(e); "Failed to subscribe");
                continue;
              }
            }
            if let Err(e) = async {
              let me = me.read().await;
              me.kvs.lpush::<usize>(&me.node_id.unwrap().to_string(), to_add, None).await
            }.await
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
          warn!("Received signal to stop handle_unsubscribe");
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
            let mut to_del = me.write().await;
            let to_del = &mut to_del.symbols_to_del;
            to_del.drain(..).collect()
          }.await;

          {
            let mut me = me.write().await;
            let trade_handler = &mut me.trade_handler;
            if let Err(e) = trade_handler.unsubscribe(to_del.as_slice()).await
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

  async fn request_node_id(me: Arc<RwLock<Self>>) -> ObserverResult<()> {
    let node_event = async {
      let me = me.read().await;
      me.node_event.clone()
    }
    .await;
    info!("Creating empty stream to store node id request payload.");
    let _ = node_event.get_or_create_stream(None).await?;
    info!("Empty Stream created. Requesting node id.");
    let mut response_stream = node_event
      .request::<TradeObserverControlEvent>(TradeObserverNodeEvent::Regist(
        Exchanges::Binance,
      ))
      .await?;
    info!("Node ID request sent. Waiting for response.");
    while let Some((event, _)) = response_stream.next().await {
      if let TradeObserverControlEvent::NodeIDAssigned(id) = event {
        let mut me = me.write().await;
        me.node_id = Some(id);
        info!(node_id = id.to_string(); "Assigned node id.");
      } else {
        warn!(
          "Received unexpected response while waiting for Node ID: {:?}",
          event
        );
      }
    }
    return Ok(());
  }

  async fn ping(
    me: Arc<RwLock<Self>>,
    mut signal: Receiver<Option<()>>,
  ) -> ObserverResult<()> {
    let mut interval = interval(Duration::from_secs(1));
    loop {
      select! {
        _ = signal.changed() => {
          warn!("Received signal to stop ping");
          break;
        },
        _ = interval.tick() => {
          let me = me.read().await;
          if let Some(node_id) = me.node_id {
            let _ = me
              .node_event
              .publish(TradeObserverNodeEvent::Ping(node_id))
              .await;
          }
          continue;
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
  async fn start(self: Box<Self>, signal: Box<Signal>) -> ObserverResult<()> {
    let me = Arc::new(RwLock::new(*self));
    let mut signal = signal;
    let (signal_tx, signal_rx) = channel::<Option<()>>(None);
    let signal_defer = signal
      .recv()
      .then(|_| async {
        let me = me.read().await;
        if let Some(node_id) = me.node_id {
          let _ = me
            .node_event
            .publish(TradeObserverNodeEvent::Unregist(node_id))
            .await;
          info!("Unregistered node id: {}", node_id);
        }
        let _ = signal_tx.send(Some(()));
        return Ok::<(), ObserverError>(());
      })
      .boxed();
    let handle_trade = async {
      let mut me = me.write().await;
      me.trade_handler.start(signal_rx.clone()).await
    }
    .boxed();

    if let Err(e) = try_join_all([
      signal_defer,
      handle_trade,
      Self::ping(me.clone(), signal_rx.clone()).boxed(),
      Self::request_node_id(me.clone()).boxed(),
      Self::handle_control_event(me.clone(), signal_rx.clone()).boxed(),
      Self::handle_subscribe(me.clone(), signal_rx.clone()).boxed(),
      Self::handle_unsubscribe(me.clone(), signal_rx.clone()).boxed(),
    ])
    .await
    {
      return Err(e);
    };
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

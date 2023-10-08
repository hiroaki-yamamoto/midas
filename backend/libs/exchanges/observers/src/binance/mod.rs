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
  symbols_to_add: Arc<RwLock<Vec<String>>>,
  symbols_to_del: Arc<RwLock<Vec<String>>>,
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
      symbols_to_add: Arc::new(RwLock::new(Vec::new())),
      symbols_to_del: Arc::new(RwLock::new(Vec::new())),
    };
    return Ok(me);
  }

  async fn handle_control_event(
    self: &mut Box<Self>,
    mut signal: Receiver<Option<()>>,
  ) -> ObserverResult<()> {
    let mut control_event = self
      .control_event
      .pull_subscribe("biannceTradeObserver")
      .await?;
    loop {
      select! {
        _ = signal.changed() => {
          warn!("Received signal to stop handle_control_event");
          break;
        }
        Some((event, _)) = control_event.next() => {
          match event {
            TradeObserverControlEvent::NodeIDAssigned(node_id) => {
              warn!(
                req_node_id = node_id.to_string(),
                node_id = self.node_id.map(|id| id.to_string());
                "Received Node ID Assigned event that is not recognized.",
              );
              continue;
            }
            TradeObserverControlEvent::SymbolAdd(exchange, symbol) => {
              if exchange != Exchanges::Binance {
                continue;
              }
              {
                self.symbols_to_add.write().await.push(symbol);
              }
            }
            TradeObserverControlEvent::SymbolDel(exchange, symbol) => {
              if exchange != Exchanges::Binance {
                continue;
              }
              {
                self.symbols_to_del.write().await.push(symbol);
              }
            }
          }
        }
      }
    }
    return Ok(());
  }

  async fn handle_subscribe(
    self: &mut Box<Self>,
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
            if self.node_id.is_none() {
              continue;
            }
          }
          let mut symbols_to_add: Vec<String> = {
            self.symbols_to_add.write().await.drain(..).collect()
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
              if let Err(e) = self.trade_handler.subscribe(
                to_add.as_slice()
              ).await {
                warn!(error = as_error!(e); "Failed to subscribe");
                continue;
              }
            }
            if let Err(e) = self.kvs.lpush::<usize>(
              &self.node_id.unwrap().to_string(), to_add, None
            ).await
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
    &self,
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
          if self.node_id.is_none() {
            continue;
          }
          let mut lrem_defer = vec![];
          let to_del: Vec<String> = {
            self.symbols_to_del.write().await.drain(..).collect()
          };

          if let Err(e) = self.trade_handler.unsubscribe(to_del.as_slice()).await
          {
            warn!(error = as_error!(e); "Failed to unsubscribe");
          }
          lrem_defer.extend(to_del.into_iter().map(|sym| {
            return async move {
              self.kvs
                .lrem::<usize>(self.node_id.unwrap().to_string().as_str(), 0, sym)
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

  async fn request_node_id(
    self: &mut Box<Self>,
    mut signal: Receiver<Option<()>>,
  ) -> ObserverResult<()> {
    let node_event = self.node_event.clone();
    info!("Creating empty stream to store node id request payload.");
    let _ = node_event.get_or_create_stream(None).await?;
    info!("Empty Stream created. Requesting node id.");
    let mut response_stream = node_event
      .request::<TradeObserverControlEvent>(TradeObserverNodeEvent::Regist(
        Exchanges::Binance,
      ))
      .await?;
    info!("Node ID request sent. Waiting for response.");
    loop {
      select! {
        _ = signal.changed() => {
          warn!("Received signal to stop handle_unsubscribe");
          break;
        },
        Some((event, _)) = response_stream.next() => {
          if let TradeObserverControlEvent::NodeIDAssigned(id) = event {
            self.node_id = Some(id);
            info!(node_id = id.to_string(); "Assigned node id.");
          } else {
            warn!(
              "Received unexpected response while waiting for Node ID: {:?}",
              event
            );
          }
        },
      }
    }
    return Ok(());
  }

  async fn ping(
    self: &mut Box<Self>,
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
          if let Some(node_id) = self.node_id {
            let _ = self
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
  async fn start(
    self: &'async_trait mut Box<Self>,
    signal: Box<Signal>,
  ) -> ObserverResult<()> {
    let mut signal = signal;
    let (signal_tx, signal_rx) = channel::<Option<()>>(None);
    let signal_defer = signal
      .recv()
      .then(|_| async {
        if let Some(node_id) = self.node_id {
          let _ = self
            .node_event
            .publish(TradeObserverNodeEvent::Unregist(node_id))
            .await;
          info!("Unregistered node id: {}", node_id);
        }
        let _ = signal_tx.send(Some(()));
        return Ok::<(), ObserverError>(());
      })
      .boxed();

    if let Err(e) = try_join_all([
      signal_defer,
      self.trade_handler.start(signal_rx.clone()).boxed(),
      self.ping(signal_rx.clone()).boxed(),
      self.request_node_id(signal_rx.clone()).boxed(),
      self.handle_control_event(signal_rx.clone()).boxed(),
      self.handle_subscribe(signal_rx.clone()).boxed(),
      self.handle_unsubscribe(signal_rx.clone()).boxed(),
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

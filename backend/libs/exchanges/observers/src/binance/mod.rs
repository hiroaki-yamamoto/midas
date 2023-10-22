pub mod entities;
pub(crate) mod handlers;
mod pubsub;

use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::try_join_all;
use ::futures::stream::{BoxStream, StreamExt};
use ::futures::FutureExt;
use ::log::{as_error, as_serde, debug, info, warn};
use ::tokio::select;
use ::tokio::signal::unix::Signal;
use ::tokio::sync::{oneshot, watch, Mutex, RwLock};
use ::tokio::time::interval;

use ::config::Database;
use ::entities::BookTicker as CommonBookTicker;
use ::errors::{CreateStreamResult, ObserverError, ObserverResult};
use ::kvs::redis::AsyncCommands as RedisCommands;
use ::kvs::traits::last_checked::ListOp;
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;

use crate::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use crate::kvs::NODE_KVS_BUILDER;
use crate::pubsub::{NodeControlEventPubSub, NodeEventPubSub};
use crate::services::{Init, NodeIDManager};
use crate::traits::{
  TradeObserver as TradeObserverTrait, TradeSubscriber as TradeSubscriberTrait,
};

use self::handlers::trade::BookTickerHandler;
use self::pubsub::BookTickerPubSub;

const SUBSCRIBE_DELAY: Duration = Duration::from_secs(1);

pub struct TradeObserver<T, KVSType>
where
  T: RedisCommands + Send + Sync + 'static,
  KVSType: ListOp<T, String>,
{
  node_id: Arc<RwLock<Option<String>>>,
  kvs: KVSType,
  control_event: NodeControlEventPubSub,
  node_event: NodeEventPubSub,
  trade_handler: Arc<BookTickerHandler>,
  symbols_to_add: Arc<Mutex<Vec<String>>>,
  symbols_to_del: Arc<Mutex<Vec<String>>>,
  signal_tx: Arc<watch::Sender<bool>>,
  signal_rx: watch::Receiver<bool>,
  node_id_manager: Arc<NodeIDManager<T>>,
  initer: Arc<Init<T>>,
}

impl<T, KVSType> TradeObserver<T, KVSType>
where
  T: RedisCommands + Send + Sync,
  KVSType: ListOp<T, String>,
{
  pub async fn new(
    broker: &Nats,
    redis_cmd: T,
    db: Database,
  ) -> ObserverResult<Self> {
    let control_event = NodeControlEventPubSub::new(broker).await?;
    let node_event = NodeEventPubSub::new(broker).await?;
    let (signal_tx, signal_rx) = watch::channel::<bool>(false);
    let trade_handler = BookTickerHandler::new(broker).await?;
    let node_id_manager = NodeIDManager::new(redis_cmd.clone().into());
    let initer = Init::new(redis_cmd.clone().into(), db, broker).await?;

    let kvs = NODE_KVS_BUILDER.build(redis_cmd);
    let me = Self {
      node_id: Arc::new(RwLock::new(None)),
      trade_handler: trade_handler.into(),
      kvs: kvs.into(),
      control_event,
      node_event,
      signal_tx: Arc::new(signal_tx),
      signal_rx,
      symbols_to_add: Mutex::new(Vec::new()).into(),
      symbols_to_del: Mutex::new(Vec::new()).into(),
      node_id_manager: Arc::new(node_id_manager),
      initer: Arc::new(initer),
    };
    return Ok(me);
  }

  async fn handle_control_event(
    mut signal: watch::Receiver<bool>,
    control_event: NodeControlEventPubSub,
    symbols_to_add: Arc<Mutex<Vec<String>>>,
    symbols_to_del: Arc<Mutex<Vec<String>>>,
    ready: oneshot::Sender<()>,
  ) -> ObserverResult<()> {
    let mut control_event =
      control_event.pull_subscribe("biannceTradeObserver").await?;
    let _ = ready.send(());
    loop {
      select! {
        _ = signal.changed() => {
          warn!("Received signal to stop handle_control_event");
          break;
        }
        Some((event, _)) = control_event.next() => {
          match event {
            TradeObserverControlEvent::SymbolAdd(exchange, symbol) => {
              if exchange != Exchanges::Binance {
                continue;
              }
              info!(symbol = symbol.as_str(); "Received symbol add event.");
              {
                let mut symbols_to_add = symbols_to_add.lock().await;
                symbols_to_add.push(symbol);
              }
            }
            TradeObserverControlEvent::SymbolDel(exchange, symbol) => {
              if exchange != Exchanges::Binance {
                continue;
              }
              info!(symbol = symbol.as_str(); "Received symbol del event.");
              {
                let mut to_del = symbols_to_del.lock().await;
                to_del.push(symbol);
              }
            }
          }
        }
      }
    }
    return Ok(());
  }

  async fn handle_subscribe(
    node_id: Arc<RwLock<Option<String>>>,
    mut signal: watch::Receiver<bool>,
    symbols_to_add: Arc<Mutex<Vec<String>>>,
    trade_handler: Arc<BookTickerHandler>,
    kvs: KVSType,
  ) -> ObserverResult<()> {
    let mut interval = interval(SUBSCRIBE_DELAY);
    loop {
      select! {
        _ = signal.changed() => {
          warn!("Received signal to stop handle_subscribe");
          break;
        },
        _ = interval.tick() => {
          let node_id = { node_id.read().await.clone() };
          if node_id.is_none() {
            continue;
          }
          let mut symbols_to_add: Vec<String> = {
            let mut to_add = symbols_to_add.lock().await;
            to_add.drain(..).collect()
          };
          info!(symbols = as_serde!(symbols_to_add); "Start subscription process");
          while !symbols_to_add.is_empty() {
            let to_add: Vec<String> = {
              let to_add = &mut symbols_to_add;
              if to_add.len() > 10 {
                to_add.drain(..10)
              } else {
                to_add.drain(..)
              }
            }.collect();
            info!(symbols = as_serde!(to_add); "Calling subscribe function");
            if let Err(e) = trade_handler.subscribe(
              to_add.as_slice()
            ).await {
              warn!(error = as_error!(e); "Failed to send subscription signal");
              continue;
            }
            info!(symbols = as_serde!(to_add); "Registered symbols to Websocket");
            if let Err(e) =
              kvs.lpush::<usize>(node_id.clone().unwrap().as_str(), to_add.clone(), None).await
            {
              warn!(
                error = as_error!(e);
                "Failed to register the symbols to the node KVS"
              );
              continue;
            }
            info!(symbols = as_serde!(to_add); "Registered symbol info to kvs");
          }
        },
      };
    }
    return Ok(());
  }

  async fn handle_unsubscribe(
    mut signal: watch::Receiver<bool>,
    node_id_lock: Arc<RwLock<Option<String>>>,
    symbols_to_del: Arc<Mutex<Vec<String>>>,
    trade_handler: Arc<BookTickerHandler>,
    node_kvs: KVSType,
  ) -> ObserverResult<()> {
    let mut interval = interval(SUBSCRIBE_DELAY);
    loop {
      select! {
        _ = signal.changed() => {
          warn!("Received signal to stop handle_unsubscribe");
          break;
        },
        _ = interval.tick() => {
          let node_id = { node_id_lock.read().await.clone() } ;
          if node_id.is_none() {
            continue;
          }
          let mut lrem_defer = vec![];
          let to_del: Vec<String> = {
            let mut to_del = symbols_to_del.lock().await;
            to_del.drain(..).collect()
          };

          if let Err(e) = trade_handler.unsubscribe(to_del.clone()).await
          {
            warn!(error = as_error!(e); "Failed to unsubscribe");
          };

          lrem_defer.extend(to_del.into_iter().map(|sym| {
            let node_id = node_id.clone();
            let node_id = node_id.unwrap();
            let kvs = node_kvs.clone();
            return async move {kvs
                .lrem::<usize>(node_id.as_str(), 0, sym).await};
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
    node_id_manager: Arc<NodeIDManager<T>>,
    node_id_lock: Arc<RwLock<Option<String>>>,
    ready: oneshot::Receiver<()>,
    init: Arc<Init<T>>,
  ) -> ObserverResult<()> {
    let node_id = node_id_manager.register(Exchanges::Binance).await?;
    {
      *node_id_lock.write().await = Some(node_id.clone());
    };
    let _ = ready.await?;
    info!(node_id = node_id; "Registered node id");
    let _ = init.init(Exchanges::Binance).await;
    return Ok(());
  }

  async fn ping(
    node_id: Arc<RwLock<Option<String>>>,
    mut signal_rx: watch::Receiver<bool>,
    node_event: NodeEventPubSub,
  ) -> ObserverResult<()> {
    let mut interval = interval(Duration::from_secs(1));
    loop {
      select! {
        _ = signal_rx.changed() => {
          warn!("Received signal to stop ping");
          break;
        },
        _ = interval.tick() => {
          if let Some(node_id) = { node_id.read().await.clone() } {
            let _ = node_event
              .publish(TradeObserverNodeEvent::Ping(node_id.clone()))
              .await;
            debug!(node_id = node_id; "Ping sent");
          }
          continue;
        },
      }
    }
    return Ok(());
  }
}

#[async_trait]
impl<T, KVSType> TradeObserverTrait for TradeObserver<T, KVSType>
where
  T: RedisCommands + Send + Sync + 'static,
  KVSType: ListOp<T, String>,
{
  async fn start(&self, signal: Box<Signal>) -> ObserverResult<()> {
    let (ready_evloop_tx, ready_evloop_rx) = oneshot::channel();
    let node_id_manager = self.node_id_manager.clone();
    let node_event = self.node_event.clone();
    let node_id_lock = self.node_id.clone();
    let signal_tx = self.signal_tx.clone();
    let signal_defer = async move {
      let mut signal = signal;
      let ret = signal
        .recv()
        .then(|_| async {
          if let Some(node_id) = { node_id_lock.read().await.clone() } {
            let (exchange, symbols) =
              node_id_manager.unregist(&node_id).await?;
            let _ = node_event
              .publish(TradeObserverNodeEvent::Unregist(exchange, symbols))
              .await;
            info!("Unregistered node id: {}", node_id);
            {
              *node_id_lock.write().await = None;
            };
          }
          let _ = signal_tx.send(true);
          return Ok::<(), ObserverError>(());
        })
        .await;
      ret
    };
    let signal_defer = ::tokio::spawn(signal_defer.boxed());
    let ping = ::tokio::spawn(Self::ping(
      self.node_id.clone(),
      self.signal_rx.clone(),
      self.node_event.clone(),
    ));
    let request_node_id = ::tokio::spawn(Self::request_node_id(
      self.node_id_manager.clone(),
      self.node_id.clone(),
      ready_evloop_rx,
      self.initer.clone(),
    ));
    let unsubscribe = ::tokio::spawn(Self::handle_unsubscribe(
      self.signal_rx.clone(),
      self.node_id.clone(),
      self.symbols_to_del.clone(),
      self.trade_handler.clone(),
      self.kvs.clone(),
    ));
    let subscribe = ::tokio::spawn(Self::handle_subscribe(
      self.node_id.clone(),
      self.signal_rx.clone(),
      self.symbols_to_add.clone(),
      self.trade_handler.clone(),
      self.kvs.clone(),
    ));
    let handle_control = ::tokio::spawn(Self::handle_control_event(
      self.signal_rx.clone(),
      self.control_event.clone(),
      self.symbols_to_add.clone(),
      self.symbols_to_del.clone(),
      ready_evloop_tx,
    ));
    let result: ObserverResult<Vec<()>> = try_join_all([
      signal_defer,
      ping,
      request_node_id,
      unsubscribe,
      subscribe,
      handle_control,
    ])
    .await?
    .into_iter()
    .collect();
    let _ = result?;
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

pub mod entities;
pub(crate) mod handlers;
mod pubsub;

use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::log::{as_error, warn};
use ::tokio::join;
use ::tokio::sync::Mutex;
use ::tokio::time::interval;
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
  T: RedisCommands + Send + Sync,
{
  node_id: Option<Uuid>,
  kvs: ObserverNodeKVS<T>,
  control_event: NodeControlEventPubSub,
  node_event: NodeEventPubSub,
  trade_handler: BookTickerHandler,
  symbol_to_add: Mutex<Vec<String>>,
  symbol_to_del: Mutex<Vec<String>>,
}

impl<T> TradeObserver<T>
where
  T: RedisCommands + Send + Sync,
{
  pub async fn new(
    broker: &Nats,
    redis_cmd: Connection<T>,
  ) -> ObserverResult<Self> {
    let (control_event, node_event, trade_handler) = join!(
      NodeControlEventPubSub::new(broker),
      NodeEventPubSub::new(broker),
      BookTickerHandler::new(broker),
    );

    let (control_event, node_event, trade_handler) =
      (control_event?, node_event?, trade_handler?);
    let kvs = ObserverNodeKVS::new(redis_cmd.into());
    let me = Self {
      node_id: None,
      trade_handler,
      kvs,
      control_event,
      node_event,
      symbol_to_add: Mutex::new(Vec::new()),
      symbol_to_del: Mutex::new(Vec::new()),
    };
    return Ok(me);
  }

  async fn handle_control_event(&mut self) -> ObserverResult<()> {
    let mut control_event = self
      .control_event
      .pull_subscribe("biannceTradeObserver")
      .await?;
    while let Some((event, _)) = control_event.next().await {
      match event {
        TradeObserverControlEvent::NodeIDAssigned(node_id) => {
          self.node_id = Some(node_id);
        }
        TradeObserverControlEvent::SymbolAdd(exchange, symbol) => {
          if exchange != Exchanges::Binance {
            continue;
          }
          let mut symbol_add = self.symbol_to_add.lock().await;
          symbol_add.push(symbol);
        }
        TradeObserverControlEvent::SymbolDel(exchange, symbol) => {
          if exchange != Exchanges::Binance {
            continue;
          }
          let mut symbol_del = self.symbol_to_del.lock().await;
          symbol_del.push(symbol);
        }
      }
    }
    return Ok(());
  }

  async fn handle_subscribe(&mut self) {
    let mut interval = interval(SUBSCRIBE_DELAY);
    loop {
      let _ = interval.tick().await;
      if self.node_id.is_none() {
        continue;
      }
      let mut to_add = self.symbol_to_add.lock().await;
      while !to_add.is_empty() {
        let to_add: Vec<String> = if to_add.len() > 10 {
          to_add.drain(..10)
        } else {
          to_add.drain(..)
        }
        .collect();
        if let Err(e) = self.trade_handler.subscribe(to_add.as_slice()).await {
          warn!(error = as_error!(e); "Failed to subscribe");
          continue;
        };
        if let Err(e) = self
          .kvs
          .lpush::<usize>(&self.node_id.unwrap().to_string(), to_add, None)
          .await
        {
          warn!(
            error = as_error!(e);
            "Failed to register the symbols to the node KVS"
          );
          continue;
        };
      }
    }
  }

  async fn handle_unsubscribe(&mut self) {
    let mut interval = interval(SUBSCRIBE_DELAY);
    loop {
      let _ = interval.tick().await;
      if self.node_id.is_none() {
        continue;
      }
      let mut to_del = self.symbol_to_del.lock().await;
      while !to_del.is_empty() {
        let to_del: Vec<String> = if to_del.len() > 10 {
          to_del.drain(..10)
        } else {
          to_del.drain(..)
        }
        .collect();
        if let Err(e) = self.trade_handler.unsubscribe(to_del.as_slice()).await
        {
          warn!(error = as_error!(e); "Failed to subscribe");
        };
      }
    }
  }
}

#[async_trait]
impl<T> TradeObserverTrait for TradeObserver<T>
where
  T: RedisCommands + Send + Sync,
{
  async fn start(&self) -> ObserverResult<()> {
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

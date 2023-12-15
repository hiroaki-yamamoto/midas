pub mod entities;
pub(crate) mod interfaces;
mod pubsub;
pub(crate) mod sockets;

use ::std::collections::HashMap;
use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::rug::Float;
use ::subscribe::nats::client::Client as Nats;
use ::tokio::signal::unix::Signal;

use ::entities::BookTicker as CommonBookTicker;
use ::errors::{CreateStreamResult, ObserverResult};
use ::subscribe::PubSub;

use crate::binance::{
  entities::BookTicker, interfaces::IBookTickerSubscription,
  pubsub::BookTickerPubSub,
};
use crate::traits::{ITradeObserver, ITradeSubscriber};

pub struct TradeObserver {
  pubsub: Arc<dyn PubSub<Output = BookTicker<Float>> + Send + Sync>,
  sockets: HashMap<u64, Arc<dyn IBookTickerSubscription + Send + Sync>>,
}

#[async_trait]
impl ITradeObserver for TradeObserver {
  async fn start(&self, signal: Box<Signal>) -> ObserverResult<()> {
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
impl ITradeSubscriber for TradeSubscriber {
  async fn subscribe(&self) -> ObserverResult<BoxStream<'_, CommonBookTicker>> {
    let st = self.pubsub.pull_subscribe("binanceObserver").await?;
    let st = st.map(|(item, _)| {
      let ret: CommonBookTicker = item.into();
      return ret;
    });
    return Ok(st.boxed());
  }
}

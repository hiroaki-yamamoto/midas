use ::std::collections::HashMap;
use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::rug::Float;
use ::subscribe::nats::client::Client as Nats;
use ::tokio::signal::unix::Signal;

use ::errors::{CreateStreamResult, ObserverResult};
use ::subscribe::PubSub;

use crate::binance::{
  entities::BookTicker, interfaces::IBookTickerSubscription,
  pubsub::BookTickerPubSub,
};
use crate::traits::ITradeObserver;

pub struct TradeObserver {
  pubsub: Arc<dyn PubSub<Output = BookTicker<Float>> + Send + Sync>,
  sockets: HashMap<u64, Arc<dyn IBookTickerSubscription + Send + Sync>>,
}

impl TradeObserver {
  pub async fn new(broker: &Nats) -> CreateStreamResult<Self> {
    let pubsub = BookTickerPubSub::new(&broker).await?;
    return Ok(Self {
      pubsub: Arc::new(pubsub),
      sockets: HashMap::new(),
    });
  }
}

#[async_trait]
impl ITradeObserver for TradeObserver {
  async fn start(&self, signal: Box<Signal>) -> ObserverResult<()> {
    return Ok(());
  }
}

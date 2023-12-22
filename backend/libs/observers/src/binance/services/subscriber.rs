use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::subscribe::nats::client::Client as Nats;

use ::entities::BookTicker as CommonBookTicker;
use ::errors::{CreateStreamResult, ObserverResult};
use ::subscribe::PubSub;

use crate::binance::pubsub::BookTickerPubSub;
use crate::traits::ITradeSubscriber;

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

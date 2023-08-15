use ::async_trait::async_trait;
use ::futures::stream::BoxStream;

use ::entities::BookTicker;
use ::errors::ObserverResult;

#[async_trait]
pub trait TradeObserver {
  async fn start(&self) -> ObserverResult<()>;
  async fn subscribe(&self) -> ObserverResult<BoxStream<'_, BookTicker>>;
}

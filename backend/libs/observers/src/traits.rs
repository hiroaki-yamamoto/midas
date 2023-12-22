use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::tokio::signal::unix::Signal;

use ::entities::BookTicker;
use ::errors::ObserverResult;

#[async_trait]
pub trait ITradeObserver {
  async fn start(&mut self, signal: &mut Signal) -> ObserverResult<()>;
}

#[async_trait]
pub trait ITradeSubscriber {
  async fn subscribe(&self) -> ObserverResult<BoxStream<'_, BookTicker>>;
}

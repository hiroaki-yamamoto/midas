use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::tokio::signal::unix::Signal;

use ::entities::BookTicker;
use ::errors::ObserverResult;

#[async_trait]
pub trait TradeObserver {
  async fn start(self: Box<Self>, signal: Box<Signal>) -> ObserverResult<()>;
}

#[async_trait]
pub trait TradeSubscriber {
  async fn subscribe(&self) -> ObserverResult<BoxStream<'_, BookTicker>>;
}

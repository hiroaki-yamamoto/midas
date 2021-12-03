use ::async_trait::async_trait;
use ::futures::stream::BoxStream;

use ::entities::BookTicker;
use ::types::GenericResult;

#[async_trait]
pub trait TradeObserver {
  async fn start(&self) -> GenericResult<()>;
  async fn subscribe(&self) -> ::std::io::Result<BoxStream<'_, BookTicker>>;
}

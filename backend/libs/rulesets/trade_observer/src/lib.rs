use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::futures_core::stream::BoxStream;

use ::entities::BookTicker;
use ::types::GenericResult;

#[async_trait]
pub trait TradeObserver {
  async fn start(&self) -> GenericResult<()>;
  async fn subscribe(&self) -> ::std::io::Result<BoxStream<'_, BookTicker>>;
}

pub trait TradeDateTime {
  fn symbol(&self) -> String;
  fn open_time(&self) -> DateTime<Utc>;
  fn close_time(&self) -> DateTime<Utc>;
}

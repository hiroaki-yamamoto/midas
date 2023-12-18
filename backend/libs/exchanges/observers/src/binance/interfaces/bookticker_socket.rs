use ::async_trait::async_trait;
use ::errors::ObserverResult;
use ::futures::Stream;
use ::rug::Float;

use crate::binance::entities::BookTicker;

#[async_trait]
pub trait IBookTickerSocket: Stream<Item = BookTicker<Float>> + Unpin {
  fn has_symbol(&self, symbol: &str) -> bool;
  async fn subscribe(&mut self, symbols: &[String]) -> ObserverResult<()>;
  async fn unsubscribe(&mut self, symbols: &[String]) -> ObserverResult<()>;
  fn len(&self) -> usize;
  fn len_socket(&self) -> usize;
}

pub type BookTickerStream = Box<dyn IBookTickerSocket + Send>;

use ::std::pin::Pin;

use ::async_trait::async_trait;
use ::errors::ObserverResult;
use ::futures::Stream;
use ::rug::Float;

use ::round_robin_client::entities::WSMessageDetail;

use crate::binance::entities::BookTicker;

#[async_trait]
pub trait IBookTickerSocket:
  Stream<Item = WSMessageDetail<BookTicker<Float>>> + Unpin
{
  fn has_symbol(&self, symbol: &str) -> bool;
  async fn subscribe(&mut self, symbols: &[String]) -> ObserverResult<()>;
  async fn unsubscribe(&mut self, symbols: &[String]) -> ObserverResult<()>;
  fn len(&self) -> usize;
}

pub type BookTickerStream = Pin<Box<dyn IBookTickerSocket + Send>>;

use ::async_trait::async_trait;
use ::errors::ObserverResult;

#[async_trait]
pub trait IBookTickerSubscription {
  fn has_symbol(&self, symbol: &str) -> bool;
  async fn subscribe(&mut self, symbols: &[String]) -> ObserverResult<()>;
  async fn unsubscribe(&mut self, symbols: &[String]) -> ObserverResult<()>;
  fn len(&self) -> usize;
  fn len_socket(&self) -> usize;
}

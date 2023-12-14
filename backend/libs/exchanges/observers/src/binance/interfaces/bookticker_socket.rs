use ::async_trait::async_trait;
use ::errors::ObserverResult;

#[async_trait]
pub trait IBookTickerSubscription {
  async fn has_symbol(&self, symbol: &str) -> ObserverResult<bool>;
  async fn subscribe(&mut self, symbols: &[String]) -> ObserverResult<()>;
  async fn unsubscribe(&mut self, symbols: &[String]) -> ObserverResult<()>;
}

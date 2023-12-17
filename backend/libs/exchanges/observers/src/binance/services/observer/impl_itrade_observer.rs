use ::async_trait::async_trait;
use ::tokio::signal::unix::Signal;

use ::errors::ObserverResult;

use crate::traits::ITradeObserver;

use super::TradeObserver;

#[async_trait]
impl ITradeObserver for TradeObserver {
  async fn start(&self, signal: Box<Signal>) -> ObserverResult<()> {
    return Ok(());
  }
}

use ::async_trait::async_trait;
use ::types::ThreadSafeResult;

use crate::entities::KlinesByExchange;

#[async_trait]
pub trait HistoryWriter {
  async fn write(&self, klines: KlinesByExchange) -> ThreadSafeResult<()>;
}

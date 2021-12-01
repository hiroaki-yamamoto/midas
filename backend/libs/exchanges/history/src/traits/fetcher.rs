use ::async_trait::async_trait;
use ::entities::HistoryFetchRequest;

use ::types::ThreadSafeResult;

use super::traits::Kline;

#[async_trait]
pub trait HistoryFetcher {
  // type Kline: Kline;
  async fn fetch(
    &self,
    req: &HistoryFetchRequest,
  ) -> ThreadSafeResult<Vec<Box<dyn Kline + Send + Sync>>>;
}

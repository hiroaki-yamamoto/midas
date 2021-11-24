use ::async_trait::async_trait;
use ::entities::HistoryFetchRequest;

use ::types::ThreadSafeResult;

#[async_trait]
pub trait HistoryFetcher {
  type Klines: Clone;
  async fn fetch(
    &self,
    req: &HistoryFetchRequest,
  ) -> ThreadSafeResult<Self::Klines>;
}

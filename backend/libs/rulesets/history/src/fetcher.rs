use ::async_trait::async_trait;
use ::entities::HistoryFetchRequest;

use ::types::ThreadSafeResult;

use super::traits::KlineTrait;

#[async_trait]
pub trait HistoryFetcher {
  async fn fetch<T>(
    &self,
    req: &HistoryFetchRequest,
  ) -> ThreadSafeResult<Vec<T>>
  where
    T: KlineTrait + Clone;
}

use ::async_trait::async_trait;
use ::entities::HistoryFetchRequest;

use ::reqwest::Error;

use super::traits::KlineTrait;

#[async_trait]
pub trait HistoryFetcher {
  async fn fetch<T>(&self, req: &HistoryFetchRequest) -> Result<Vec<T>, Error>
  where
    T: KlineTrait + Clone;
}

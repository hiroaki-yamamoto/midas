use super::kline::Kline as KlineTrait;
use ::async_trait::async_trait;
use ::types::ThreadSafeResult;

#[async_trait]
pub trait HistoryWriter {
  async fn write(
    &self,
    klines: Vec<Box<dyn KlineTrait + Send + Sync>>,
  ) -> ThreadSafeResult<()>;
}

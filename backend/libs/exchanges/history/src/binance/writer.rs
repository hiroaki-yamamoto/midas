use crate::traits::traits::Kline as KlineTrait;
use crate::traits::HistoryWriter as HistoryWriterTrait;
use ::async_trait::async_trait;
use ::mongodb::Collection;
use ::types::ThreadSafeResult;

use super::entities::Kline;

pub struct HistoryWriter {
  col: Collection<Kline>,
}

#[async_trait]
impl HistoryWriterTrait for HistoryWriter {
  async fn write(
    &self,
    klines: Vec<Box<dyn KlineTrait + Send + Sync>>,
  ) -> ThreadSafeResult<()> {
    return Ok(());
  }
}

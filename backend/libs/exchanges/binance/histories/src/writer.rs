use ::async_trait::async_trait;
use ::history::traits::Kline as KlineTrait;
use ::history::HistoryWriter as HistoryWriterTrait;
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
    let klines: Vec<Kline> = klines.into_iter().collect();
    return Ok(());
  }
}

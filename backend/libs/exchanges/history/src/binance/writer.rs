use ::async_trait::async_trait;
use ::mongodb::Collection;
use ::types::ThreadSafeResult;

use super::entities::Kline;
use crate::entities::KlinesByExchange;
use crate::traits::HistoryWriter as HistoryWriterTrait;

pub struct HistoryWriter {
  col: Collection<Kline>,
}

#[async_trait]
impl HistoryWriterTrait for HistoryWriter {
  async fn write(&self, klines: KlinesByExchange) -> ThreadSafeResult<()> {
    return Ok(());
  }
}

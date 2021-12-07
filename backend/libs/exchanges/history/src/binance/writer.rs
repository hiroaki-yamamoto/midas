use ::std::convert::TryFrom;

use ::async_trait::async_trait;
use ::futures::future::try_join_all;
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::{doc, to_document};
use ::mongodb::options::UpdateModifications;
use ::mongodb::options::UpdateOptions;
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
    let klines = Vec::<Kline>::try_from(klines)?;
    let mut defers = vec![];
    for kline in klines {
      let kline_doc = to_document(&kline)?;
      defers.push(self.col.update_one(
        doc! {"_id": kline.id},
        UpdateModifications::Document(kline_doc),
        UpdateOptions::builder().upsert(true).build(),
      ));
    }
    try_join_all(defers).await?;
    return Ok(());
  }
}

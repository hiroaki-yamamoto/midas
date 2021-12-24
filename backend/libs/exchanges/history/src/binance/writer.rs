use ::std::convert::TryFrom;

use ::async_trait::async_trait;
use ::futures::future::{try_join_all, FutureExt};
use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::{doc, to_document, Document};
use ::mongodb::error::Result as MongoResult;
use ::mongodb::options::{UpdateModifications, UpdateOptions};
use ::mongodb::{Collection, Database};
use ::types::ThreadSafeResult;

use super::entities::Kline;
use crate::entities::KlinesByExchange;
use crate::traits::HistoryWriter as HistoryWriterTrait;

#[derive(Debug, Clone)]
pub struct HistoryWriter {
  col: Collection<Kline>,
}

impl HistoryWriter {
  pub fn new(db: &Database) -> Self {
    return Self {
      col: db.collection("binance.klines"),
    };
  }
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
  async fn list(
    self,
    query: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> MongoResult<BoxStream<'async_trait, KlinesByExchange>> {
    let st = self
      .col
      .find(query, None)
      .map(|cur_res| {
        cur_res.map(|cur| {
          cur
            .filter_map(|kline| async { kline.ok() })
            .map(|kline| KlinesByExchange::Binance(vec![kline]))
            .boxed()
        })
      })
      .await;
    return st;
  }
}

use ::std::convert::TryFrom;

use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::{doc, Document};
use ::mongodb::error::Result as MongoResult;
use ::mongodb::results::{DeleteResult, InsertManyResult};
use ::mongodb::{Collection, Database};

use ::errors::WriterResult;
use ::writers::DatabaseWriter;

use super::entities::Kline;
use crate::entities::KlinesByExchange;
use crate::traits::HistoryWriter as HistoryWriterTrait;

#[derive(Debug, Clone)]
pub struct HistoryWriter {
  col: Collection<Kline>,
  db: Database,
}

impl HistoryWriter {
  pub async fn new(db: &Database) -> Self {
    let me = Self {
      col: db.collection("binance.klines"),
      db: db.clone(),
    };
    me.update_indices(&["symbol"]).await;
    return me;
  }
}

#[async_trait]
impl DatabaseWriter for HistoryWriter {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return self.col.name();
  }
}

#[async_trait]
impl HistoryWriterTrait for HistoryWriter {
  async fn delete_by_symbol(&self, symbol: &str) -> MongoResult<DeleteResult> {
    return self.col.delete_many(doc! {"symbol": symbol}).await;
  }

  async fn write(
    &self,
    klines: KlinesByExchange,
  ) -> WriterResult<InsertManyResult> {
    let klines = Vec::<Kline>::try_from(klines)?;
    return Ok(self.col.insert_many(klines).await?);
  }

  async fn list(
    self,
    query: Document,
  ) -> MongoResult<BoxStream<'async_trait, KlinesByExchange>> {
    let cur = self.col.find(query).await?;
    let st = cur
      .filter_map(|kline| async move { kline.ok() })
      .map(|kline| KlinesByExchange::Binance(vec![kline]))
      .boxed();
    return Ok(st);
  }
}

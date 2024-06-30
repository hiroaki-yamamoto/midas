use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::mongodb::bson::{doc, Document};
use ::mongodb::error::Result as DBResult;
use ::mongodb::results::InsertManyResult;
use ::mongodb::{Collection, Database};

use ::writers::DatabaseWriter as DBWriterTrait;

use crate::traits::SymbolReader as SymbolReaderTrait;
use crate::types::ListSymbolStream;

use super::entities::Symbol;

pub(crate) type InHouseListSymbolStream<'a> = BoxStream<'a, Symbol>;

#[derive(Debug, Clone)]
pub struct SymbolWriter {
  col: Collection<Symbol>,
  db: Database,
}

impl SymbolWriter {
  pub async fn new(db: &Database) -> Self {
    let ret = Self {
      col: (&db).collection("binance.symbol"),
      db: db.clone(),
    };
    ret.update_indices(&["symbol"]).await;
    return ret;
  }

  pub(crate) async fn list(
    &self,
    query: Document,
  ) -> DBResult<InHouseListSymbolStream> {
    let cur = self.col.find(query).await?;
    let cur = cur.filter_map(|doc| async { doc.ok() }).boxed();
    return Ok(cur);
  }

  pub(crate) async fn update_symbols(
    &self,
    value: Vec<Symbol>,
  ) -> DBResult<InsertManyResult> {
    let _ = self.col.delete_many(doc! {}).await?;
    return Ok(self.col.insert_many(value.into_iter()).await?);
  }
}

impl DBWriterTrait for SymbolWriter {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return &self.col.name();
  }
}

#[async_trait]
impl SymbolReaderTrait for SymbolWriter {
  async fn list_all(&self) -> DBResult<ListSymbolStream> {
    let cur = self.col.find(doc! {}).await?;
    let cur = cur
      .filter_map(|doc_res| async { doc_res.ok() })
      .map(|doc| doc.into());
    return Ok(cur.boxed());
  }

  async fn list_trading(&self) -> DBResult<ListSymbolStream> {
    return Ok(
      self
        .col
        .find(doc! {"status": "TRADING"})
        .await?
        .filter_map(|doc_res| async { doc_res.ok() })
        .map(|item| item.into())
        .boxed(),
    );
  }

  async fn list_base_currencies(&self) -> DBResult<Vec<String>> {
    return Ok(
      self
        .col
        .distinct("quoteAsset", doc! {})
        .await?
        .into_iter()
        .filter_map(|base_bson| base_bson.as_str().map(|base| base.to_string()))
        .collect(),
    );
  }
}

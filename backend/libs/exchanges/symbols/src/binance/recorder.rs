use ::async_trait::async_trait;
use ::futures::stream::StreamExt;
use ::mongodb::bson;
use ::mongodb::results::InsertManyResult;
use ::mongodb::{Collection, Database};

use ::types::ThreadSafeResult;

use super::entities::{ListSymbolStream, Symbol};
use ::symbol_recorder::SymbolRecorder as SymbolRecorderTrait;
use ::writers::DatabaseWriter as DBWriterTrait;

#[derive(Debug, Clone)]
pub struct SymbolRecorder {
  col: Collection<Symbol>,
  db: Database,
}

impl SymbolRecorder {
  pub async fn new(db: Database) -> Self {
    let ret = Self {
      col: (&db).collection("binance.symbol"),
      db,
    };
    ret.update_indices(&["symbol"]).await;
    return ret;
  }
}

impl DBWriterTrait for SymbolRecorder {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return &self.col.name();
  }
}

#[async_trait]
impl SymbolRecorderTrait for SymbolRecorder {
  type ListStream = ListSymbolStream<'static>;
  type Type = Symbol;
  async fn list(
    &self,
    query: impl Into<Option<bson::Document>> + Send + 'async_trait,
  ) -> ThreadSafeResult<Self::ListStream> {
    let cur = self.col.find(query, None).await?;
    let cur = cur.filter_map(|doc| async { doc.ok() }).boxed();
    return Ok(cur as Self::ListStream);
  }
  async fn update_symbols(
    &self,
    value: Vec<Self::Type>,
  ) -> ThreadSafeResult<InsertManyResult> {
    let _ = self.col.delete_many(bson::doc! {}, None).await?;
    return Ok(self.col.insert_many(value.into_iter(), None).await?);
  }
  async fn list_base_currencies(&self) -> ThreadSafeResult<Vec<String>> {
    return Ok(
      self
        .col
        .distinct("quoteAsset", None, None)
        .await?
        .into_iter()
        .filter_map(|base_bson| base_bson.as_str().map(|base| base.to_string()))
        .collect(),
    );
  }
}

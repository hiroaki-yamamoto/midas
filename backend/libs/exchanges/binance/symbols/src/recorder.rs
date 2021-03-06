use ::async_trait::async_trait;
use ::futures::stream::StreamExt;
use ::mongodb::bson;
use ::mongodb::results::InsertManyResult;
use ::mongodb::{Collection, Database};
use ::serde::Serialize;

use ::types::ThreadSafeResult;

use super::entities::{ListSymbolStream, Symbol};
use ::base_recorder::Recorder as RecorderTrait;
use ::symbol_recorder::SymbolRecorder as SymbolRecorderTrait;

#[derive(Debug, Clone)]
pub struct SymbolRecorder {
  col: Collection,
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

impl RecorderTrait for SymbolRecorder {
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
  async fn list(
    &self,
    query: impl Into<Option<bson::Document>> + Send + 'async_trait,
  ) -> ThreadSafeResult<Self::ListStream> {
    let cur = self.col.find(query, None).await?;
    let cur = cur
      .filter_map(|doc| async { doc.ok() })
      .map(|doc| bson::from_bson::<Symbol>(bson::Bson::Document(doc)))
      .filter_map(|doc| async { doc.ok() })
      .boxed();
    return Ok(cur as Self::ListStream);
  }
  async fn update_symbols<T>(
    &self,
    value: Vec<T>,
  ) -> ThreadSafeResult<InsertManyResult>
  where
    T: Serialize + Send,
  {
    let empty = bson::Array::new();
    let serialized: Vec<bson::Document> = bson::to_bson(&value)?
      .as_array()
      .unwrap_or(&empty)
      .into_iter()
      .filter_map(|item| item.as_document())
      .map(|item| item.clone())
      .collect();
    let _ = self.col.delete_many(bson::doc! {}, None).await?;
    return Ok(self.col.insert_many(serialized.into_iter(), None).await?);
  }
}

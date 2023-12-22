use ::async_trait::async_trait;
use ::mongodb::error::Result as DBResult;

use crate::types::ListSymbolStream;

#[async_trait]
pub trait SymbolReader {
  async fn list_all(&self) -> DBResult<ListSymbolStream>; // will remove
  async fn list_trading(&self) -> DBResult<ListSymbolStream>;
  async fn list_base_currencies(&self) -> DBResult<Vec<String>>;
}

use std::clone;

use ::futures::StreamExt;
use ::kvs::redis::Commands;
use ::kvs::{Connection as KVSConnection, Store};
use ::observers::kvs::ObserverNodeKVS;
use ::symbols::traits::{Symbol, SymbolWriter};

use crate::errors::Result as ObserverControlResult;

pub enum SymbolDiff {
  Add(String),
  Del(String),
}

pub struct SyncHandler<S, T>
where
  T: Commands,
  S: SymbolWriter,
{
  symbol_db: S,
  kvs: ObserverNodeKVS<T>,
}

impl<S, T> SyncHandler<S, T>
where
  S: SymbolWriter,
  T: Commands,
{
  pub fn new(symbol_db: S, cmd: KVSConnection<T>) -> Self {
    return Self {
      symbol_db,
      kvs: ObserverNodeKVS::new(cmd.clone().into()),
    };
  }

  pub async fn get_symbol_diff(
    &mut self,
  ) -> ObserverControlResult<Vec<String>> {
    let node_ids: Vec<String> = self.kvs.scan_match("*")?;
    let mut nodes_symbols = vec![];
    for node_id in node_ids {
      let symbols: Vec<String> = self.kvs.lrange(&node_id, 0, -1)?;
      for symbol in symbols {
        if !symbol.is_empty() {
          nodes_symbols.push(symbol);
        }
      }
    }
    let db_symbols = self.symbol_db.list_trading().await?.map(|s| s.symbol());
    unimplemented!();
    return Ok(nodes_symbols);
  }

  pub fn handle(&self) {}
}

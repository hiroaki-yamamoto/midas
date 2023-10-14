use ::std::collections::HashSet;

use ::futures::StreamExt;
use ::log::info;
use ::mongodb::Database;

use crate::entities::TradeObserverControlEvent as Event;
use crate::kvs::NodeFilter;
use crate::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::kvs::redis::Commands;
use ::kvs::Connection as KVSConnection;
use ::rpc::entities::Exchanges;
use ::symbols::get_reader;

use ::errors::ObserverResult;

pub struct NodeDIffTaker<T>
where
  T: Commands + Send + Sync,
{
  db: Database,
  kvs: ObserverNodeKVS<T>,
  type_kvs: ONEXTypeKVS<T>,
}

impl<T> NodeDIffTaker<T>
where
  T: Commands + Send + Sync,
{
  pub async fn new(
    db: &Database,
    cmd: KVSConnection<T>,
  ) -> ObserverResult<Self> {
    return Ok(Self {
      db: db.clone(),
      kvs: ObserverNodeKVS::new(cmd.clone().into()),
      type_kvs: ONEXTypeKVS::new(cmd.clone().into()),
    });
  }

  pub async fn get_symbol_diff(
    &self,
    exchange: &Exchanges,
  ) -> ObserverResult<HashSet<Event>> {
    let symbol_reader = get_reader(&self.db, exchange.clone()).await; // TODO: fix this
    let trading_symbols_list = symbol_reader.list_trading().await?;
    info!("Fetching symbols from DB");
    let db_symbols: HashSet<String> =
      trading_symbols_list.map(|s| s.symbol).collect().await;

    info!("Fetching symbols from KVS");
    let node_filter = NodeFilter::new(&self.kvs, &self.type_kvs);
    let nodes_symbols: HashSet<String> = node_filter
      .get_handling_symbol_at_exchange(exchange.clone())
      .await?
      .collect()
      .await;
    info!("Taking symbols to add");
    let to_add = (&db_symbols - &nodes_symbols)
      .into_iter()
      .map(|s| Event::SymbolAdd(exchange.clone(), s));
    info!("Taking symbols to remove");
    let to_remove = (&nodes_symbols - &db_symbols)
      .into_iter()
      .map(|s| Event::SymbolDel(exchange.clone(), s));
    let merged: HashSet<Event> = to_add.chain(to_remove).collect();
    return Ok(merged);
  }
}

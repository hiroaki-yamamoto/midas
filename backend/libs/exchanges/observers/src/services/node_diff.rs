use ::std::collections::HashSet;
use ::std::fmt::Debug;
use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::futures::StreamExt;
use ::log::info;
use ::mongodb::Database;

use ::kvs::redis::AsyncCommands as Commands;
use ::kvs::traits::last_checked::ListOp;
use ::rpc::exchanges::Exchanges;
use ::symbols::get_reader;

use ::errors::ObserverResult;

use crate::entities::TradeObserverControlEvent as Event;
use crate::kvs::{NODE_EXCHANGE_TYPE_KVS_BUILDER, NODE_KVS_BUILDER};

use super::{NodeFilter, NodeIndexer};

pub struct NodeDIffTaker<T>
where
  T: Commands + Clone + Debug + Send + Sync + 'static,
{
  db: Database,
  kvs: Arc<dyn ListOp<Commands = T, Value = String> + Send + Sync>,
  indexer: Arc<NodeIndexer<T>>,
  _t: PhantomData<T>,
}

impl<T> NodeDIffTaker<T>
where
  T: Commands + Clone + Debug + Send + Sync + 'static,
{
  pub async fn new(db: &Database, cmd: T) -> ObserverResult<Self> {
    return Ok(Self {
      db: db.clone(),
      kvs: Arc::new(NODE_KVS_BUILDER.build(cmd.clone())),
      indexer: Arc::new(NodeIndexer::new(
        NODE_EXCHANGE_TYPE_KVS_BUILDER.build(cmd).into(),
      )),
      _t: PhantomData,
    });
  }

  pub async fn get_symbol_diff(
    &self,
    exchange: Box<Exchanges>,
  ) -> ObserverResult<HashSet<Event>> {
    let symbol_reader = get_reader(&self.db, exchange.clone()).await; // TODO: fix this
    let trading_symbols_list = symbol_reader.list_trading().await?;
    info!("Fetching symbols from DB");
    let db_symbols: HashSet<Arc<String>> = trading_symbols_list
      .map(|s| Arc::new(s.symbol))
      .collect()
      .await;

    info!("Fetching symbols from KVS");
    let node_filter = NodeFilter::new(self.kvs.clone(), self.indexer.clone());
    let nodes_symbols: HashSet<Arc<String>> = node_filter
      .get_handling_symbol_at_exchange(exchange.clone())
      .await?
      .collect()
      .await;
    info!("Taking symbols to add");
    let to_add = (&db_symbols - &nodes_symbols)
      .into_iter()
      .map(|s| Event::SymbolAdd(exchange.clone(), s.to_string()));
    info!("Taking symbols to remove");
    let to_remove = (&nodes_symbols - &db_symbols)
      .into_iter()
      .map(|s| Event::SymbolDel(exchange.clone(), s.to_string()));
    let merged: HashSet<Event> = to_add.chain(to_remove).collect();
    return Ok(merged);
  }
}

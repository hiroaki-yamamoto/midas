use ::std::collections::HashSet;
use ::std::fmt::Debug;
use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::futures::StreamExt;

use ::errors::{KVSResult, ObserverResult};
use ::kvs::redis::AsyncCommands as Commands;
use ::kvs::traits::last_checked::ListOp;
use ::rpc::exchanges::Exchanges;

use crate::entities::TradeObserverControlEvent as ControlEvent;
use crate::kvs::{NODE_EXCHANGE_TYPE_KVS_BUILDER, NODE_KVS_BUILDER};

use super::{NodeFilter, NodeIndexer};

pub struct ObservationBalancer<T>
where
  T: Commands + Clone + Send + Sync + Debug + 'static,
{
  node_kvs: Arc<dyn ListOp<Value = String, Commands = T> + Send + Sync>,
  indexer: Arc<NodeIndexer<T>>,
  node_filter: Arc<NodeFilter<T>>,
  _t: PhantomData<T>,
}

impl<T> ObservationBalancer<T>
where
  T: Commands + Clone + Send + Sync + Debug + 'static,
{
  pub async fn new(kvs: T) -> ObserverResult<Self> {
    let node_kvs: Arc<dyn ListOp<Commands = T, Value = String> + Send + Sync> =
      Arc::new(NODE_KVS_BUILDER.build(kvs.clone()));
    let exchange_type_kvs: Arc<_> =
      NODE_EXCHANGE_TYPE_KVS_BUILDER.build(kvs).into();
    let indexer: Arc<_> = NodeIndexer::new(exchange_type_kvs.clone()).into();
    let filter = NodeFilter::new(node_kvs.clone(), indexer.clone()).into();
    return Ok(Self {
      node_kvs: node_kvs,
      indexer: indexer,
      node_filter: filter,
      _t: PhantomData,
    });
  }

  async fn calc_num_average_symbols(
    &self,
    exchange: Box<Exchanges>,
  ) -> KVSResult<usize> {
    let nodes = self.indexer.get_nodes_by_exchange(exchange.clone()).await?;
    let num_nodes = nodes.count().await;
    let num_symbols = self
      .node_filter
      .get_handling_symbol_at_exchange(exchange)
      .await?
      .count()
      .await;
    return Ok(num_symbols / num_nodes);
  }

  pub async fn get_event_to_balancing(
    &self,
    exchange: Box<Exchanges>,
  ) -> ObserverResult<HashSet<ControlEvent>> {
    let num_average_symbols =
      self.calc_num_average_symbols(exchange.clone()).await?;
    let overflowed_nodes = self
      .node_filter
      .get_overflowed_nodes(exchange.clone(), num_average_symbols)
      .await?;
    let mut symbol_diff: HashSet<ControlEvent> = HashSet::new();
    for node in overflowed_nodes {
      let symbols: Vec<String> = self
        .node_kvs
        .lrange(node, num_average_symbols as isize, -1)
        .await?;
      let remove: Vec<ControlEvent> = symbols
        .clone()
        .into_iter()
        .map(|symbol| ControlEvent::SymbolDel(exchange.clone(), symbol))
        .collect();
      let add: Vec<ControlEvent> = symbols
        .into_iter()
        .map(|symbol| ControlEvent::SymbolAdd(exchange.clone(), symbol))
        .collect();
      symbol_diff.extend(remove);
      symbol_diff.extend(add);
    }
    return Ok(symbol_diff);
  }
}

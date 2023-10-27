use ::std::sync::Arc;

use ::std::collections::HashSet;
use ::std::marker::PhantomData;

use ::errors::KVSResult;
use ::futures::stream::{iter, BoxStream, StreamExt};
use ::kvs::redis::AsyncCommands as Commands;
use ::kvs::traits::last_checked::{Get, ListOp, SetOp};
use ::rpc::entities::Exchanges;

use super::NodeIndexer;

pub struct NodeFilter<T, NodeKVS, ExchangeTypeKVS>
where
  T: Commands + Send + Sync,
  NodeKVS: ListOp<T, String> + Send + Sync,
  ExchangeTypeKVS: Get<T, String> + SetOp<T, String> + Send + Sync,
{
  node_kvs: Arc<NodeKVS>,
  indexer: Arc<NodeIndexer<T, ExchangeTypeKVS>>,
  _t: PhantomData<T>,
}

impl<T, NodeKVS, ExchangeTypeKVS> NodeFilter<T, NodeKVS, ExchangeTypeKVS>
where
  T: Commands + Send + Sync,
  NodeKVS: ListOp<T, String> + Send + Sync,
  ExchangeTypeKVS: Get<T, String> + SetOp<T, String> + Send + Sync,
{
  pub fn new(
    node_kvs: Arc<NodeKVS>,
    indexer: Arc<NodeIndexer<T, ExchangeTypeKVS>>,
  ) -> Self {
    return Self {
      node_kvs,
      indexer,
      _t: PhantomData,
    };
  }

  pub async fn get_handling_symbol_at_exchange(
    &self,
    exchange: Exchanges,
  ) -> KVSResult<BoxStream<Arc<String>>> {
    let nodes = self
      .indexer
      .get_nodes_by_exchange(exchange)
      .await?
      .filter_map(move |node_id| async {
        let val: Option<Vec<String>> =
          self.node_kvs.lrange(node_id, 0, -1).await.ok();
        return val;
      })
      .map(|symbol_vec| {
        let set: HashSet<Arc<String>> =
          symbol_vec.into_iter().map(|symbol| symbol.into()).collect();
        return set;
      })
      .flat_map(|symbols| iter(symbols));
    return Ok(nodes.boxed());
  }

  pub async fn get_overflowed_nodes(
    &self,
    exchange: Exchanges,
    num_symbols: usize,
  ) -> KVSResult<Vec<Arc<String>>> {
    let nodes: Vec<Arc<String>> = self
      .get_handling_symbol_at_exchange(exchange)
      .await?
      .filter_map(|node| async move {
        return self
          .node_kvs
          .llen(node.clone())
          .await
          .map(|num| (node, num))
          .ok();
      })
      .filter_map(|(node, num)| async move {
        return if num > num_symbols { Some(node) } else { None };
      })
      .collect()
      .await;
    return Ok(nodes);
  }
}

use ::std::collections::HashSet;

use ::errors::KVSResult;
use ::futures::stream::{iter, BoxStream, StreamExt};
use ::kvs::redis::Commands;
use ::kvs::traits::last_checked::ListOp;
use ::rpc::entities::Exchanges;

use super::NodeIndexer;

pub struct NodeFilter<T, NodeKVS>
where
  T: Commands + Send + Sync,
  NodeKVS: ListOp<T, String>,
{
  node_kvs: NodeKVS,
  indexer: NodeIndexer<T>,
}

impl<T, NodeKVS> NodeFilter<T, NodeKVS>
where
  T: Commands + Send + Sync,
  NodeKVS: ListOp<T, String>,
{
  pub fn new(node_kvs: NodeKVS, indexer: NodeIndexer<T>) -> Self {
    Self { node_kvs, indexer }
  }

  pub async fn get_handling_symbol_at_exchange(
    &self,
    exchange: Exchanges,
  ) -> KVSResult<BoxStream<String>> {
    let nodes = self
      .exchange_type_kvs
      .get_nodes_by_exchange(exchange)
      .await?
      .filter_map(move |node_id| async move {
        let node_id_cloned = node_id.clone();
        return self
          .node_kvs
          .lrange::<HashSet<String>>(&node_id_cloned, 0, -1)
          .await
          .ok();
      })
      .flat_map(|symbols| iter(symbols));
    return Ok(nodes.boxed());
  }

  pub async fn get_overflowed_nodes(
    &self,
    exchange: Exchanges,
    num_symbols: usize,
  ) -> KVSResult<Vec<String>> {
    let nodes: Vec<String> = self
      .get_handling_symbol_at_exchange(exchange)
      .await?
      .filter_map(|node| async move {
        return self
          .node_kvs
          .llen::<usize>(&node)
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

use ::std::sync::Arc;

use ::std::collections::HashSet;
use ::std::marker::PhantomData;

use ::errors::KVSResult;
use ::futures::stream::{iter, BoxStream, StreamExt};
use ::kvs::redis::AsyncCommands as Commands;
use ::kvs::traits::last_checked::ListOp;
use ::rpc::exchanges::Exchanges;

use super::NodeIndexer;

pub struct NodeFilter<T>
where
  T: Commands + Send + Sync + 'static,
{
  node_kvs: Arc<dyn ListOp<Commands = T, Value = String> + Send + Sync>,
  indexer: Arc<NodeIndexer<T>>,
  _t: PhantomData<T>,
}

impl<T> NodeFilter<T>
where
  T: Commands + Send + Sync,
{
  pub fn new(
    node_kvs: Arc<dyn ListOp<Commands = T, Value = String> + Send + Sync>,
    indexer: Arc<NodeIndexer<T>>,
  ) -> Self {
    return Self {
      node_kvs,
      indexer,
      _t: PhantomData,
    };
  }

  pub async fn get_handling_symbol_at_exchange(
    &self,
    exchange: Box<Exchanges>,
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
    exchange: Box<Exchanges>,
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

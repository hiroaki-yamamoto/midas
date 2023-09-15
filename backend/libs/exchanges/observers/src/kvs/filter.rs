use ::std::collections::HashSet;

use ::futures::stream::{iter, BoxStream, StreamExt};
use ::kvs::redis::{Commands, RedisResult};
use ::kvs::traits::last_checked::ListOp;
use ::rpc::entities::Exchanges;

use super::{ONEXTypeKVS, ObserverNodeKVS};

pub struct NodeFilter<T>
where
  T: Commands + Send + Sync,
{
  node_kvs: ObserverNodeKVS<T>,
  exchange_type_kvs: ONEXTypeKVS<T>,
}

impl<T> NodeFilter<T>
where
  T: Commands + Send + Sync,
{
  pub fn new(
    node_kvs: &ObserverNodeKVS<T>,
    exchange_type_kvs: &ONEXTypeKVS<T>,
  ) -> Self {
    Self {
      node_kvs: node_kvs.clone(),
      exchange_type_kvs: exchange_type_kvs.clone(),
    }
  }

  pub async fn get_handling_symbol_at_exchange(
    &self,
    exchange: Exchanges,
  ) -> RedisResult<BoxStream<String>> {
    let nodes = self
      .exchange_type_kvs
      .get_nodes_by_exchange(exchange)
      .await?
      .map(move |node_id| {
        let node_id_cloned = node_id.clone();
        return async move {
          self
            .node_kvs
            .lrange::<HashSet<String>>(&node_id_cloned, 0, -1)
            .await
        };
      })
      .filter_map(|lrange_fut| async { lrange_fut.await.ok() })
      .flat_map(|symbols| iter(symbols));
    return Ok(nodes.boxed());
  }

  pub async fn get_overflowed_nodes(
    &self,
    exchange: Exchanges,
    num_symbols: usize,
  ) -> RedisResult<Vec<String>> {
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

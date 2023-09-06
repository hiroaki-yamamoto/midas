use ::futures::stream::{iter, BoxStream, StreamExt};
use ::std::collections::HashSet;

use ::kvs::redis::Commands;
use ::kvs::redis::RedisResult;
use ::kvs::traits::normal::ListOp;
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
      .map(|node_id| self.node_kvs.lrange::<HashSet<String>>(node_id, 0, -1))
      .filter_map(|lrange_fut| async { lrange_fut.await.ok() })
      .flat_map(|symbols| iter(symbols));
    return Ok(nodes.boxed());
  }
}

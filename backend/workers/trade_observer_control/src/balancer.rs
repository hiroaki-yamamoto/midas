use ::futures::StreamExt;

use ::kvs::redis::{Commands, RedisResult};
use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::entities::Exchanges;

pub struct SymbolBalancer<T>
where
  T: Commands + Send + Sync,
{
  control_pubsub: NodeControlEventPubSub,
  node_kvs: ObserverNodeKVS<T>,
  exchange_type_kvs: ONEXTypeKVS<T>,
}

impl<T> SymbolBalancer<T>
where
  T: Commands + Send + Sync,
{
  pub fn new(
    control_pubsub: &NodeControlEventPubSub,
    node_kvs: &ObserverNodeKVS<T>,
    exchange_type_kvs: &ONEXTypeKVS<T>,
  ) -> Self {
    Self {
      control_pubsub: control_pubsub.clone(),
      node_kvs: node_kvs.clone(),
      exchange_type_kvs: exchange_type_kvs.clone(),
    }
  }

  async fn calc_num_average_symbols(
    &self,
    exchange: Exchanges,
    num_added_nodes: usize,
  ) -> RedisResult<usize> {
    let nodes = self
      .exchange_type_kvs
      .get_nodes_by_exchange(exchange)
      .await?;
    let num_nodes = num_added_nodes;
    let num_symbols = 0;
    nodes.for_each(|node_id| {
      num_nodes += 1;
      self.node_kvs.get_handling_symbols()
    });
  }
}

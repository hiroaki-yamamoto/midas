use ::futures::StreamExt;

use ::kvs::redis::{Commands, RedisResult};
use ::observers::kvs::{NodeFilter, ONEXTypeKVS, ObserverNodeKVS};
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
    let num_nodes = num_added_nodes + nodes.count().await;
    let filter = NodeFilter::new(&self.node_kvs, &self.exchange_type_kvs);
    let num_symbols = filter
      .get_handling_symbol_at_exchange(exchange)
      .await?
      .count()
      .await;
    return Ok(num_symbols / num_nodes);
  }
}

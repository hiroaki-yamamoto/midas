use ::kvs::redis::Commands;
use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;

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

  async fn calc_num_average_symbols(&self) -> usize {}
}

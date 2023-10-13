use ::futures::future::try_join_all;
use ::futures::StreamExt;

use ::kvs::redis::{Commands, RedisResult};
use ::kvs::traits::last_checked::ListOp;
use ::observers::entities::TradeObserverControlEvent;
use ::observers::kvs::{NodeFilter, ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::entities::Exchanges;
use ::subscribe::PubSub;

use crate::errors::Result as ControlResult;

pub struct SymbolBalancer<T>
where
  T: Commands + Send + Sync,
{
  control_pubsub: NodeControlEventPubSub,
  node_kvs: ObserverNodeKVS<T>,
  exchange_type_kvs: ONEXTypeKVS<T>,
  node_filter: NodeFilter<T>,
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
    let filter = NodeFilter::new(node_kvs, exchange_type_kvs);
    Self {
      control_pubsub: control_pubsub.clone(),
      node_kvs: node_kvs.clone(),
      exchange_type_kvs: exchange_type_kvs.clone(),
      node_filter: filter,
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
    let num_symbols = self
      .node_filter
      .get_handling_symbol_at_exchange(exchange)
      .await?
      .count()
      .await;
    return Ok(num_symbols / num_nodes);
  }

  pub async fn broadcast_equalization(
    &self,
    exchange: Exchanges,
    num_added_nodes: usize,
  ) -> ControlResult<()> {
    let num_average_symbols = self
      .calc_num_average_symbols(exchange, num_added_nodes)
      .await?;
    let overflowed_nodes = self
      .node_filter
      .get_overflowed_nodes(exchange, num_average_symbols)
      .await?;
    let mut defer = vec![];
    for node in overflowed_nodes {
      let symbols: Vec<String> = self
        .node_kvs
        .lrange(&node, num_average_symbols as isize, -1)
        .await?;
      let mut remove_defer = symbols
        .clone()
        .into_iter()
        .map(|symbol| {
          return self
            .control_pubsub
            .publish(TradeObserverControlEvent::SymbolDel(exchange, symbol));
        })
        .collect();
      let mut add_defer = symbols
        .into_iter()
        .map(|symbol| {
          return self
            .control_pubsub
            .publish(TradeObserverControlEvent::SymbolAdd(exchange, symbol));
        })
        .collect();
      defer.append(&mut add_defer);
      defer.append(&mut remove_defer);
    }
    let _ = try_join_all(defer).await?;
    return Ok(());
  }
}

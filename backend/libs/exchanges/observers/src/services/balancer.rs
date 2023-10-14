use ::futures::future::try_join_all;
use ::futures::StreamExt;

use ::errors::{KVSResult, ObserverResult};
use ::kvs::redis::Commands;
use ::kvs::traits::last_checked::ListOp;
use ::kvs::Connection;
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;

use crate::entities::TradeObserverControlEvent;
use crate::kvs::{NodeFilter, ONEXTypeKVS, ObserverNodeKVS};
use crate::pubsub::NodeControlEventPubSub;

pub struct ObservationBalancer<T>
where
  T: Commands + Send + Sync,
{
  control_pubsub: NodeControlEventPubSub,
  node_kvs: ObserverNodeKVS<T>,
  exchange_type_kvs: ONEXTypeKVS<T>,
  node_filter: NodeFilter<T>,
}

impl<T> ObservationBalancer<T>
where
  T: Commands + Send + Sync,
{
  pub async fn new(kvs: Connection<T>, nats: Nats) -> ObserverResult<Self> {
    let control_pubsub = NodeControlEventPubSub::new(&nats).await?;
    let node_kvs = ObserverNodeKVS::new(kvs.clone().into());
    let exchange_type_kvs = ONEXTypeKVS::new(kvs.clone().into());
    let filter = NodeFilter::new(&node_kvs, &exchange_type_kvs);
    return Ok(Self {
      control_pubsub: control_pubsub,
      node_kvs: node_kvs,
      exchange_type_kvs: exchange_type_kvs,
      node_filter: filter,
    });
  }

  async fn calc_num_average_symbols(
    &self,
    exchange: Exchanges,
    num_added_nodes: usize,
  ) -> KVSResult<usize> {
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
  ) -> ObserverResult<()> {
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

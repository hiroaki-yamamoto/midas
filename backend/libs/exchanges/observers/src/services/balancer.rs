use ::std::collections::HashSet;

use ::futures::StreamExt;

use ::errors::{KVSResult, ObserverResult};
use ::kvs::redis::AsyncCommands as Commands;
use ::kvs::traits::last_checked::{ListOp, SetOp};
use ::rpc::entities::Exchanges;

use crate::entities::TradeObserverControlEvent as ControlEvent;
use crate::kvs::NodeFilter;
use crate::kvs::{NODE_EXCHANGE_TYPE_KVS_BUILDER, NODE_KVS_BUILDER};

pub struct ObservationBalancer<T, NodeKVS, ExchangeTypeKVS>
where
  T: Commands + Send + Sync,
  NodeKVS: ListOp<T, String>,
  ExchangeTypeKVS: SetOp<T, String>,
{
  node_kvs: NodeKVS,
  exchange_type_kvs: ExchangeTypeKVS,
  node_filter: NodeFilter<T, NodeKVS, ExchangeTypeKVS>,
}

impl<T, NodeKVS, ExchangeTypeKVS>
  ObservationBalancer<T, NodeKVS, ExchangeTypeKVS>
where
  T: Commands + Send + Sync,
  NodeKVS: ListOp<T, String>,
  ExchangeTypeKVS: SetOp<T, String>,
{
  pub async fn new(kvs: T) -> ObserverResult<Self> {
    let node_kvs = NODE_KVS_BUILDER.build(kvs).await?;
    let exchange_type_kvs = NODE_EXCHANGE_TYPE_KVS_BUILDER.build(kvs).await?;
    let filter = NodeFilter::new(&node_kvs, &exchange_type_kvs);
    return Ok(Self {
      node_kvs: node_kvs.into(),
      exchange_type_kvs: exchange_type_kvs.into(),
      node_filter: filter,
    });
  }

  async fn calc_num_average_symbols(
    &self,
    exchange: Exchanges,
  ) -> KVSResult<usize> {
    let nodes = self
      .exchange_type_kvs
      .get_nodes_by_exchange(exchange)
      .await?;
    let num_nodes = nodes.count().await;
    let num_symbols = self
      .node_filter
      .get_handling_symbol_at_exchange(exchange)
      .await?
      .count()
      .await;
    return Ok(num_symbols / num_nodes);
  }

  pub async fn get_event_to_balancing(
    &self,
    exchange: Exchanges,
  ) -> ObserverResult<HashSet<ControlEvent>> {
    let num_average_symbols = self.calc_num_average_symbols(exchange).await?;
    let overflowed_nodes = self
      .node_filter
      .get_overflowed_nodes(exchange, num_average_symbols)
      .await?;
    let mut symbol_diff: HashSet<ControlEvent> = HashSet::new();
    for node in overflowed_nodes {
      let symbols: Vec<String> = self
        .node_kvs
        .lrange(&node, num_average_symbols as isize, -1)
        .await?;
      let remove: Vec<ControlEvent> = symbols
        .clone()
        .into_iter()
        .map(|symbol| ControlEvent::SymbolDel(exchange, symbol))
        .collect();
      let add: Vec<ControlEvent> = symbols
        .into_iter()
        .map(|symbol| ControlEvent::SymbolAdd(exchange, symbol))
        .collect();
      symbol_diff.extend(remove);
      symbol_diff.extend(add);
    }
    return Ok(symbol_diff);
  }
}

use ::std::collections::HashSet;

use ::futures::StreamExt;

use ::errors::{KVSResult, ObserverResult};
use ::kvs::redis::Commands;
use ::kvs::traits::last_checked::ListOp;
use ::kvs::Connection;
use ::rpc::entities::Exchanges;

use crate::entities::TradeObserverControlEvent as ControlEvent;
use crate::kvs::{NodeFilter, ONEXTypeKVS, ObserverNodeKVS};

pub struct ObservationBalancer<T>
where
  T: Commands + Send + Sync,
{
  node_kvs: ObserverNodeKVS<T>,
  exchange_type_kvs: ONEXTypeKVS<T>,
  node_filter: NodeFilter<T>,
}

impl<T> ObservationBalancer<T>
where
  T: Commands + Send + Sync,
{
  pub async fn new(kvs: Connection<T>) -> ObserverResult<Self> {
    let node_kvs = ObserverNodeKVS::new(kvs.clone().into());
    let exchange_type_kvs = ONEXTypeKVS::new(kvs.clone().into());
    let filter = NodeFilter::new(&node_kvs, &exchange_type_kvs);
    return Ok(Self {
      node_kvs: node_kvs,
      exchange_type_kvs: exchange_type_kvs,
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

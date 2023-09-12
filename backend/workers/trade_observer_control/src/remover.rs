use ::std::collections::HashSet;

use ::futures::join;
use ::mongodb::Database;
use ::uuid::Uuid;

use ::entities::TradeObserverControlEvent;
use ::errors::{KVSResult, UnknownExchangeError};
use ::kvs::redis::Commands;
use ::kvs::traits::last_checked::{Get, ListOp, Remove};
use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::entities::Exchanges;
use ::subscribe::PubSub;

use crate::errors::Result as ControlResult;
use crate::handlers::SyncHandler;

pub struct NodeRemover<C>
where
  C: Commands + Send + Sync,
{
  node_kvs: ObserverNodeKVS<C>,
  type_kvs: ONEXTypeKVS<C>,
  control_event: NodeControlEventPubSub,
  db: Database,
}

impl<C> NodeRemover<C>
where
  C: Commands + Send + Sync,
{
  pub fn new(
    node_kvs: ObserverNodeKVS<C>,
    type_kvs: ONEXTypeKVS<C>,
    control_event: NodeControlEventPubSub,
    db: Database,
  ) -> Self {
    return Self {
      node_kvs,
      type_kvs,
      control_event,
      db,
    };
  }

  async fn remove_node(&self, node_id: &Uuid) {
    let node_id = node_id.to_string();
    let (_, _): (KVSResult<usize>, KVSResult<usize>) =
      join!(self.node_kvs.del(&node_id), self.type_kvs.del(&node_id));
  }

  // async fn list_trading_symbols(
  //   &self,
  //   exchange: Exchanges,
  // ) -> ControlResult<HashSet<String>> {
  //   let symbol_reader = get_reader(&self.db, exchange).await;
  //   let trading_symbols: HashSet<String> = symbol_reader
  //     .list_trading()
  //     .await?
  //     .map(|sym| sym.symbol)
  //     .collect()
  //     .await;
  //   return Ok(trading_symbols);
  // }

  pub async fn handle(&self, node_id: Uuid) -> ControlResult<()> {
    let (symbols, exchange) = join!(
      self.node_kvs.lrange(node_id.to_string(), 0, -1),
      self.type_kvs.get(node_id.to_string())
    );
    let symbols: Vec<String> = symbols.unwrap_or(vec![]);
    let exchange: String = exchange.unwrap_or("".into());
    let exchange: Exchanges = match Exchanges::from_str_name(&exchange) {
      Some(exchange) => Ok(exchange),
      None => {
        self.remove_node(&node_id).await;
        Err(UnknownExchangeError::new(exchange))
      }
    }?;
    let symbols: HashSet<String> = if symbols.is_empty() {
      let mut sync_handler = SyncHandler::from_raw(
        self.db.clone(),
        self.node_kvs.clone(),
        self.type_kvs.clone(),
        self.control_event.clone(),
      );
      let _ = sync_handler.handle(&exchange).await;
      HashSet::new()
    } else {
      symbols.into_iter().collect()
    };
    self.remove_node(&node_id).await;
    symbols.into_iter().for_each(|symbol| {
      let _ = self
        .control_event
        .publish(TradeObserverControlEvent::SymbolAdd(exchange, symbol));
    });
    return Ok(());
  }
}

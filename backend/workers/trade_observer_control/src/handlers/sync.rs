use ::std::collections::HashSet;

use ::futures::future::{try_join_all, TryFutureExt};
use ::futures::StreamExt;
use ::mongodb::Database;

use ::entities::TradeObserverControlEvent as Event;
use ::kvs::redis::Commands;
use ::kvs::{Connection as KVSConnection, Store};
use ::observers::kvs::ObserverNodeKVS;
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::entities::Exchanges;
use ::subscribe::natsJS::context::Context;
use ::subscribe::PubSub;
use ::symbols::binance::recorder::SymbolWriter as BinanceSymbolWriter;
use ::symbols::traits::SymbolReader as SymbolReaderTrait;

use crate::errors::Result as ObserverControlResult;

pub struct SyncHandler<T>
where
  T: Commands + Send + Sync,
{
  db: Database,
  kvs: ObserverNodeKVS<T>,
  nats: Context,
}

impl<T> SyncHandler<T>
where
  T: Commands + Send + Sync,
{
  pub fn new(db: &Database, cmd: KVSConnection<T>, nats: &Context) -> Self {
    return Self {
      db: db.clone(),
      kvs: ObserverNodeKVS::new(cmd.clone().into()),
      nats: nats.clone(),
    };
  }

  pub async fn get_symbol_diff(
    &mut self,
    exchange: &Exchanges,
  ) -> ObserverControlResult<Vec<Event>> {
    let symbol_reader: Box<dyn SymbolReaderTrait + Send + Sync> = match exchange
    {
      Exchanges::Binance => Box::new(BinanceSymbolWriter::new(&self.db).await),
    };
    let trading_symbols_list = symbol_reader.list_trading().await?;
    let db_symbols: HashSet<String> =
      trading_symbols_list.map(|s| s.symbol).collect().await;

    let node_ids: Vec<String> = self.kvs.scan_match("*")?;
    let mut nodes_symbols = vec![];
    for node_id in node_ids {
      let symbols: Vec<String> = self.kvs.lrange(&node_id, 0, -1)?;
      for symbol in symbols {
        if !symbol.is_empty() {
          nodes_symbols.push(symbol);
        }
      }
    }
    let nodes_symbols = HashSet::from_iter(nodes_symbols.into_iter());
    let to_add = (&db_symbols - &nodes_symbols)
      .into_iter()
      .map(|s| Event::SymbolAdd(exchange.clone(), s));
    let to_remove = (&nodes_symbols - &db_symbols)
      .into_iter()
      .map(|s| Event::SymbolDel(exchange.clone(), s));
    let merged: Vec<Event> = to_add.chain(to_remove).collect();
    return Ok(merged);
  }

  pub async fn handle(
    &mut self,
    exchange: &Exchanges,
  ) -> ObserverControlResult<()> {
    let publisher = NodeControlEventPubSub::new(&self.nats).await?;
    let publish_defer =
      self
        .get_symbol_diff(exchange)
        .await?
        .into_iter()
        .map(|diff| {
          return publisher.publish(diff);
        });
    let _ = try_join_all(publish_defer).await?;
    return Ok(());
  }
}

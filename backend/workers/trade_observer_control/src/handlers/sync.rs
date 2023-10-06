use ::std::collections::HashSet;

use ::futures::future::try_join_all;
use ::futures::StreamExt;
use ::log::info;
use ::mongodb::Database;

use ::entities::TradeObserverControlEvent as Event;
use ::kvs::redis::Commands;
use ::kvs::Connection as KVSConnection;
use ::observers::kvs::NodeFilter;
use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;
use ::symbols::get_reader;

use crate::errors::Result as ObserverControlResult;

pub struct SyncHandler<T>
where
  T: Commands + Send + Sync,
{
  db: Database,
  kvs: ObserverNodeKVS<T>,
  type_kvs: ONEXTypeKVS<T>,
  publisher: NodeControlEventPubSub,
}

impl<T> SyncHandler<T>
where
  T: Commands + Send + Sync,
{
  pub async fn new(
    db: &Database,
    cmd: KVSConnection<T>,
    nats: &Nats,
  ) -> ObserverControlResult<Self> {
    return Ok(Self {
      db: db.clone(),
      kvs: ObserverNodeKVS::new(cmd.clone().into()),
      type_kvs: ONEXTypeKVS::new(cmd.clone().into()),
      publisher: NodeControlEventPubSub::new(nats).await?,
    });
  }

  pub fn from_raw(
    db: Database,
    kvs: ObserverNodeKVS<T>,
    type_kvs: ONEXTypeKVS<T>,
    publisher: NodeControlEventPubSub,
  ) -> Self {
    return Self {
      db,
      kvs,
      type_kvs,
      publisher,
    };
  }

  pub async fn get_symbol_diff(
    &mut self,
    exchange: &Exchanges,
  ) -> ObserverControlResult<Vec<Event>> {
    let symbol_reader = get_reader(&self.db, exchange.clone()).await; // TODO: fix this
    let trading_symbols_list = symbol_reader.list_trading().await?;
    info!("Fetching symbols from DB");
    let db_symbols: HashSet<String> =
      trading_symbols_list.map(|s| s.symbol).collect().await;

    info!("Fetching symbols from KVS");
    let node_filter = NodeFilter::new(&self.kvs, &self.type_kvs);
    let nodes_symbols: HashSet<String> = node_filter
      .get_handling_symbol_at_exchange(exchange.clone())
      .await?
      .collect()
      .await;
    info!("Taking symbols to add");
    let to_add = (&db_symbols - &nodes_symbols)
      .into_iter()
      .map(|s| Event::SymbolAdd(exchange.clone(), s));
    info!("Taking symbols to remove");
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
    let publish_defer =
      self
        .get_symbol_diff(exchange)
        .await?
        .into_iter()
        .map(|diff| {
          info!("Publishing symbol {:?}", diff);
          return self.publisher.publish(diff);
        });
    let _ = try_join_all(publish_defer).await?;
    return Ok(());
  }
}

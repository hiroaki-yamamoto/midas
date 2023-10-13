use ::std::time::Duration;

use ::futures::future::try_join;
use ::futures::stream::StreamExt;
use ::uuid::Uuid;

use ::config::{Database, ObserverConfig};
use ::kvs::redis::Commands;
use ::kvs::traits::normal::{Expiration, Lock, Set};
use ::kvs::{Connection, WriteOption};
use ::log::{as_error, error, info};
use ::observers::entities::{
  TradeObserverControlEvent, TradeObserverNodeEvent,
};
use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::natsJS::message::Message;
use ::tokio::time::sleep;

use crate::balancer::SymbolBalancer;
use crate::dlock::InitLock;
use crate::errors::Result as ControlResult;
use crate::remover::NodeRemover;

use super::SyncHandler;

pub struct FromNodeEventHandler<C>
where
  C: Commands + Send + Sync,
{
  nats: Nats,
  kvs_cmd: Connection<C>,
  control_event: NodeControlEventPubSub,
  node_kvs: ObserverNodeKVS<C>,
  db: Database,
  type_kvs: ONEXTypeKVS<C>,
  init_lock: InitLock<C>,
  symbol_balancer: SymbolBalancer<C>,
}

impl<C> FromNodeEventHandler<C>
where
  C: Commands + Send + Sync,
{
  pub async fn new(
    kvs_com: Connection<C>,
    db: Database,
    nats: &Nats,
  ) -> ControlResult<Self> {
    let control_event = NodeControlEventPubSub::new(nats).await?;
    let node_kvs = ObserverNodeKVS::new(kvs_com.clone().into());
    let type_kvs = ONEXTypeKVS::new(kvs_com.clone().into());
    let symbol_balancer =
      SymbolBalancer::new(&control_event, &node_kvs, &type_kvs);
    return Ok(Self {
      kvs_cmd: kvs_com.clone().into(),
      nats: nats.clone(),
      init_lock: InitLock::new(kvs_com.into()),
      control_event,
      node_kvs,
      type_kvs,
      symbol_balancer,
      db,
    });
  }

  pub async fn handle(
    &mut self,
    msg: &Message,
    event: TradeObserverNodeEvent,
    config: &ObserverConfig,
  ) -> ControlResult<()> {
    match event {
      TradeObserverNodeEvent::Ping(node_id) => {
        try_join(
          self
            .node_kvs
            .expire(&node_id.to_string(), Duration::from_secs(30)),
          self
            .type_kvs
            .expire(&node_id.to_string(), Duration::from_secs(30)),
        )
        .await?;
      }
      TradeObserverNodeEvent::Regist(exchange) => {
        info!("Prepare for node id assignment");
        let node_count = self
          .type_kvs
          .get_nodes_by_exchange(exchange)
          .await?
          .count()
          .await;
        let min_node_init = config.min_node_init(exchange);
        info!(
          node_count = node_count,
          min_node_init = min_node_init;
          "Retrive number of nodes"
        );
        if node_count == min_node_init {
          let _ = self
            .init_lock
            .lock("observer_control_node_event_handler", || async {
              let sync_handler = SyncHandler::new(
                &self.db,
                self.kvs_cmd.clone().into(),
                &self.nats,
              )
              .await;
              if let Err(ref e) = sync_handler {
                error!("Synchronization Handler Initalization Failed: {}", e);
                return;
              };
              let mut sync_handler = sync_handler.unwrap();
              info!("Init Triggered");
              if let Err(e) = sync_handler.handle(&exchange).await {
                error!("Synchronization Handling Filed: {}", e);
              };
            })
            .await;
        } else if node_count > min_node_init {
          let _ = self
            .symbol_balancer
            .broadcast_equalization(exchange, 0)
            .await;
        }
      }
      TradeObserverNodeEvent::Unregist(exchange, symbols) => {
        let remover = NodeRemover::new(
          self.node_kvs.clone(),
          self.type_kvs.clone(),
          self.control_event.clone(),
          self.db.clone(),
        );
        remover.handle(node_id).await?;
      }
    }
    return Ok(());
  }
}

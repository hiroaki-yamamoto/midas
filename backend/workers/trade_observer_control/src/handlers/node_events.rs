use ::std::time::Duration;

use ::futures::future::try_join;
use ::futures::stream::StreamExt;
use ::uuid::Uuid;

use ::config::{Database, ObserverConfig};
use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::kvs::redis::Commands;
use ::kvs::traits::normal::{Expiration, ListOp, Lock, Set};
use ::kvs::{Connection, WriteOption};
use ::log::{as_error, error, info};
use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::natsJS::message::Message;
use ::subscribe::traits::Respond;
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

  /// Push NodeID to KVS
  /// Note that the return ID is not always the same as the input ID.
  /// E.g. When the id is duplicated, the new ID is generated and returned.
  /// Return Value: NodeID that pushed to KVS.
  async fn push_nodeid(
    &mut self,
    exchange: Exchanges,
    msg: &Message,
  ) -> ControlResult<Uuid> {
    let redis_option: Option<WriteOption> = WriteOption::default()
      .duration(Duration::from_secs(30).into())
      .non_existent_only(true)
      .into();
    let mut node_id = Uuid::new_v4();
    info!(node_id = node_id.to_string().as_str(); "NodeID Generated");
    loop {
      let node_id_txt = node_id.to_string();
      match self.type_kvs.index_node(node_id_txt.clone()).await {
        Ok(num) => {
          if num > 0 {
            info!(node_id = node_id.to_string(); "Node indexed");
            break;
          } else {
            node_id = Uuid::new_v4();
            continue;
          }
        }
        Err(e) => {
          error!(error = as_error!(e); "Failed to index node");
          sleep(Duration::from_secs(1)).await;
          continue;
        }
      }
    }
    let node_id_txt = node_id.to_string();
    self
      .type_kvs
      .set(
        &node_id_txt,
        exchange.as_str_name().into(),
        redis_option.clone(),
      )
      .await?;
    self
      .node_kvs
      .lpush::<usize>(&node_id_txt, vec!["".into()], redis_option.clone())
      .await?;
    self.node_kvs.lpop(&node_id_txt, None).await?;
    info!(node_id = node_id_txt; "Acquired NodeID");
    info!(node_id = node_id_txt; "Sending NodeID to Node");
    msg
      .respond(&TradeObserverControlEvent::NodeIDAssigned(node_id.clone()))
      .await?;
    info!(node_id = node_id_txt; "NodeID Sent");
    return Ok(node_id);
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
        match self.push_nodeid(exchange, msg).await {
          Ok(node_id) => {
            info!(
              "Node Connected. NodeID: {}, Exchange: {}",
              node_id,
              exchange.as_str_name()
            );
          }
          Err(e) => {
            error!(error = as_error!(e); "NodeID assignment failed");
          }
        }
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
      TradeObserverNodeEvent::Unregist(node_id) => {
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

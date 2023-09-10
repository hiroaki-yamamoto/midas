use ::std::time::Duration;

use ::uuid::Uuid;

use ::config::{Database, ObserverConfig};
use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::errors::KVSResult;
use ::kvs::redis::Commands;
use ::kvs::traits::normal::{Expiration, ListOp, Lock, Set};
use ::kvs::{Connection, WriteOption};
use ::log::{error, info};
use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::natsJS::message::Message;
use ::subscribe::traits::Respond;

use crate::balancer::SymbolBalancer;
use crate::dlock::InitLock;
use crate::errors::Result as ControlResult;

use super::SyncHandler;

pub(crate) struct FromNodeEventHandler<C>
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
    node_id: &Uuid,
    exchange: Exchanges,
    msg: &Message,
  ) -> ControlResult<Uuid> {
    let mut fixed_node_id = node_id.clone();
    let redis_option: Option<WriteOption> = WriteOption::default()
      .duration(Duration::from_secs(30).into())
      .non_existent_only(true)
      .into();
    loop {
      let push_result: KVSResult<usize> = self
        .node_kvs
        .lpush(node_id.to_string(), "".into(), redis_option.clone())
        .await;
      if push_result.is_ok() {
        self.node_kvs.lpop(node_id.to_string(), None).await?;
        break;
      }
      fixed_node_id = Uuid::new_v4();
    }
    self
      .type_kvs
      .set(
        fixed_node_id.to_string(),
        exchange.as_str_name().into(),
        redis_option,
      )
      .await?;
    if node_id != &fixed_node_id {
      msg
        .respond(&TradeObserverControlEvent::NodeIDChanged(
          node_id.clone(),
          fixed_node_id.clone(),
        ))
        .await?;
    }
    return Ok(fixed_node_id);
  }

  pub async fn handle(
    &mut self,
    msg: &Message,
    event: TradeObserverNodeEvent,
    config: &ObserverConfig,
  ) -> ControlResult<()> {
    match event {
      TradeObserverNodeEvent::Ping(node_id) => {
        self
          .node_kvs
          .expire(&node_id.to_string(), Duration::from_secs(30))
          .await?;
      }
      TradeObserverNodeEvent::Regist(exchange, node_id) => {
        if self.push_nodeid(&node_id, exchange, msg).await.is_ok() {
          info!(
            "Node Connected. NodeID: {}, Exchange: {}",
            node_id,
            exchange.as_str_name()
          );
        }
        let node_count = self.node_kvs.count_nodes().await?;
        let min_node_init = config.min_node_init(exchange);
        if node_count == min_node_init {
          let _ = self
            .init_lock
            .lock("observer_control_node_event_handler", || async {
              let mut sync_handler: SyncHandler<_> = SyncHandler::new(
                &self.db,
                self.kvs_cmd.clone().into(),
                &self.nats,
              );
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
      TradeObserverNodeEvent::Unregist(node_id) => {}
    }
    return Ok(());
  }
}

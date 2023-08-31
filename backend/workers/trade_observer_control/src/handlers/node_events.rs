use ::std::time::Duration;

use ::uuid::Uuid;

use ::config::{Database, ObserverConfig};
use ::dlock::Dlock;
use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::errors::KVSResult;
use ::kvs::redis::Commands;
use ::kvs::{Connection, SoftExpirationStore, WriteOption};
use ::log::{error, info};
use ::observers::kvs::{
  ONEXTypeKVS, ONEXTypeLastCheckedKVS, ObserverNodeKVS,
  ObserverNodeLastCheckKVS,
};
use ::rpc::entities::Exchanges;
use ::subscribe::natsJS::context::Context;
use ::symbols::traits::SymbolReader as SymbolReaderTrait;

use crate::dlock::InitLock;
use crate::errors::Result as ControlResult;

use super::SyncHandler;

pub(crate) struct FromNodeEventHandler<C>
where
  C: Commands + Send + Sync,
{
  kvs_cmd: Connection<C>,
  nats: Context,
  node_kvs: ObserverNodeKVS<C>,
  db: Database,
  type_kvs: ONEXTypeKVS<C>,
  last_check_kvs: ObserverNodeLastCheckKVS<C>,
  type_last_check_kvs: ONEXTypeLastCheckedKVS<C>,
  init_lock: InitLock<C>,
}

impl<C> FromNodeEventHandler<C>
where
  C: Commands + Send + Sync,
{
  pub fn new(kvs_com: Connection<C>, db: Database, nats: &Context) -> Self {
    return Self {
      kvs_cmd: kvs_com.clone().into(),
      nats: nats.clone(),
      node_kvs: ObserverNodeKVS::new(kvs_com.clone().into()),
      type_kvs: ONEXTypeKVS::new(kvs_com.clone().into()),
      last_check_kvs: ObserverNodeLastCheckKVS::new(kvs_com.clone().into()),
      type_last_check_kvs: ONEXTypeLastCheckedKVS::new(kvs_com.clone().into()),
      init_lock: InitLock::new(kvs_com.into()),
      db,
    };
  }

  /// Push NodeID to KVS
  /// Note that the return ID is not always the same as the input ID.
  /// E.g. When the id is duplicated, the new ID is generated and returned.
  /// Return Value: NodeID that pushed to KVS.
  fn push_nodeid(
    &mut self,
    node_id: &Uuid,
    exchange: Exchanges,
  ) -> KVSResult<Uuid> {
    let mut node_id = node_id.clone();
    let redis_option: Option<WriteOption> = WriteOption::default()
      .duration(Duration::from_secs(30).into())
      .non_existent_only(true)
      .into();
    loop {
      let push_result: KVSResult<usize> = self.node_kvs.lpush(
        &node_id.to_string(),
        "".into(),
        redis_option.clone(),
        &mut self.last_check_kvs,
      );
      if push_result.is_ok() {
        break;
      }
      node_id = Uuid::new_v4();
    }
    self.type_kvs.set(
      node_id.to_string(),
      exchange.as_str_name().into(),
      redis_option,
      &mut self.type_last_check_kvs,
    )?;
    return Ok(node_id);
  }

  pub async fn handle(
    &mut self,
    event: TradeObserverNodeEvent,
    config: &ObserverConfig,
  ) -> ControlResult<()> {
    match event {
      TradeObserverNodeEvent::Ping(node_id) => {
        self.node_kvs.expire(
          &node_id.to_string(),
          Duration::from_secs(30),
          &mut self.last_check_kvs,
        )?;
      }
      TradeObserverNodeEvent::Regist(exchange, node_id) => {
        if self.push_nodeid(&node_id, exchange).is_ok() {
          info!(
            "Node Connected. NodeID: {}, Exchange: {}",
            node_id,
            exchange.as_str_name()
          );
        }
        if self.node_kvs.count_nodes()? == config.min_node_init(exchange) {
          let _ = self
            .init_lock
            .lock(|| async {
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
        }
      }
      TradeObserverNodeEvent::Unregist(node_id) => {}
    }
    return Ok(());
  }
}

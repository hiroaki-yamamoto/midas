use ::std::time::Duration;

use ::uuid::Uuid;

use ::config::{Database, ObserverConfig};
use ::dlock::Dlock;
use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::errors::KVSResult;
use ::kvs::redis::Commands;
use ::kvs::Connection;
use ::kvs::{SoftExpirationStore, WriteOption};
use ::log::info;
use ::observers::kvs::{
  ONEXTypeKVS, ONEXTypeLastCheckedKVS, ObserverNodeKVS,
  ObserverNodeLastCheckKVS,
};
use ::rpc::entities::Exchanges;

use crate::dlock::InitLock;
use crate::errors::Result as ControlResult;

use super::SyncHandler;

pub(crate) struct FromNodeEventHandler<C>
where
  C: Commands + Send + Sync,
{
  kvs: ObserverNodeKVS<C>,
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
  pub fn new(kvs_com: Connection<C>, db: Database) -> Self {
    return Self {
      kvs: ObserverNodeKVS::new(kvs_com.clone().into()),
      type_kvs: ONEXTypeKVS::new(kvs_com.clone().into()),
      last_check_kvs: ObserverNodeLastCheckKVS::new(kvs_com.clone().into()),
      type_last_check_kvs: ONEXTypeLastCheckedKVS::new(kvs_com.clone().into()),
      init_lock: InitLock::new(kvs_com.into()),
      db,
    };
  }

  pub async fn handle(
    &mut self,
    event: TradeObserverNodeEvent,
    config: &ObserverConfig,
  ) -> ControlResult<()> {
    match event {
      TradeObserverNodeEvent::Ping(node_id) => {
        self.kvs.expire(
          &node_id.to_string(),
          Duration::from_secs(30),
          &mut self.last_check_kvs,
        )?;
      }
      TradeObserverNodeEvent::Regist(exchange, node_id) => {
        let mut node_id = node_id;
        let redis_option: Option<WriteOption> = WriteOption::default()
          .duration(Duration::from_secs(30).into())
          .non_existent_only(true)
          .into();
        loop {
          let push_result: KVSResult<usize> = self.kvs.lpush(
            &node_id.to_string(),
            "".into(),
            redis_option.clone(),
            &mut self.last_check_kvs,
          );
          match push_result {
            Ok(_) => {
              let _ = self.type_kvs.set(
                node_id.to_string(),
                exchange.as_str_name().into(),
                redis_option,
                &mut self.type_last_check_kvs,
              )?;
              info!(
                "Node Connected. NodeID: {}, Exchange: {}",
                node_id,
                exchange.as_str_name()
              );
              break;
            }
            Err(_) => {
              node_id = Uuid::new_v4();
            }
          }
          if self.kvs.count_nodes()? == config.min_node_init(exchange) {
            let _ = self
              .init_lock
              .lock(|| {
                info!("Init Triggered");
                unimplemented!("Not Implemented Yet");
              })
              .await;
          }
        }
      }
      TradeObserverNodeEvent::Unregist(node_id) => {}
    }
    return Ok(());
  }
}

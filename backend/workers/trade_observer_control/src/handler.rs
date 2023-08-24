use ::std::sync::{Arc, Mutex};
use ::std::time::Duration;

use ::uuid::Uuid;

use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::errors::KVSResult;
use ::kvs::redis::Commands;
use ::kvs::{SoftExpirationStore, WriteOption};
use ::log::info;
use ::observers::kvs::{
  ONEXTypeKVS, ONEXTypeLastCheckedKVS, ObserverNodeKVS,
  ObserverNodeLastCheckKVS,
};
use ::rpc::entities::Exchanges;

use crate::errors::Result as ControlResult;

pub(crate) struct FromNodeEventHandler<C>
where
  C: Commands,
{
  kvs: ObserverNodeKVS<C>,
  type_kvs: ONEXTypeKVS<C>,
  last_check_kvs: ObserverNodeLastCheckKVS<C>,
  type_last_check_kvs: ONEXTypeLastCheckedKVS<C>,
}

impl<C> FromNodeEventHandler<C>
where
  C: Commands,
{
  pub fn new(kvs_com: Arc<Mutex<C>>) -> Self {
    return Self {
      kvs: ObserverNodeKVS::new(kvs_com.clone()),
      type_kvs: ONEXTypeKVS::new(kvs_com.clone()),
      last_check_kvs: ObserverNodeLastCheckKVS::new(kvs_com.clone()),
      type_last_check_kvs: ONEXTypeLastCheckedKVS::new(kvs_com.clone()),
    };
  }

  pub fn handle(&mut self, event: TradeObserverNodeEvent) -> ControlResult<()> {
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
                &mut self.last_check_kvs,
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
        }
      }
      TradeObserverNodeEvent::Unregist(node_id) => {}
    }
    return Ok(());
  }
}

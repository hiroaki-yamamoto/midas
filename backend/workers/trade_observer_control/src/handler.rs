use ::std::time::Duration;

use ::uuid::Uuid;

use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::errors::KVSResult;
use ::kvs::redis::Commands;
use ::kvs::{SoftExpirationStore, WriteOption};
use ::log::info;
use ::observers::kvs::{ObserverNodeKVS, ObserverNodeLastCheckKVS};
use ::rpc::entities::Exchanges;

use crate::errors::Result as ControlResult;

pub(crate) fn events_from_node<T>(
  event: TradeObserverNodeEvent,
  kvs: &mut ObserverNodeKVS<T>,
  last_check_kvs: &mut ObserverNodeLastCheckKVS<T>,
) -> ControlResult<()>
where
  T: Commands,
{
  match event {
    TradeObserverNodeEvent::Ping(node_id) => {
      kvs.expire(
        &node_id.to_string(),
        Duration::from_secs(30),
        last_check_kvs,
      )?;
    }
    TradeObserverNodeEvent::Regist(exchange, node_id) => {
      let mut node_id = node_id;
      let redis_option: Option<WriteOption> = WriteOption::default()
        .duration(Duration::from_secs(30).into())
        .non_existent_only(true)
        .into();
      loop {
        let push_result: KVSResult<usize> = kvs.lpush(
          &node_id.to_string(),
          "".into(),
          redis_option.clone(),
          last_check_kvs,
        );
        match push_result {
          Ok(_) => {
            let _ = kvs.set(
              format!("{}:exchange", node_id),
              exchange.as_str_name().into(),
              redis_option,
              last_check_kvs,
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

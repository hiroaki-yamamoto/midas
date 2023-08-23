use ::std::time::Duration;

use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::kvs::redis::Commands;
use ::kvs::{SoftExpirationStore, WriteOption};
use ::observers::kvs::{ObserverNodeKVS, ObserverNodeLastCheckKVS};

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
    TradeObserverNodeEvent::Regist(node_id, exchange) => {}
    TradeObserverNodeEvent::Unregist(node_id) => {}
  }
  return Ok(());
}

use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::std::time::Duration;

use ::kvs::redis::Commands;
use ::kvs::Store;
use ::observers::kvs::ObserverNodeKVS;

use crate::errors::Result as ControlResult;

pub(crate) fn events_from_node<T>(
  event: TradeObserverNodeEvent,
  kvs: &mut ObserverNodeKVS<T>,
) -> ControlResult<()>
where
  T: Commands,
{
  match event {
    TradeObserverNodeEvent::Ping(node_id) => {
      kvs.expire(&node_id.to_string(), Duration::from_secs(60))?;
    }
    TradeObserverNodeEvent::Regist(node_id, exchange) => {}
    TradeObserverNodeEvent::Unregist(node_id) => {}
  }
  return Ok(());
}

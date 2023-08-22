use ::std::time::{Duration, SystemTime};

use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::kvs::redis::Commands;
use ::kvs::{Store, WriteOption};
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
      let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
      kvs.expire(&node_id.to_string(), Duration::from_secs(30))?;
      last_check_kvs.set(
        &node_id.to_string(),
        now,
        WriteOption::default()
          .duration(Duration::from_secs(30).into())
          .into(),
      )?;
    }
    TradeObserverNodeEvent::Regist(node_id, exchange) => {}
    TradeObserverNodeEvent::Unregist(node_id) => {}
  }
  return Ok(());
}

use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};

use ::kvs::redis::Commands;
use ::observers::kvs::ObserverNodeKVS;

pub fn events_from_node<T>(event: TradeObserverNodeEvent, raw_kvs: &mut T)
where
  T: Commands,
{
  match event {
    TradeObserverNodeEvent::Ping(node_id) => {}
    TradeObserverNodeEvent::Regist(node_id, exchange) => {}
    TradeObserverNodeEvent::Unregist(node_id) => {}
  }
}

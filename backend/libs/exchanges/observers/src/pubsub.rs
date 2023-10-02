use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};

use ::subscribe::pubsub;

// UUID is an identifier for a node. "Exchanges" is an exchange that
// the observer handles.
pubsub!(
  pub,
  NodeEventPubSub,
  TradeObserverNodeEvent,
  "TradeObserverNodeEvent",
);

// First UUID is the old node ID, second UUID is the new node ID.
pubsub!(
  pub,
  NodeControlEventPubSub,
  TradeObserverControlEvent,
  "TradeObserverControlEvent",
);

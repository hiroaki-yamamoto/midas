use ::entities::uuid::Uuid;
use ::entities::TradeObserverControlEvent;

use ::rpc::entities::Exchanges;
use ::subscribe::pubsub;

// UUID is an identifier for a node. "Exchanges" is an exchange that
// the observer handles.
pubsub!(pub, NodeRegisterPubSub, (Exchanges, Uuid), "NodeRegister",);

// First UUID is the old node ID, second UUID is the new node ID.
pubsub!(
  pub,
  NodeControlEventPubSub,
  TradeObserverControlEvent,
  "TradeObserverControlEvent",
);

mod dlock;

use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::observers::pubsub::{NodeControlEventPubSub, NodeEventPubSub};

use ::config;

#[tokio::main]
async fn main() {
  config::init(|cfg, sig, db, broker, host| async {}).await;
}

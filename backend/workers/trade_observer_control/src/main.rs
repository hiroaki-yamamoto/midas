mod dlock;

use ::log::info;
use ::tokio::select;

use ::entities::{TradeObserverControlEvent, TradeObserverNodeEvent};
use ::observers::pubsub::{NodeControlEventPubSub, NodeEventPubSub};

use ::config;

#[tokio::main]
async fn main() {
  info!("Starting trade_observer_control");
  config::init(|cfg, mut sig, db, broker, host| async move {
    loop {
      select! {
        _ = sig.recv() => {
          break;
        },
      };
    }
  })
  .await;
  info!("Stopping trade_observer_control");
}

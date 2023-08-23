mod dlock;
mod errors;
mod handler;

use ::futures::StreamExt;
use ::log::{error, info};
use ::tokio::select;

use ::observers::kvs::{ObserverNodeKVS, ObserverNodeLastCheckKVS};
use ::observers::pubsub::NodeEventPubSub;
use ::subscribe::PubSub;

use ::config;

#[tokio::main]
async fn main() {
  info!("Starting trade_observer_control");
  config::init(|cfg, mut sig, db, broker, host| async move {
    let kvs = cfg.redis().unwrap();
    let node_event_pubsub = NodeEventPubSub::new(&broker).await.unwrap();
    let mut node_kvs = ObserverNodeKVS::new(kvs.clone());
    let mut node_last_check_kvs = ObserverNodeLastCheckKVS::new(kvs);
    let mut node_event = node_event_pubsub
      .queue_subscribe("tradeObserverController")
      .await
      .unwrap();
    loop {
      select! {
        event = node_event.next() => if let Some((event, _)) = event {
          if let Err(e) = handler::events_from_node(event, &mut node_kvs, &mut node_last_check_kvs) {
            error!("Error handling node event: {}", e);
          }
        },
        _ = sig.recv() => {
          break;
        },
      };
    }
  })
  .await;
  info!("Stopping trade_observer_control");
}

mod balancer;
mod dlock;
mod errors;
mod handlers;

use ::futures::StreamExt;
use ::log::{error, info};
use ::tokio::select;

use ::observers::pubsub::NodeEventPubSub;
use ::subscribe::PubSub;

use ::config;

#[tokio::main]
async fn main() {
  info!("Starting trade_observer_control");
  config::init(|cfg, mut sig, db, broker, _| async move {
    let kvs = cfg.redis().unwrap();
    let node_event_pubsub = NodeEventPubSub::new(&broker).await.unwrap();
    let mut node_event_handler = handlers::FromNodeEventHandler::new(
      kvs, db, &broker
    ).await.unwrap();
    let mut node_event = node_event_pubsub
      .pull_subscribe("tradeObserverController")
      .await
      .unwrap();
    loop {
      select! {
        event = node_event.next() => if let Some((event, msg)) = event {
          if let Err(e) = node_event_handler.handle(&msg, event, &cfg.observer).await {
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

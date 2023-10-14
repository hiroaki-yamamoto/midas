mod dlock;
mod errors;
mod handlers;

use ::std::time::Duration;

use ::futures::StreamExt;
use ::log::{error, info};
use ::tokio::select;
use ::tokio::time::interval;

use ::observers::pubsub::NodeEventPubSub;
use ::subscribe::PubSub;
use ::symbols::pubsub::SymbolEventPubSub;

use ::config;

#[tokio::main]
async fn main() {
  info!("Starting trade_observer_control");
  config::init(|cfg, mut sig, db, broker, _| async move {
    let kvs = cfg.redis().unwrap();
    let node_event_pubsub = NodeEventPubSub::new(&broker).await.unwrap();
    let symbol_event_pubsub = SymbolEventPubSub::new(&broker).await.unwrap();

    let rotted_node_removal_handler = handlers::RemoveRotHandler::new(
      kvs.clone().into(), &broker
    ).await.unwrap();
    let mut node_event_handler = handlers::FromNodeEventHandler::new(
      kvs, db, &broker
    ).await.unwrap();
    let symbol_event_handler = handlers::SymbolEventHandler::new(
      &broker
    ).await.unwrap();

    let mut node_event = node_event_pubsub
      .pull_subscribe("tradeObserverController")
      .await
      .unwrap();
    let mut symbol_event = symbol_event_pubsub
      .pull_subscribe("tradeObserverController")
      .await
      .unwrap();
    let rot_dur = Duration::from_secs(10);
    let mut auto_unregist_check_interval = interval(rot_dur);
    loop {
      select! {
        Some((event, _)) = node_event.next() => {
          if let Err(e) = node_event_handler.handle(event, &cfg.observer).await {
            error!("Error handling node event: {}", e);
            continue;
          }
        },
        Some((event, _)) = symbol_event.next() => {
          if let Err(e) = symbol_event_handler.handle(event).await {
            error!("Error handling symbol event: {}", e);
            continue;
          }
        },
        _ = sig.recv() => {
          break;
        },
        _ = auto_unregist_check_interval.tick() => {
          if let Err(e) = rotted_node_removal_handler.handle(rot_dur).await {
            error!("Error handling rotted node removal event: {}", e);
            continue;
          }
        }
      }
    }
  })
  .await;
  info!("Stopping trade_observer_control");
}

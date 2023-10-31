use ::std::sync::Arc;
use ::std::time::Duration;

use ::futures::future::{try_join, try_join_all};

use ::config::{Database, ObserverConfig};
use ::kvs::redis::aio::MultiplexedConnection;
use ::kvs::traits::last_checked::Expiration;
use ::observers::entities::{
  TradeObserverControlEvent, TradeObserverNodeEvent,
};
use ::observers::kvs::{NODE_EXCHANGE_TYPE_KVS_BUILDER, NODE_KVS_BUILDER};
use ::observers::pubsub::NodeControlEventPubSub;
use ::subscribe::nats::Client as Nats;
use ::subscribe::traits::PubSub;

use crate::errors::Result as ControlResult;

pub struct FromNodeEventHandler {
  control_event: NodeControlEventPubSub,
  node_kvs: Arc<dyn Expiration<Commands = MultiplexedConnection> + Send + Sync>,
  type_kvs: Arc<dyn Expiration<Commands = MultiplexedConnection> + Send + Sync>,
}

impl FromNodeEventHandler {
  pub async fn new(
    kvs_com: MultiplexedConnection,
    _: Database,
    nats: &Nats,
  ) -> ControlResult<Self> {
    let control_event = NodeControlEventPubSub::new(nats).await?;
    let node_kvs =
      Arc::new(NODE_EXCHANGE_TYPE_KVS_BUILDER.build(kvs_com.clone()));
    let type_kvs = Arc::new(NODE_KVS_BUILDER.build(kvs_com.clone()));
    return Ok(Self {
      control_event,
      node_kvs,
      type_kvs,
    });
  }

  pub async fn handle(
    &mut self,
    event: TradeObserverNodeEvent,
    _: &ObserverConfig,
  ) -> ControlResult<()> {
    match event {
      TradeObserverNodeEvent::Ping(node_id) => {
        log::debug!("Ping from {}", node_id);
        let node_id = Arc::new(node_id);
        try_join(
          self
            .node_kvs
            .expire(node_id.clone(), Duration::from_secs(30)),
          self.type_kvs.expire(node_id, Duration::from_secs(30)),
        )
        .await?;
      }
      TradeObserverNodeEvent::Regist(_) => {
        log::error!("Unimplemented");
      }
      TradeObserverNodeEvent::Unregist(exchange, symbols) => {
        let publish_defer = symbols.into_iter().map(|symbol| {
          self
            .control_event
            .publish(TradeObserverControlEvent::SymbolAdd(
              exchange,
              symbol.clone(),
            ))
        });
        let _ = try_join_all(publish_defer).await?;
      }
    }
    return Ok(());
  }
}

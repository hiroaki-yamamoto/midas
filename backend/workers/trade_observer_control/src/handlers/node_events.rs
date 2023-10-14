use ::std::time::Duration;

use ::futures::future::{try_join, try_join_all};

use ::config::{Database, ObserverConfig};
use ::kvs::redis::Commands;
use ::kvs::traits::normal::Expiration;
use ::kvs::Connection;
use ::observers::entities::{
  TradeObserverControlEvent, TradeObserverNodeEvent,
};
use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;
use ::subscribe::nats::Client as Nats;
use ::subscribe::traits::PubSub;

use crate::errors::Result as ControlResult;

pub struct FromNodeEventHandler<C>
where
  C: Commands + Send + Sync,
{
  control_event: NodeControlEventPubSub,
  node_kvs: ObserverNodeKVS<C>,
  type_kvs: ONEXTypeKVS<C>,
}

impl<C> FromNodeEventHandler<C>
where
  C: Commands + Send + Sync,
{
  pub async fn new(
    kvs_com: Connection<C>,
    _: Database,
    nats: &Nats,
  ) -> ControlResult<Self> {
    let control_event = NodeControlEventPubSub::new(nats).await?;
    let node_kvs = ObserverNodeKVS::new(kvs_com.clone().into());
    let type_kvs = ONEXTypeKVS::new(kvs_com.clone().into());
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
        try_join(
          self
            .node_kvs
            .expire(&node_id.to_string(), Duration::from_secs(30)),
          self
            .type_kvs
            .expire(&node_id.to_string(), Duration::from_secs(30)),
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

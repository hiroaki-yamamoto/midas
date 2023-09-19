use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;

use ::entities::TradeObserverControlEvent;
use ::observers::pubsub::NodeControlEventPubSub;
use ::symbols::entities::SymbolEvent;

use crate::errors::Result as ControlResult;

pub struct SymbolEventHandler {
  control_event: NodeControlEventPubSub,
}

impl SymbolEventHandler {
  pub async fn new(con: &Nats) -> ControlResult<Self> {
    return Ok(Self {
      control_event: NodeControlEventPubSub::new(con).await?,
    });
  }

  pub async fn handle(&self, event: SymbolEvent) -> ControlResult<()> {
    let payload: TradeObserverControlEvent = event.into();
    self.control_event.publish(payload).await?;
    return Ok(());
  }
}

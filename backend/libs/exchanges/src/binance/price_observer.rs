use ::nats::asynk::Connection as Broker;

pub struct PriceObserver {
  broker: Broker,
}

impl PriceObserver {
  pub fn new(broker: Broker) -> Self {
    return Self { broker };
  }
}

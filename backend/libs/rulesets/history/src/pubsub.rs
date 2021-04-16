use ::nats::Connection as Broker;
use ::subscribe::PubSub;

use ::types::stateful_setter;

use super::entities::KlineFetchStatus;

#[derive(Debug, Clone)]
pub struct FetchStatusPubSub {
  con: Broker,
}

impl FetchStatusPubSub {
  pub fn new(con: Broker) -> Self {
    return Self { con };
  }
  stateful_setter!(con, Broker);
}

impl PubSub<KlineFetchStatus> for FetchStatusPubSub {
  fn get_broker(&self) -> &Broker {
    return &self.con;
  }
  fn get_subject(&self) -> &str {
    return "kline.progress";
  }
}

use ::nats::Connection as Broker;

use ::history::entities::KlineFetchStatus;
use ::rpc::historical::HistChartProg;
use ::subscribe::PubSub;
use ::types::stateful_setter;

use super::constants::HIST_FETCHER_FETCH_PROG_SUB_NAME;

pub struct HistProgPartPubSub {
  con: Broker,
}

impl HistProgPartPubSub {
  pub fn new(con: Broker) -> Self {
    return Self { con };
  }
  stateful_setter!(con, Broker);
}

impl PubSub<HistChartProg> for HistProgPartPubSub {
  fn get_broker(&self) -> &Broker {
    return &self.con;
  }
  fn get_subject(&self) -> &str {
    return HIST_FETCHER_FETCH_PROG_SUB_NAME;
  }
}

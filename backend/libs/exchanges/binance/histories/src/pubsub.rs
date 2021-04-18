use ::nats::Connection as Broker;

use ::rpc::historical::HistChartProg;
use ::subscribe::{impl_pubsub, PubSub};

use super::entities::Param;

#[derive(Debug, Clone)]
pub struct HistProgPartPubSub {
  con: Broker,
}

impl_pubsub!(
  HistProgPartPubSub,
  HistChartProg,
  "binance.kline.fetch.param"
);

#[derive(Debug, Clone)]
pub(crate) struct HistFetchParamPubSub {
  con: Broker,
}

impl_pubsub!(HistFetchParamPubSub, Param, "binance.kline.fetch.param");

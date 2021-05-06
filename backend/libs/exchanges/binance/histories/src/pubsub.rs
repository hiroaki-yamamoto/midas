use ::entities::KlineCtrl;
use ::rpc::historical::HistChartProg;
use ::subscribe::pubsub;

use super::entities::{KlinesWithInfo, Param};

pubsub!(
  pub,
  HistProgPartPubSub,
  HistChartProg,
  "binance.kline.fetch.param"
);
pubsub!(pub(crate), HistFetchParamPubSub, Param, "binance.kline.fetch.param");
pubsub!(pub(crate), HistFetchRespPubSub, KlinesWithInfo, "binance.kline.fetch.resp");
pubsub!(pub(crate), RecLatestTradeDatePubSub, Vec<String>, "binance.kline.record.latest");
pubsub!(pub(crate), KlineControlPubSub, KlineCtrl, "binance.kline.ctrl");

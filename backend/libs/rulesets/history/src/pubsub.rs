use ::rpc::historical::HistChartProg;
use ::subscribe::PubSub;

use super::entities::KlineFetchStatus;

pub trait HistProgPartPubSub: PubSub<HistChartProg> {}

pub trait FetchStatusPubSub: PubSub<KlineFetchStatus> {
  fn get_subject(&self) -> &str {
    return "kline.progress";
  }
}

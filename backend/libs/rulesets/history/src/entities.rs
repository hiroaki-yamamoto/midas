use ::serde::{Deserialize, Serialize};

use ::rpc::entities::Exchanges;
use ::rpc::historical::HistChartProg;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum KlineFetchStatus {
  ProgressChanged {
    exchange: Exchanges,
    previous: Option<HistChartProg>,
    current: HistChartProg,
  },
  Report {
    exchange: Exchanges,
    progress: HistChartProg,
  },
  Done(String),
  Stop,
}

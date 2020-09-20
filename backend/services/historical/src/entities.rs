use ::serde::{Deserialize, Serialize};

use ::rpc::entities::Exchanges;
use ::rpc::historical::HistChartProg;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum KlineFetchStatus {
  Progress {
    exchange: Exchanges,
    progress: HistChartProg,
  },
  Stop,
}

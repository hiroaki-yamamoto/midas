use ::serde::{Deserialize, Serialize};

use ::rpc::historical::HistChartProg;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum KlineFetchStatus {
  Ready,
  WIP(HistChartProg),
  Completed,
}

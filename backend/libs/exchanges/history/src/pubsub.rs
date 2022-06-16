use ::entities::HistoryFetchRequest;
use ::subscribe::pubsub;

use crate::entities::FetchStatusChanged;

pubsub!(
  pub,
  HistChartPubSub,
  HistoryFetchRequest,
  "histChart.request"
);
pubsub!(
  pub,
  HistChartDateSplitPubSub,
  HistoryFetchRequest,
  "histChart.splitDate"
);
pubsub!(
  pub,
  FetchStatusEventPubSub,
  FetchStatusChanged,
  "histChart.progChanged"
);

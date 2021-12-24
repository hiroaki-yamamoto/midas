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
  RawHistChartPubSub,
  HistoryFetchRequest,
  "histChart.request.raw"
);
pubsub!(
  pub,
  FetchStatusEventPubSub,
  FetchStatusChanged,
  "histChart.progChanged"
);

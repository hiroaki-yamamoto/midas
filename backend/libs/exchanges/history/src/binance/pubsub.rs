use ::entities::HistoryFetchRequest;
use ::subscribe::pubsub;

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

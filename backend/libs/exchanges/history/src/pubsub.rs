use ::entities::HistoryFetchRequest;
use ::subscribe::pubsub;

use crate::entities::FetchStatusChanged;

pubsub!(
  pub,
  HistChartPubSub,
  HistoryFetchRequest,
  "HistChartRequest",
);
pubsub!(
  pub,
  HistChartDateSplitPubSub,
  HistoryFetchRequest,
  "HistChartDateSplit",
);
pubsub!(
  pub,
  FetchStatusEventPubSub,
  FetchStatusChanged,
  "HistChartProgressChanged",
);

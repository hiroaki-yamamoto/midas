use ::entities::HistoryFetchRequest;
use ::subscribe::pubsub;

use crate::entities::FetchStatusChanged;

pubsub!(
  pub,
  HistChartPubSub,
  HistoryFetchRequest,
  "HistChartRequestStream",
  "HistChartRequestConsumer",
  "HistChartRequest",
);
pubsub!(
  pub,
  HistChartDateSplitPubSub,
  HistoryFetchRequest,
  "HistChartDateSplitStream",
  "HistChartDateSplitConsumer",
  "HistChartDateSplit",
);
pubsub!(
  pub,
  FetchStatusEventPubSub,
  FetchStatusChanged,
  "HistChartProgressChangedStream",
  "HistChartProgressChangedConSumer",
  "HistChartProgressChanged",
);

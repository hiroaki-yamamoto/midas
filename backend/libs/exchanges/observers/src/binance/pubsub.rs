use ::subscribe::pubsub;

use super::entities::BookTicker;

pubsub!(
  pub,
  BookTickerPubSub,
  BookTicker<f64>,
  "BinanceTradesStream",
  "BinanceTradesConsumer",
  "BinanceTrades",
);

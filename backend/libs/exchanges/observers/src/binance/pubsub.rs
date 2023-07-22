use ::rug::Float;

use ::subscribe::pubsub;

use super::entities::BookTicker;

pubsub!(pub, BookTickerPubSub, BookTicker<Float>, "BinanceTrades",);

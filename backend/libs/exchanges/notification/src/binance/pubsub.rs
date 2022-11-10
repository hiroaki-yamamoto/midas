use super::entities::CastedUserStreamEvents;
use super::entities::ListenKeyPair;
use ::subscribe::pubsub;

pubsub!(
  pub,
  NotifyPubSub,
  CastedUserStreamEvents,
  "BinanceUserStreamNotifyStream",
  "BinanceUserStreamNotifyConsumer",
  "BinanceUserStreamNotify",
);

pubsub!(
  pub,
  ReauthPubSub,
  String,
  "BinanceUserStreamReAuthStream",
  "BinanceUserStreamReAuthConsumer",
  "BinanceUserStreamReAuth",
);
pubsub!(
  pub,
  ListenKeyPubSub,
  ListenKeyPair,
  "BinanceUserStreamListenStream",
  "BinanceUserStreamListenConsumer",
  "BinanceUserStreamListenKey",
);

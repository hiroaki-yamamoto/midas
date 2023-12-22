use super::entities::CastedUserStreamEvents;
use super::entities::ListenKeyPair;
use ::subscribe::pubsub;

pubsub!(
  pub,
  NotifyPubSub,
  CastedUserStreamEvents,
  "BinanceUserStreamNotify",
);

pubsub!(pub, ReauthPubSub, String, "BinanceUserStreamReAuth",);
pubsub!(
  pub,
  ListenKeyPubSub,
  ListenKeyPair,
  "BinanceUserStreamListenKey",
);

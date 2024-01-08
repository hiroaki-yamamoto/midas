use super::entities::CastedUserStreamEvents;
use super::entities::ListenKeyPair;
use ::keychain::APIKey;
use ::subscribe::pubsub;

pubsub!(
  pub,
  NotifyPubSub,
  CastedUserStreamEvents,
  "BinanceUserStreamNotify",
);

pubsub!(pub, ReauthPubSub, APIKey, "BinanceUserStreamReAuth",);
pubsub!(
  pub,
  ListenKeyPubSub,
  ListenKeyPair,
  "BinanceUserStreamListenKey",
);

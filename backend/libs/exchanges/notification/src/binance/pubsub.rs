use super::entities::CastedUserStreamEvents;
use super::entities::ListenKeyPair;
use ::subscribe::pubsub;

pubsub!(
  pub,
  NotifyPubSub,
  CastedUserStreamEvents,
  "binance.user_stream.notify"
);

pubsub!(pub, ReauthPubSub, String, "binance.user_stream.reauth");
pubsub!(
  pub,
  ListenKeyPubSub,
  ListenKeyPair,
  "binance.user_stream.listen_key"
);

use ::entities::APIKeyEvent;
use ::subscribe::pubsub;

pubsub!(
  pub,
  APIKeyPubSub,
  APIKeyEvent,
  "APIKeyStream",
  "APIKeyConsumer",
  "APIKey",
);

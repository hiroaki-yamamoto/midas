use ::entities::APIKey;
use ::subscribe::pubsub;

pubsub!(pub, APIKeyPubSub, APIKey, "apikey");

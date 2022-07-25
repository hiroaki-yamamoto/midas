use ::entities::APIKeyEvent;
use ::subscribe::pubsub;

pubsub!(pub, APIKeyPubSub, APIKeyEvent, "APIKey");

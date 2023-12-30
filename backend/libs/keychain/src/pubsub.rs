use ::subscribe::pubsub;

use crate::entities::APIKeyEvent;

pubsub!(pub, APIKeyPubSub, APIKeyEvent, "APIKey",);

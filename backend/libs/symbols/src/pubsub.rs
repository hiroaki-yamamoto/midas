use ::subscribe::pubsub;

use super::entities::SymbolEvent;

pubsub!(pub, SymbolEventPubSub, SymbolEvent, "BinanceSymbolEvents",);

use ::subscribe::pubsub;

use super::entities::Symbol;

pubsub!(
  pub,
  SymbolRemovalEventPubSub,
  Symbol,
  "binance.symbol.events.remove"
);

pubsub!(
  pub,
  SymbolAddEventPubSub,
  Symbol,
  "binance.symbol.events.remove"
);

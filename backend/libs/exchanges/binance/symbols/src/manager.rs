use ::std::collections::HashSet;

use ::nats::Connection as NatsCon;
use ::slog::Logger;

use super::entities::Symbol;
use super::pubsub::{SymbolAddEventPubSub, SymbolRemovalEventPubSub};
use ::subscribe::PubSub;

#[derive(Debug, Clone)]
pub struct SymbolUpdateEventManager {
  pub to_add: Vec<Symbol>,
  pub to_remove: Vec<Symbol>,
  pub add_event: SymbolAddEventPubSub,
  pub del_event: SymbolRemovalEventPubSub,
  pub log: Logger,
}

impl SymbolUpdateEventManager {
  pub fn new<S, T>(log: Logger, broker: NatsCon, new: S, old: T) -> Self
  where
    S: IntoIterator<Item = Symbol> + Clone,
    T: IntoIterator<Item = Symbol> + Clone,
  {
    let new_keys: HashSet<String> =
      new.clone().into_iter().map(|item| item.symbol).collect();
    let old_keys: HashSet<String> =
      old.clone().into_iter().map(|item| item.symbol).collect();

    let to_add: Vec<String> = (&new_keys - &old_keys).into_iter().collect();
    let to_add = new
      .into_iter()
      .filter(move |item| to_add.contains(&item.symbol))
      .collect();

    let to_remove: Vec<String> = (&old_keys - &new_keys).into_iter().collect();
    let to_remove = old
      .into_iter()
      .filter(move |item| to_remove.contains(&item.symbol))
      .collect();
    return Self {
      log,
      to_add,
      to_remove,
      add_event: SymbolAddEventPubSub::new(broker.clone()),
      del_event: SymbolRemovalEventPubSub::new(broker.clone()),
    };
  }

  pub async fn publish_changes(&self) {
    for add_item in &self.to_add[..] {
      if let Err(e) = self.add_event.publish(&add_item) {
        ::slog::warn!(
          self.log,
          "Failed to publish the newly added symbol";
          "symbol" => add_item.symbol.to_owned(),
          "error" => e,
        );
      };
    }
    for del_item in &self.to_remove[..] {
      if let Err(e) = self.del_event.publish(&del_item) {
        ::slog::warn!(
          self.log,
          "Failed to publish the deleted symbol";
          "symbol" => del_item.symbol.to_owned(),
          "error" => e,
        );
      }
    }
    return;
  }
}

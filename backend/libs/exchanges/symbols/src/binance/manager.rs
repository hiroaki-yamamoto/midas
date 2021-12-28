use ::std::collections::HashSet;

use ::nats::Connection as NatsCon;
use ::slog::Logger;

use super::entities::{Symbol, SymbolEvent};
use super::pubsub::SymbolEventPubSub;
use ::subscribe::PubSub;

#[derive(Debug, Clone)]
pub struct SymbolUpdateEventManager {
  pub to_add: Vec<Symbol>,
  pub to_remove: Vec<Symbol>,
  pub event: SymbolEventPubSub,
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
      event: SymbolEventPubSub::new(broker.clone()),
    };
  }

  pub async fn publish_changes(&self) {
    for add_item in &self.to_add[..] {
      if let Err(e) = self.event.publish(&SymbolEvent::Add(add_item.clone())) {
        ::slog::warn!(
          self.log,
          "Failed to publish the newly added symbol";
          "symbol" => add_item.symbol.to_owned(),
          "error" => e,
        );
      };
    }
    for del_item in &self.to_remove[..] {
      if let Err(e) = self.event.publish(&SymbolEvent::Remove(del_item.clone()))
      {
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
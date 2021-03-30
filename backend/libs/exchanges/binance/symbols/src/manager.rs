use ::std::collections::HashSet;

use ::nats::Connection as NatsCon;
use ::rmp_serde::to_vec as to_msgpack;
use ::slog::Logger;

use super::constants::{SYMBOL_ADD_EVENT, SYMBOL_REMOVE_EVENT};

use super::entities::Symbol;

#[derive(Debug, Clone)]
pub struct SymbolUpdateEventManager<'s, 't> {
  pub to_add: Vec<Symbol>,
  pub to_remove: Vec<Symbol>,
  pub broker: &'t NatsCon,
  pub log: &'s Logger,
}

impl<'s, 't> SymbolUpdateEventManager<'s, 't> {
  pub fn new<S, T>(log: &'s Logger, broker: &'t NatsCon, new: S, old: T) -> Self
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
      broker,
    };
  }

  pub async fn publish_changes(&self) {
    for add_item in &self.to_add[..] {
      let msg = match to_msgpack(&add_item) {
        Err(e) => {
          ::slog::warn!(
            self.log,
            "Failed to encode symbol addition event message: {}",
            e
          );
          return;
        }
        Ok(v) => v,
      };
      if let Err(e) = self
        .broker
        .publish(SYMBOL_ADD_EVENT, msg.as_slice().to_owned())
      {
        ::slog::warn!(
          self.log,
          "Failed to publish the newly added symbol";
          "symbol" => add_item.symbol.to_owned(),
          "error" => e,
        );
      };
    }
    for del_item in &self.to_remove[..] {
      let msg = match to_msgpack(&del_item) {
        Err(e) => {
          ::slog::warn!(
            self.log,
            "Failed to encode symbol deletion event message: {}",
            e
          );
          return;
        }
        Ok(v) => v,
      };
      if let Err(e) = self
        .broker
        .publish(SYMBOL_REMOVE_EVENT, msg.as_slice().to_owned())
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

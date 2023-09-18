use ::std::collections::HashSet;

use ::errors::CreateStreamResult;
use ::log::{as_error, error};
use ::subscribe::nats::Client as Nats;

use super::entities::Symbol;
use crate::entities::SymbolEvent;
use crate::pubsub::SymbolEventPubSub;
use ::subscribe::PubSub;

#[derive(Debug, Clone)]
pub struct SymbolUpdateEventManager {
  pub to_add: Vec<Symbol>,
  pub to_remove: Vec<Symbol>,
  pub event: SymbolEventPubSub,
}

impl SymbolUpdateEventManager {
  pub async fn new<S, T>(
    broker: &Nats,
    new: S,
    old: T,
  ) -> CreateStreamResult<Self>
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
    return Ok(Self {
      to_add,
      to_remove,
      event: SymbolEventPubSub::new(broker).await?,
    });
  }

  pub async fn publish_changes(&self) {
    for add_item in &self.to_add[..] {
      if let Err(e) =
        self.event.publish(&SymbolEvent::Add(add_item.into())).await
      {
        error!(
          symbol = add_item.symbol.to_owned(),
          error = as_error!(e);
          "Failed to publish the newly added symbol",
        );
      };
    }
    for del_item in &self.to_remove[..] {
      if let Err(e) = self
        .event
        .publish(&SymbolEvent::Remove(del_item.into()))
        .await
      {
        error!(
          symbol = del_item.symbol.to_owned(),
          error = as_error!(e);
          "Failed to publish the deleted symbol",
        );
      }
    }
    return;
  }
}

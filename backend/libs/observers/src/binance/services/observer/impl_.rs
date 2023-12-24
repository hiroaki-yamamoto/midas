use ::std::collections::HashSet;
use ::std::sync::Arc;

use ::futures::future::try_join_all;
use ::mongodb::Database;
use ::subscribe::nats::client::Client as Nats;
use ::tokio_stream::StreamMap;

use ::errors::{CreateStreamResult, ObserverResult};
use ::random::generate_random_txt;
use ::rpc::exchanges::Exchanges;
use ::symbols::get_reader;
use ::symbols::pubsub::SymbolEventPubSub;

use crate::binance::{
  pubsub::BookTickerPubSub, sockets::BookTickerSocket,
  sockets::IBookTickerSocket,
};

use super::TradeObserver;

impl TradeObserver {
  pub async fn new(broker: &Nats, db: &Database) -> CreateStreamResult<Self> {
    let pubsub = BookTickerPubSub::new(&broker).await?;
    let symbol_event = SymbolEventPubSub::new(&broker).await?;
    let symbol_reader = get_reader(db, Exchanges::Binance.into()).await;
    return Ok(Self {
      pubsub: Arc::new(pubsub),
      symbol_reader: Arc::from(symbol_reader),
      symbol_event: Arc::new(symbol_event),
      sockets: StreamMap::new(),
    });
  }

  pub(super) async fn subscribe(
    &mut self,
    symbols: &[String],
  ) -> ObserverResult<()> {
    let not_subscribed: HashSet<String> = symbols
      .into_iter()
      .filter(|symbol| {
        !self
          .sockets
          .values()
          .any(|socket| socket.has_symbol(symbol))
      })
      .map(|symbol| symbol.to_string())
      .collect();
    for i in (0..not_subscribed.len()).step_by(10) {
      let symbols: Vec<String> = not_subscribed
        .iter()
        .skip(i)
        .take(10)
        .map(|s| s.to_string())
        .collect();
      let mut socket = BookTickerSocket::new().await?;
      socket.subscribe(&symbols).await?;
      let mut id = generate_random_txt(16);
      while self.sockets.contains_key(&id) {
        id = generate_random_txt(16);
      }
      self.sockets.insert(generate_random_txt(16), socket.into());
    }

    return Ok(());
  }

  pub(super) async fn unsubscribe(
    &mut self,
    symbols: &[String],
  ) -> ObserverResult<()> {
    let to_remove: Vec<String> = symbols
      .iter()
      .filter(|symbol| {
        self
          .sockets
          .values()
          .any(|socket| socket.has_symbol(symbol))
      })
      .map(|symbol| symbol.to_string())
      .collect();
    // Send unsubscribe request.
    try_join_all(
      self
        .sockets
        .values_mut()
        .map(|socket| socket.unsubscribe(&to_remove)),
    )
    .await?;
    // Remove the unused sockets from manager.
    let empty_socket_ids: Vec<String> = self
      .sockets
      .iter()
      .filter(|(_, socket)| socket.len() < 1)
      .map(|(id, _)| id.to_string())
      .collect();
    empty_socket_ids.iter().for_each(|id| {
      self.sockets.remove(id);
    });
    return Ok(());
  }
}

use ::std::collections::{HashMap, HashSet};
use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::mongodb::Database;
use ::rug::Float;
use ::subscribe::nats::client::Client as Nats;
use ::tokio::signal::unix::Signal;
use ::tokio_stream::StreamMap;

use ::errors::{CreateStreamResult, ObserverResult};
use ::rpc::exchanges::Exchanges;
use ::subscribe::PubSub;
use ::symbols::entities::SymbolEvent;
use ::symbols::get_reader;
use ::symbols::pubsub::SymbolEventPubSub;
use ::symbols::traits::SymbolReader;

use crate::binance::{
  entities::{BookTicker, WebsocketPayload},
  interfaces::IBookTickerSubscription,
  pubsub::BookTickerPubSub,
  sockets::BookTickerSocket,
};
use crate::traits::ITradeObserver;

pub struct TradeObserver {
  pubsub: Arc<dyn PubSub<Output = BookTicker<Float>> + Send + Sync>,
  sockets: HashMap<usize, BookTickerSocket>,
  symbol_reader: Arc<dyn SymbolReader + Send + Sync>,
  symbol_event: Arc<dyn PubSub<Output = SymbolEvent> + Send + Sync>,
}

impl TradeObserver {
  pub async fn new(broker: &Nats, db: &Database) -> CreateStreamResult<Self> {
    let pubsub = BookTickerPubSub::new(&broker).await?;
    let symbol_event = SymbolEventPubSub::new(&broker).await?;
    let symbol_reader = get_reader(db, Exchanges::Binance.into()).await;
    return Ok(Self {
      pubsub: Arc::new(pubsub),
      symbol_reader: Arc::from(symbol_reader),
      symbol_event: Arc::new(symbol_event),
      sockets: HashMap::new(),
    });
  }

  fn get_socket(&mut self) -> Option<&mut BookTickerSocket> {
    let mut socket_index = self.sockets.len();
    socket_index = if socket_index < 1 {
      0
    } else {
      socket_index - 1
    };
    let socket = self.sockets.get_mut(&socket_index);
    if let Some(socket) = socket {
      if socket.len() < 100 && socket.len_socket() < 10 {
        return Some(socket);
      }
    }
    return None;
  }

  async fn subscribe(&mut self, symbols: &[String]) -> ObserverResult<()> {
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
      let socket = self.get_socket();
      if let Some(socket) = socket {
        socket.subscribe(&symbols).await?;
      } else {
        let mut socket = BookTickerSocket::new().await?;
        socket.subscribe(&symbols).await?;
        self.sockets.insert(self.sockets.len(), socket);
      }
    }

    return Ok(());
  }
}

#[async_trait]
impl ITradeObserver for TradeObserver {
  async fn start(&self, signal: Box<Signal>) -> ObserverResult<()> {
    return Ok(());
  }
}

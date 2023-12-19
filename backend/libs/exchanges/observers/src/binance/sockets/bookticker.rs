use ::std::collections::HashMap;
use ::std::ops::Drop;
use ::std::task::Poll;

use ::async_trait::async_trait;
use ::errors::{ObserverResult, ParseResult};
use ::futures::ready;
use ::futures::{SinkExt, Stream, StreamExt};
use ::log::{as_error, as_serde, debug, info};
use ::random::generate_random_txt;
use ::rug::Float;

use ::clients::binance::WS_ENDPOINT;
use ::round_robin_client::WebSocket;

use crate::binance::entities::{
  BookTicker, SubscribeRequest, SubscribeRequestInner, WebsocketPayload,
};
use crate::binance::interfaces::{BookTickerStream, IBookTickerSocket};

pub struct BookTickerSocket {
  socket: WebSocket<WebsocketPayload, SubscribeRequest>,
  symbols: HashMap<String, Vec<String>>,
  pending: HashMap<String, Vec<String>>,
}

impl BookTickerSocket {
  pub async fn new() -> ObserverResult<Self> {
    let socket = WebSocket::new(&[WS_ENDPOINT.to_string()]).await?;
    let inst = Self {
      socket,
      symbols: HashMap::new(),
      pending: HashMap::new(),
    };
    return Ok(inst);
  }

  fn random_id(&self) -> String {
    let mut random_id = generate_random_txt(36);
    while self.symbols.contains_key(&random_id) {
      random_id = generate_random_txt(36);
    }
    return random_id;
  }

  fn parse_payload(
    &mut self,
    payload: WebsocketPayload,
  ) -> Option<BookTicker<Float>> {
    match payload {
      WebsocketPayload::BookTicker(book_ticker) => {
        info!(payload = as_serde!(book_ticker); "Received Payload");
        let book_ticker: ParseResult<BookTicker<Float>> =
          book_ticker.try_into();
        return match book_ticker {
          Ok(book_ticker) => Some(book_ticker),
          Err(error) => {
            info!(error=as_error!(error); "Error");
            None
          }
        };
      }
      WebsocketPayload::Error(error) => {
        info!(error=as_serde!(&error); "Error");
      }
      WebsocketPayload::Result(result) => {
        let id = result.id.clone();
        let symbols = self.pending.remove(&id);
        if let Some(symbols) = symbols {
          self.symbols.insert(id, symbols);
        }
        info!(result=as_serde!(&result); "Result");
      }
    }
    return None;
  }
}

#[async_trait]
impl IBookTickerSocket for BookTickerSocket {
  fn has_symbol(&self, symbol: &str) -> bool {
    for subscribed_symbols in self.symbols.values() {
      if subscribed_symbols.contains(&symbol.to_string()) {
        return true;
      }
    }
    return false;
  }

  fn len(&self) -> usize {
    let len: usize = self.symbols.values().fold(0, |acc, lst| acc + lst.len());
    return len;
  }

  fn len_socket(&self) -> usize {
    return self.symbols.len();
  }

  async fn resubscribe(&mut self) -> ObserverResult<()> {
    let pending: HashMap<String, Vec<String>> = self.pending.drain().collect();
    for (id, symbols) in pending {
      info!(id = as_serde!(id), symbols=as_serde!(symbols); "Resubscribe");
      let _ = self.unsubscribe(&symbols).await;
      self.subscribe(&symbols).await?;
    }
    return Ok(());
  }

  async fn subscribe(&mut self, symbols: &[String]) -> ObserverResult<()> {
    let id = self.random_id();
    let payload = SubscribeRequestInner {
      id: id.clone(),
      params: symbols
        .iter()
        .map(|symbol| format!("{}@bookTicker", symbol))
        .collect(),
    }
    .into_subscribe();
    debug!(payload = as_serde!(payload); "Start Subscribe");
    self.socket.send(payload).await?;
    self.pending.insert(id, symbols.to_vec());
    return Ok(());
  }

  async fn unsubscribe(&mut self, symbols: &[String]) -> ObserverResult<()> {
    let payloads: Vec<SubscribeRequest> = symbols
      .iter()
      .filter(|symbol| self.has_symbol(symbol))
      .map(|symbol| {
        let id = self
          .symbols
          .iter()
          .find(|(_, v)| v.contains(symbol))
          .map(|(k, _)| k)
          .unwrap();
        return SubscribeRequestInner {
          id: id.clone(),
          params: vec![format!("{}@bookTicker", symbol)],
        }
        .into_unsubscribe();
      })
      .collect();
    for payload in payloads {
      debug!(payload = as_serde!(payload); "Start Unsubscribe");
      self.socket.send(payload).await?;
    }
    // Remove symbols from the map
    for subscribed_symbols in self.symbols.values_mut() {
      subscribed_symbols.retain(|symbol| !symbols.contains(symbol));
    }
    self.symbols.retain(|_, v| !v.is_empty());

    return Ok(());
  }
}

impl From<BookTickerSocket> for BookTickerStream {
  fn from(socket: BookTickerSocket) -> Self {
    return Box::pin(socket);
  }
}

impl Stream for BookTickerSocket {
  type Item = BookTicker<Float>;

  fn poll_next(
    mut self: ::std::pin::Pin<&mut Self>,
    cx: &mut ::std::task::Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let payload = ready!(self.socket.poll_next_unpin(cx));
    return match payload {
      None => Poll::Ready(None),
      Some(payload) => {
        let book_ticker = self.parse_payload(payload);
        Poll::Ready(book_ticker)
      }
    };
  }
}

impl Drop for BookTickerSocket {
  fn drop(&mut self) {
    let _ = self.socket.close();
  }
}

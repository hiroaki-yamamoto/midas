use ::std::collections::HashMap;
use ::std::ops::Drop;
use ::std::pin::Pin;

use ::async_trait::async_trait;
use ::errors::ObserverResult;
use ::futures::future::try_join_all;
use ::futures::{SinkExt, Stream, StreamExt};

use ::clients::binance::WS_ENDPOINT;
use ::round::WebSocket;

use crate::binance::entities::{
  SubscribeRequest, SubscribeRequestInner, WebsocketPayload,
};
use crate::binance::interfaces::{BookTickerStream, IBookTickerSocket};

pub struct BookTickerSocket {
  param_id: u64,
  socket: WebSocket<WebsocketPayload, SubscribeRequest>,
  symbols: HashMap<u64, Vec<String>>,
}

impl BookTickerSocket {
  pub async fn new() -> ObserverResult<Self> {
    let socket = WebSocket::new(&[WS_ENDPOINT.to_string()]).await?;
    return Ok(Self {
      param_id: 0,
      socket,
      symbols: HashMap::new(),
    });
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

  async fn subscribe(&mut self, symbols: &[String]) -> ObserverResult<()> {
    let payload = SubscribeRequestInner {
      id: self.param_id,
      params: symbols
        .iter()
        .map(|symbol| format!("{}@bookTicker", symbol))
        .collect(),
    }
    .into_subscribe();
    self.socket.send(payload).await?;
    self.symbols.insert(self.param_id, symbols.to_vec());
    self.param_id += 1;
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
          id: *id,
          params: vec![format!("{}@bookTicker", symbol)],
        }
        .into_unsubscribe();
      })
      .collect();
    for payload in payloads {
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
    return Box::new(socket);
  }
}

impl Stream for BookTickerSocket {
  type Item = WebsocketPayload;

  fn poll_next(
    mut self: ::std::pin::Pin<&mut Self>,
    cx: &mut ::std::task::Context<'_>,
  ) -> ::std::task::Poll<Option<Self::Item>> {
    return self.socket.poll_next_unpin(cx);
  }
}

impl Drop for BookTickerSocket {
  fn drop(&mut self) {
    let _ = self.socket.close();
  }
}

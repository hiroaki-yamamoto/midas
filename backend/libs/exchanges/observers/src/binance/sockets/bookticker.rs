use ::std::collections::HashMap;

use ::async_trait::async_trait;
use ::errors::ObserverResult;
use ::futures::{SinkExt, Stream, StreamExt};

use ::clients::binance::WS_ENDPOINT;
use ::round::WebSocket;

use crate::binance::entities::{
  SubscribeRequest, SubscribeRequestInner, WebsocketPayload,
};
use crate::binance::interfaces::IBookTickerSubscription;

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
impl IBookTickerSubscription for BookTickerSocket {
  async fn has_symbol(&self, symbol: &str) -> ObserverResult<bool> {
    for subscribed_symbols in self.symbols.values() {
      if subscribed_symbols.iter().any(|s| s == symbol) {
        return Ok(true);
      }
    }
    return Ok(false);
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
    let payloads: Vec<SubscribeRequest> = self
      .symbols
      .iter()
      .filter(|(_, v)| {
        symbols.iter().any(|symbol| v.iter().any(|s| s == symbol))
      })
      .map(|(k, _)| {
        let payload = SubscribeRequestInner {
          id: *k,
          params: vec![],
        }
        .into_unsubscribe();
        return payload;
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

impl Stream for BookTickerSocket {
  type Item = WebsocketPayload;

  fn poll_next(
    mut self: ::std::pin::Pin<&mut Self>,
    cx: &mut ::std::task::Context<'_>,
  ) -> ::std::task::Poll<Option<Self::Item>> {
    return self.socket.poll_next_unpin(cx);
  }
}

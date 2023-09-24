use ::futures::sink::SinkExt;
use ::std::collections::HashMap;

use ::clients::binance::WS_ENDPOINT;
use ::errors::{ObserverResult, WebSocketInitResult};
use ::round::WebSocket;

use crate::binance::entities::{
  SubscribeRequest, SubscribeRequestInner, WebsocketPayload,
};

const MAX_NUM_PARAMS: u64 = 5;

pub type BookTickerSocket = WebSocket<WebsocketPayload, SubscribeRequest>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Cursor {
  pub socket: usize,
  pub param: u64,
}

#[derive(Default)]
pub struct BookTickerHandler {
  sockets: Vec<BookTickerSocket>,
  symbol_index: HashMap<String, Cursor>,
  cur: Cursor,
}

impl BookTickerHandler {
  pub fn new() -> Self {
    return Self::default();
  }

  async fn get_or_new_socket(
    &mut self,
  ) -> WebSocketInitResult<&mut BookTickerSocket> {
    if self.sockets.is_empty()
      || self.sockets.len() < self.cur.socket
      || self.cur.param >= MAX_NUM_PARAMS
    {
      let socket = WebSocket::new(&[WS_ENDPOINT.to_string()]).await?;
      self.sockets.push(socket);
      self.cur.socket = self.sockets.len() - 1;
      self.cur.param = 0;
      return Ok(self.sockets.get_mut(self.cur.socket).unwrap());
    }
    return Ok(&mut self.sockets[self.cur.socket]);
  }

  /// Reference: https://binance-docs.github.io/apidocs/spot/en/#individual-symbol-book-ticker-streams
  pub async fn subscribe(&mut self, symbols: &[&str]) -> ObserverResult<()> {
    let id = self.cur.param.clone();
    let socket = self.get_or_new_socket().await?;
    let payload = SubscribeRequestInner {
      id,
      params: symbols
        .iter()
        .map(|symbol| format!("{}@bookTicker", symbol))
        .collect(),
    }
    .into_subscribe();
    socket.send(payload).await?;
    socket.flush().await?;
    symbols.into_iter().for_each(|&symbol| {
      self.symbol_index.insert(symbol.into(), self.cur.clone());
    });
    self.cur.param += 1;
    return Ok(());
  }
}

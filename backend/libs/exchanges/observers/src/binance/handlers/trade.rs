use ::std::collections::HashMap;
use ::std::sync::Arc;

use ::futures::future::try_join_all;
use ::futures::sink::SinkExt;
use ::tokio::sync::Mutex;
use ::tokio_stream::StreamMap;

use ::clients::binance::WS_ENDPOINT;
use ::errors::{ObserverResult, WebSocketInitResult, WebsocketSinkError};
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
  sockets: StreamMap<usize, BookTickerSocket>,
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
      self.sockets.insert(self.sockets.len(), socket);
      self.cur.socket = self.sockets.len() - 1;
      self.cur.param = 0;
      return Ok(&mut self.sockets.iter_mut().last().unwrap().1);
    }
    return Ok(&mut self.sockets.iter_mut().nth(self.cur.socket).unwrap().1);
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

  fn build_params(
    &mut self,
    symbols: &[&str],
  ) -> HashMap<Cursor, SubscribeRequestInner> {
    let mut params_dict: HashMap<Cursor, SubscribeRequestInner> =
      HashMap::new();
    for symbol in symbols {
      let cursor = self.symbol_index.remove(*symbol);
      if cursor.is_none() {
        continue;
      }

      let cursor = cursor.unwrap();
      match params_dict.get_mut(&cursor) {
        Some(req_inner) => {
          req_inner.params.push(format!("{}@bookTicker", symbol));
        }
        None => {
          params_dict.insert(
            cursor.clone(),
            SubscribeRequestInner {
              id: cursor.param,
              params: vec![format!("{}@bookTicker", symbol)],
            },
          );
        }
      };
    }
    return params_dict;
  }

  pub async fn unsubscribe(&mut self, symbols: &[&str]) -> ObserverResult<()> {
    let requests: HashMap<usize, SubscribeRequest> = self
      .build_params(symbols)
      .into_iter()
      .map(|(cur, inner)| (cur.socket, inner.into_unsubscribe()))
      .collect();
    let sockets = Arc::new(Mutex::new(&mut self.sockets));
    let mut defer = vec![];
    for (socket_id, req) in requests {
      let sockets = sockets.clone();
      defer.push(async move {
        let mut sockets = sockets.lock().await;
        let socket_id = socket_id.clone();
        if let Some((_, socket)) = sockets.iter_mut().nth(socket_id) {
          socket.send(req).await?;
          socket.flush().await?;
        }
        Ok::<_, WebsocketSinkError>(())
      })
    }
    try_join_all(defer).await?;
    return Ok(());
  }
}

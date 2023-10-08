use ::std::collections::HashMap;
use ::std::future::Future;
use ::std::sync::Arc;

use ::futures::future::try_join_all;
use ::futures::sink::SinkExt;
use ::futures::stream::StreamExt;
use ::log::{as_error, as_serde, error, info};
use ::rug::Float;
use ::tokio::select;
use ::tokio::sync::watch::Receiver;
use ::tokio::sync::Mutex;
use ::tokio_stream::StreamMap;

use ::clients::binance::WS_ENDPOINT;
use ::errors::{ObserverResult, WebSocketInitResult, WebsocketSinkError};
use ::round::WebSocket;
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;

use crate::binance::entities::{
  BookTicker, SubscribeRequest, SubscribeRequestInner, WebsocketPayload,
};
use crate::binance::pubsub::BookTickerPubSub;

const MAX_NUM_PARAMS: u64 = 5;

pub type BookTickerSocket = WebSocket<WebsocketPayload, SubscribeRequest>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Cursor {
  pub socket: usize,
  pub param: u64,
}

pub struct BookTickerHandler {
  sockets: StreamMap<usize, BookTickerSocket>,
  symbol_index: HashMap<String, Cursor>,
  cur: Cursor,
  pubsub: BookTickerPubSub,
}

impl BookTickerHandler {
  pub async fn new(pubsub: &Nats) -> ObserverResult<Self> {
    let me = Self {
      sockets: StreamMap::new(),
      symbol_index: HashMap::new(),
      cur: Cursor::default(),
      pubsub: BookTickerPubSub::new(pubsub).await?,
    };

    return Ok(me);
  }

  pub fn start(
    self: Box<Self>,
    sig: Receiver<bool>,
  ) -> impl Future<Output = ObserverResult<()>> {
    let me = Arc::new(Mutex::new(*self));
    return Self::event_loop(me, sig);
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
  pub async fn subscribe(&mut self, symbols: &[String]) -> ObserverResult<()> {
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
    symbols.into_iter().for_each(|symbol| {
      self.symbol_index.insert(symbol.into(), self.cur.clone());
    });
    self.cur.param += 1;
    return Ok(());
  }

  fn build_params(
    &mut self,
    symbols: &[String],
  ) -> HashMap<Cursor, SubscribeRequestInner> {
    let mut params_dict: HashMap<Cursor, SubscribeRequestInner> =
      HashMap::new();
    for symbol in symbols {
      let cursor = self.symbol_index.remove(symbol);
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

  pub async fn unsubscribe(
    &mut self,
    symbols: &[String],
  ) -> ObserverResult<()> {
    let requests: HashMap<usize, SubscribeRequest> = self
      .build_params(symbols)
      .into_iter()
      .map(|(cur, inner)| (cur.socket, inner.into_unsubscribe()))
      .collect();
    let mut defer = vec![];
    let mut sockets = self.sockets.iter_mut();
    for (socket_id, req) in requests {
      if let Some((_, socket)) = sockets.find(|&&mut (id, _)| id == socket_id) {
        defer.push(async {
          socket.send(req).await?;
          socket.flush().await?;
          Ok::<_, WebsocketSinkError>(())
        });
      }
    }
    try_join_all(defer).await?;
    return Ok(());
  }

  async fn event_loop(
    me: Arc<Mutex<Self>>,
    mut sig: Receiver<bool>,
  ) -> ObserverResult<()> {
    loop {
      let mut me = me.lock().await;
      select! {
        _ = sig.changed() => {
          break;
        },
        Some((_, payload)) = me.sockets.next() => {
          match payload {
            WebsocketPayload::Result(result) => {
              info!(id = result.id; "Request accepted");
            }
            WebsocketPayload::Error(error) => {
              error!(error = as_serde!(error); "Request rejected");
            }
            WebsocketPayload::BookTicker(ticker) => {
              let ticker = match BookTicker::<Float>::try_from(ticker) {
                Ok(ticker) => ticker,
                Err(e) => {
                  error!(error = as_error!(e); "Failed to parse book ticker");
                  continue;
                }
              };
              if let Err(e) = me.pubsub.publish(&ticker).await {
                error!(error = as_error!(e); "Failed to publish book ticker");
                continue;
              }
            }
          }
        },
      }
    }
    return Ok(());
  }
}

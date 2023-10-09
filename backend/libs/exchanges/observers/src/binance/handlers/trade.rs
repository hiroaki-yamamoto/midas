use ::std::collections::HashMap;
use ::std::future::Future;
use ::std::sync::Arc;

use ::futures::future::try_join_all;
use ::futures::sink::SinkExt;
use ::futures::stream::StreamExt;
use ::log::{as_error, as_serde, error, info};
use ::rug::Float;
use ::tokio::select;
use ::tokio::sync::{mpsc, watch};
use ::tokio::sync::{Mutex, RwLock};
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
  subscribe_rx: Arc<Mutex<mpsc::UnboundedReceiver<Vec<String>>>>,
  subscribe_tx: mpsc::UnboundedSender<Vec<String>>,
  unsub_rx: Arc<Mutex<mpsc::UnboundedReceiver<Vec<String>>>>,
  unsub_tx: mpsc::UnboundedSender<Vec<String>>,
  cur: Cursor,
  pubsub: BookTickerPubSub,
}

impl BookTickerHandler {
  pub async fn new(pubsub: &Nats) -> ObserverResult<Self> {
    let (subscribe_tx, subscribe_rx) = mpsc::unbounded_channel();
    let (unsub_tx, unsub_rx) = mpsc::unbounded_channel();
    let me = Self {
      sockets: StreamMap::new(),
      symbol_index: HashMap::new(),
      subscribe_rx: Arc::new(Mutex::new(subscribe_rx)),
      subscribe_tx,
      unsub_rx: Arc::new(Mutex::new(unsub_rx)),
      unsub_tx,
      cur: Cursor::default(),
      pubsub: BookTickerPubSub::new(pubsub).await?,
    };

    return Ok(me);
  }

  pub fn start(
    me: Arc<RwLock<Self>>,
    sig: watch::Receiver<bool>,
  ) -> impl Future<Output = ObserverResult<()>> {
    // let me = Arc::new(RwLock::new(*self));
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
  async fn handle_subscribe(
    &mut self,
    symbols: Vec<String>,
  ) -> ObserverResult<()> {
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
    symbols.into_iter().for_each(|symbol| {
      self.symbol_index.insert(symbol.into(), self.cur.clone());
    });
    self.cur.param += 1;
    return Ok(());
  }

  pub fn subscribe(&self, symbols: &[String]) -> ObserverResult<()> {
    info!(symbols = as_serde!(symbols); "Signaling subscribe event");
    return Ok(self.subscribe_tx.send(symbols.into())?);
  }

  fn build_params(
    &mut self,
    symbols: Vec<String>,
  ) -> HashMap<Cursor, SubscribeRequestInner> {
    let mut params_dict: HashMap<Cursor, SubscribeRequestInner> =
      HashMap::new();
    for symbol in symbols {
      let cursor = self.symbol_index.remove(&symbol);
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

  pub async fn handle_unsubscribe(
    &mut self,
    symbols: Vec<String>,
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
          Ok::<_, WebsocketSinkError>(())
        });
      }
    }
    try_join_all(defer).await?;
    return Ok(());
  }

  pub fn unsubscribe(&self, symbols: Vec<String>) -> ObserverResult<()> {
    return Ok(self.unsub_tx.send(symbols)?);
  }

  async fn event_loop(
    me: Arc<RwLock<Self>>,
    mut sig: watch::Receiver<bool>,
  ) -> ObserverResult<()> {
    let subscribe_rx = {
      let me = me.read().await;
      me.subscribe_rx.clone()
    };
    let mut subscribe_rx = subscribe_rx.lock().await;
    let unsub_rx = {
      let me = me.read().await;
      me.unsub_rx.clone()
    };
    let mut unsub_rx = unsub_rx.lock().await;
    loop {
      let mut me = me.write().await;
      select! {
        _ = sig.changed() => {
          break;
        },
        Some(symbols) = subscribe_rx.recv() => {
          info!(symbols = as_serde!(symbols); "Received subscribe event");
          if let Err(e) = me.handle_subscribe(symbols).await {
            error!(error = as_error!(e); "Failed to subscribe");
          };
        },
        Some(symbols) = unsub_rx.recv() => {
          info!(symbols = as_serde!(symbols); "Received unsubscribe event");
          if let Err(e) = me.handle_unsubscribe(symbols).await {
            error!(error = as_error!(e); "Failed to unsubscribe");
          };
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

use ::std::collections::HashMap;
use ::std::sync::Arc;

use ::futures::future::try_join_all;
use ::futures::sink::SinkExt;
use ::futures::stream::StreamExt;
use ::log::{as_error, as_serde, error, info};
use ::rug::Float;
use ::tokio::select;
use ::tokio::sync::{mpsc, oneshot, watch};
use ::tokio::sync::{Mutex, RwLock};
use ::tokio_stream::StreamMap;

use ::clients::binance::WS_ENDPOINT;
use ::errors::{
  ObserverError, ObserverResult, WebSocketInitResult, WebsocketSinkError,
};
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
  symbol_index: HashMap<String, Cursor>,
  subscribe_rx: Arc<Mutex<mpsc::UnboundedReceiver<Vec<String>>>>,
  subscribe_tx: mpsc::UnboundedSender<Vec<String>>,
  unsub_rx: Arc<Mutex<mpsc::UnboundedReceiver<Vec<String>>>>,
  unsub_tx: mpsc::UnboundedSender<Vec<String>>,
  regist_socket_rx:
    Arc<Mutex<mpsc::UnboundedReceiver<oneshot::Sender<Cursor>>>>,
  regist_socket_tx: mpsc::UnboundedSender<oneshot::Sender<Cursor>>,
  send_payload_rx:
    Arc<Mutex<mpsc::UnboundedReceiver<(SubscribeRequest, Cursor)>>>,
  send_payload_tx: mpsc::UnboundedSender<(SubscribeRequest, Cursor)>,
  cur: RwLock<Cursor>,
  pubsub: BookTickerPubSub,
}

impl BookTickerHandler {
  pub async fn new(pubsub: &Nats) -> ObserverResult<Self> {
    let (subscribe_tx, subscribe_rx) = mpsc::unbounded_channel();
    let (unsub_tx, unsub_rx) = mpsc::unbounded_channel();
    let (regist_socket_tx, regist_socket_rx) = mpsc::unbounded_channel();
    let (send_payload_tx, send_payload_rx) = mpsc::unbounded_channel();
    let me = Self {
      symbol_index: HashMap::new(),
      subscribe_rx: Arc::new(Mutex::new(subscribe_rx)),
      subscribe_tx,
      unsub_rx: Arc::new(Mutex::new(unsub_rx)),
      unsub_tx,
      regist_socket_rx: Arc::new(Mutex::new(regist_socket_rx)),
      regist_socket_tx,
      send_payload_rx: Arc::new(Mutex::new(send_payload_rx)),
      send_payload_tx,
      cur: RwLock::new(Cursor::default()),
      pubsub: BookTickerPubSub::new(pubsub).await?,
    };

    return Ok(me);
  }

  async fn get_or_new_socket(
    sockets: &mut StreamMap<usize, BookTickerSocket>,
    current: &mut Cursor,
  ) -> WebSocketInitResult<usize> {
    let sockets_size = sockets.len();
    if sockets.is_empty()
      || sockets_size < current.socket
      || current.param >= MAX_NUM_PARAMS
    {
      let socket = WebSocket::new(&[WS_ENDPOINT.to_string()]).await?;
      sockets.insert(sockets_size, socket);
      current.socket = sockets_size - 1;
      current.param = 0;
    }
    return Ok(current.socket);
  }

  /// Reference: https://binance-docs.github.io/apidocs/spot/en/#individual-symbol-book-ticker-streams
  async fn handle_subscribe(
    &mut self,
    symbols: Vec<String>,
  ) -> ObserverResult<()> {
    let (socket_id_tx, socket_id_rx) = oneshot::channel();
    self.regist_socket_tx.send(socket_id_tx);
    let socket_id = match socket_id_rx.await {
      Ok(socket_id) => socket_id,
      Err(e) => {
        return Err(ObserverError::Other(format!(
          "Failed to receive socket id: {:?}",
          e
        )));
      }
    };
    let payload = SubscribeRequestInner {
      id: socket_id.param,
      params: symbols
        .iter()
        .map(|symbol| format!("{}@bookTicker", symbol))
        .collect(),
    }
    .into_subscribe();
    self.send_payload_tx.send((payload, socket_id));
    let cur = { self.cur.read().await };
    symbols.into_iter().for_each(|symbol| {
      self.symbol_index.insert(symbol.into(), cur.clone());
    });
    {
      let mut cur = self.cur.write().await;
      cur.param += 1;
    }
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

  pub async fn start_subscribe_event_loop(
    me: Arc<RwLock<Self>>,
    mut sig: watch::Receiver<bool>,
  ) -> ObserverResult<()> {
    let subscribe_rx = {
      let me = me.read().await;
      me.subscribe_rx.clone()
    };
    let mut subscribe_rx = subscribe_rx.lock().await;
    loop {
      select! {
        _ = sig.changed() => {
          break;
        },
        Some(symbols) = subscribe_rx.recv() => {
          info!(symbols = as_serde!(symbols); "Received subscribe event");
          if let Err(e) = {
            let mut me = me.write().await;
            if let Err(e) = me.handle_subscribe(symbols).await {
              error!(error = as_error!(e); "Failed to handle subscribe");
            }
          } {
            error!(error = as_error!(e); "Failed to subscribe");
          };
        },
      }
    }
    return Ok(());
  }

  pub async fn start_unsubscribe_event_loop(
    me: Arc<RwLock<Self>>,
    mut sig: watch::Receiver<bool>,
  ) -> ObserverResult<()> {
    let unsub_rx = {
      let me = me.read().await;
      me.unsub_rx.clone()
    };
    let mut unsub_rx = unsub_rx.lock().await;
    loop {
      select! {
        _ = sig.changed() => {
          break;
        },
        Some(symbols) = unsub_rx.recv() => {
          info!(symbols = as_serde!(symbols); "Received unsubscribe event");
          if let Err(e) = {
            let mut me = me.write().await;
            me.handle_unsubscribe(symbols).await
          } {
            error!(error = as_error!(e); "Failed to unsubscribe");
          };
        }
      }
    }
    return Ok(());
  }

  pub async fn socket_event_loop(
    &self,
    mut sig: watch::Receiver<bool>,
  ) -> ObserverResult<()> {
    let mut sockets: StreamMap<usize, BookTickerSocket> = StreamMap::new();
    let mut regist_socket_rx = self.regist_socket_rx.lock().await;
    let mut send_payload_rx = self.send_payload_rx.lock().await;
    loop {
      select! {
        _ = sig.changed() => {
          break;
        },
        Some((payload, cur)) = send_payload_rx.recv() => {
          match sockets.iter_mut().find(|&&mut (id, _)| id == cur.socket) {
            Some((_, socket)) => {
              if let Err(e) = socket.send(payload).await {
                error!(error = as_error!(e); "Failed to send payload");
              };
            },
            None => {
              error!(id = cur.socket; "Socket not found");
            }
          };
        },
        Some(socket_id_tx) = regist_socket_rx.recv() => {
          let mut cur = self.cur.write().await;
          match Self::get_or_new_socket(&mut sockets, &mut cur).await {
            Err(e) => {
              error!(error = as_error!(e); "Failed to register a socket");
            },
            Ok(socket_id) => {
              info!(id = socket_id; "Socket registered");
              socket_id_tx.send(cur);
            }
          }
        },
        Some((_, payload)) = sockets.next() => {
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
              if let Err(e) = self.pubsub.publish(&ticker).await {
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

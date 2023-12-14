use ::std::collections::HashMap;
use ::std::sync::Arc;

use ::futures::sink::SinkExt;
use ::futures::stream::StreamExt;
use ::log::{as_error, as_serde, error, info};
use ::rug::Float;
use ::serde::Serialize;
use ::tokio::select;
use ::tokio::sync::{mpsc, oneshot, watch};
use ::tokio::sync::{Mutex, RwLock};
use ::tokio_stream::StreamMap;

use ::clients::binance::WS_ENDPOINT;
use ::errors::{ObserverError, ObserverResult, WebSocketInitResult};
use ::round::WebSocket;
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;

use crate::binance::entities::{
  BookTicker, SubscribeRequest, SubscribeRequestInner, WebsocketPayload,
};
use crate::binance::pubsub::BookTickerPubSub;

const MAX_NUM_PARAMS: u64 = 5;

pub type BookTickerSocket = WebSocket<WebsocketPayload, SubscribeRequest>;

enum SocketCommand {
  Regist(oneshot::Sender<Cursor>),
  Send(SubscribeRequest, Cursor),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize)]
struct Cursor {
  pub socket: usize,
  pub param: u64,
}

pub struct BookTickerHandler {
  symbol_index: Mutex<HashMap<String, Cursor>>,
  cmd_tx: mpsc::UnboundedSender<SocketCommand>,
  term_tx: watch::Sender<bool>,
  cur: Arc<RwLock<Cursor>>,
}

impl Drop for BookTickerHandler {
  fn drop(&mut self) {
    let _ = self.term_tx.send(true);
  }
}

impl BookTickerHandler {
  pub async fn new(pubsub: &Nats) -> ObserverResult<Self> {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let (term_tx, term_rx) = watch::channel(false);
    let current = Arc::new(RwLock::new(Cursor::default()));
    let pubsub = BookTickerPubSub::new(pubsub).await?;
    ::tokio::spawn(Self::socket_event_loop(
      current.clone(),
      pubsub,
      term_rx.clone(),
      cmd_rx,
    ));
    let me = Self {
      symbol_index: Mutex::new(HashMap::new()),
      cmd_tx,
      term_tx,
      cur: current,
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
      current.socket = sockets_size;
      current.param = 0;
      info!(current_ids = as_serde!(current); "New socket created");
    }
    return Ok(current.socket);
  }

  /// Reference: https://binance-docs.github.io/apidocs/spot/en/#individual-symbol-book-ticker-streams
  async fn handle_subscribe(&self, symbols: Vec<String>) -> ObserverResult<()> {
    let (socket_id_tx, socket_id_rx) = oneshot::channel();
    if let Err(e) = self.cmd_tx.send(SocketCommand::Regist(socket_id_tx)) {
      return Err(ObserverError::Other(format!(
        "Failed to request socket id: {:?}",
        e
      )));
    };
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
    if let Err(e) = self.cmd_tx.send(SocketCommand::Send(payload, socket_id)) {
      return Err(ObserverError::Other(format!(
        "Failed to send payload: {:?}",
        e
      )));
    }
    {
      let cur = self.cur.read().await;
      let mut symbol_index = self.symbol_index.lock().await;
      symbols.into_iter().for_each(|symbol| {
        symbol_index.insert(symbol.into(), cur.clone());
      });
    }
    {
      let mut cur = self.cur.write().await;
      cur.param += 1;
    }
    return Ok(());
  }

  pub async fn subscribe(&self, symbols: &[String]) -> ObserverResult<()> {
    info!(symbols = as_serde!(symbols); "Signaling subscribe event");
    return self.handle_subscribe(symbols.to_vec()).await;
  }

  async fn build_params(
    &self,
    symbols: Vec<String>,
  ) -> HashMap<Cursor, SubscribeRequestInner> {
    let mut params_dict: HashMap<Cursor, SubscribeRequestInner> =
      HashMap::new();
    for symbol in symbols {
      let cursor = {
        let mut symbol_index = self.symbol_index.lock().await;
        symbol_index.remove(&symbol)
      };
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
    &self,
    symbols: Vec<String>,
  ) -> ObserverResult<()> {
    let requests: HashMap<Cursor, SubscribeRequest> = self
      .build_params(symbols)
      .await
      .into_iter()
      .map(|(cur, payload)| (cur, payload.into_unsubscribe()))
      .collect();
    for (cursor, req) in requests {
      let _ = self.cmd_tx.send(SocketCommand::Send(req, cursor));
    }
    return Ok(());
  }

  pub async fn unsubscribe(&self, symbols: Vec<String>) -> ObserverResult<()> {
    return self.handle_unsubscribe(symbols).await;
  }

  async fn socket_event_loop(
    current: Arc<RwLock<Cursor>>,
    pubsub: BookTickerPubSub,
    mut sig: watch::Receiver<bool>,
    mut command: mpsc::UnboundedReceiver<SocketCommand>,
  ) -> ObserverResult<()> {
    let mut sockets: StreamMap<usize, BookTickerSocket> = StreamMap::new();
    loop {
      select! {
        _ = sig.changed() => {
          break;
        },
        Some(cmd) = command.recv() => {
          match cmd {
            SocketCommand::Send(payload, cur) => {
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
            SocketCommand::Regist(socket_id_tx) => {
              {
                let mut cur = current.write().await;
                match Self::get_or_new_socket(&mut sockets, &mut cur).await {
                  Err(e) => {
                    error!(error = as_error!(e); "Failed to register a socket");
                  },
                  Ok(socket_id) => {
                    info!(id = socket_id; "Socket registered");
                    if let Err(_) = socket_id_tx.send(cur.clone()) {
                      error!("Failed to send socket id");
                    };
                  }
                }
              }
            },
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
              info!(id = ticker.id; "Received bookticker info");
              let ticker = match BookTicker::<Float>::try_from(ticker) {
                Ok(ticker) => ticker,
                Err(e) => {
                  error!(error = as_error!(e); "Failed to parse book ticker");
                  continue;
                }
              };
              info!(ticker = as_serde!(ticker); "Publishing bookticker info...");
              if let Err(e) = pubsub.publish(&ticker).await {
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

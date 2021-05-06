pub mod entities;
mod pubsub;

use ::std::collections::{HashMap, HashSet};
use ::std::convert::TryFrom;
use ::std::io::{Error as IOErr, ErrorKind as IOErrKind, Result as IOResult};
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::join_all;
use ::futures::sink::SinkExt;
use ::futures::stream::{BoxStream, StreamExt};
use ::futures::{Future, FutureExt};
use ::mongodb::bson::doc;
use ::mongodb::Database;
use ::nats::Connection as Broker;
use ::serde_json::{from_slice as from_json, to_vec as to_json};
use ::slog::Logger;
use ::subscribe::PubSub;
use ::tokio::select;
use ::tokio::time::{interval, sleep};
use ::tokio_stream::StreamMap;
use ::tokio_tungstenite::{connect_async, tungstenite as wsocket};

use ::binance_symbols::entities::{ListSymbolStream, Symbol, SymbolEvent};
use ::binance_symbols::pubsub::SymbolEventPubSub;
use ::binance_symbols::recorder::SymbolRecorder;
use ::config::DEFAULT_RECONNECT_INTERVAL;
use ::types::{GenericResult, TLSWebSocket, ThreadSafeResult};

use ::binance_clients::constants::WS_ENDPOINT;

use self::entities::{BookTicker, SubscribeRequest, SubscribeRequestInner};
use self::pubsub::BookTickerPubSub;

use ::entities::BookTicker as CommonBookTicker;
use ::errors::{InitError, MaximumAttemptExceeded, WebsocketError};
use ::symbol_recorder::SymbolRecorder as SymbolRecorderTrait;
pub use ::trade_observer::TradeObserver as TradeObserverTrait;

const NUM_SOCKET: usize = 10;
const EVENT_DELAY: Duration = Duration::from_secs(1);

#[derive(Clone)]
pub struct TradeObserver {
  logger: Logger,
  recorder: Option<SymbolRecorder>,
  symbol_event: SymbolEventPubSub,
  bookticker_pubsub: BookTickerPubSub,
  symbols: HashMap<String, (usize, usize)>,
}

impl TradeObserver {
  pub async fn new(
    db: Option<Database>,
    broker: Broker,
    logger: Logger,
  ) -> Self {
    let recorder = match db {
      None => None,
      Some(db) => Some(SymbolRecorder::new(db).await),
    };
    let me = Self {
      symbol_event: SymbolEventPubSub::new(broker.clone()),
      bookticker_pubsub: BookTickerPubSub::new(broker.clone()),
      logger,
      recorder,
      symbols: HashMap::new(),
    };
    return me;
  }

  async fn init_socket(&self) -> Result<TLSWebSocket, WebsocketError> {
    let (websocket, resp) = connect_async(WS_ENDPOINT)
      .then(|res| async {
        res.map_err(|err| WebsocketError {
          msg: Some(err.to_string()),
          status: None,
        })
      })
      .await?;
    let status = resp.status();
    if !status.is_informational() {
      return Err(WebsocketError {
        status: Some(status.as_u16()),
        msg: Some(status.as_str().to_string()),
      });
    }
    return Ok(websocket);
  }

  async fn connect(&mut self) -> Result<TLSWebSocket, MaximumAttemptExceeded> {
    let mut interval =
      interval(Duration::from_secs(DEFAULT_RECONNECT_INTERVAL as u64));
    for _ in 0..20 {
      let socket = match self.init_socket().await {
        Err(e) => {
          ::slog::error!(
            self.logger,
            "Failed to subscribe trade stream"; e,
          );
          interval.tick().await;
          continue;
        }
        Ok(v) => v,
      };
      return Ok(socket);
    }
    return Err(MaximumAttemptExceeded {});
  }

  async fn subscribe<T>(
    &mut self,
    socket: &mut TLSWebSocket,
    symbols: T,
  ) -> ThreadSafeResult<()>
  where
    T: Iterator<Item = String>,
  {
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();
    for symbol in symbols {
      let index = self.add_symbol(&symbol);
      let symbol = symbol.clone();
      match map.get_mut(&index) {
        None => {
          map.insert(index, vec![symbol]);
        }
        Some(a) => {
          a.push(symbol);
        }
      }
    }
    let mut timer = interval(Duration::from_secs(1));
    for (id, symbols) in map.into_iter() {
      let params: Vec<String> = symbols
        .into_iter()
        .map(|item| format!("{}@bookTicker", item.to_lowercase()))
        .collect();
      let id = id as u64;
      let req = SubscribeRequestInner { id, params };
      let req = SubscribeRequest::Subscribe(req);
      ::slog::debug!(self.logger, "Subscribe: {:?}", &req);
      let req = to_json(&req)?;
      let req = String::from_utf8(req)?;
      let _ = socket.send(wsocket::Message::Text(req)).await;
      let _ = socket.flush().await;
      let _ = timer.tick().await;
    }
    return Ok(());
  }

  async fn unsubscribe<T>(
    &mut self,
    socket: &mut TLSWebSocket,
    symbols: T,
  ) -> ThreadSafeResult<()>
  where
    T: Iterator<Item = String>,
  {
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();
    for symbol in symbols {
      match self.get_symbol_index(&symbol) {
        None => {
          continue;
        }
        Some((id, col_index)) => {
          let symbol = symbol.clone();
          match map.get_mut(&id) {
            None => {
              map.insert(id, vec![symbol]);
            }
            Some(v) => {
              v.push(symbol);
            }
          }
          if let Some(a) = self.symbols.get_mut(id) {
            a.remove(col_index);
          }
        }
      };
    }
    let mut timer = interval(Duration::from_secs(1));
    for (id, symbols) in map.into_iter() {
      let params: Vec<String> = symbols
        .into_iter()
        .map(|item| format!("{}@bookTicker", item.to_lowercase()))
        .collect();
      let req = SubscribeRequestInner {
        id: id as u64,
        params,
      };
      let req = SubscribeRequest::Unsubscribe(req);
      ::slog::debug!(self.logger, "Unsubscribe: {:?}", &req);
      let req = to_json(&req)?;
      let req = String::from_utf8(req)?;
      let _ = socket.send(wsocket::Message::Text(req)).await;
      let _ = socket.flush().await;
      let _ = timer.tick().await;
    }
    return Ok(());
  }

  async fn handle_trade(&self, data: &[u8]) {
    let book: BookTicker<f64> = match from_json::<BookTicker<String>>(data) {
      Err(e) => {
        ::slog::warn!(
          self.logger,
          "Failed to decode the payload: {}. Ignoring",
          e
        );
        let data = String::from_utf8(Vec::from(data))
          .unwrap_or(String::from("[FAILED TO DECODE]"));
        ::slog::debug!(self.logger, "Data: {}", data);
        return;
      }
      Ok(v) => match BookTicker::<f64>::try_from(v) {
        Err(e) => {
          ::slog::warn!(
            self.logger,
            "Failed to cast the trade data: {}. Ignoring",
            e
          );
          return;
        }
        Ok(v) => v,
      },
    };
    let _ = self.bookticker_pubsub.publish(&book);
  }

  async fn handle_websocket_message(
    &mut self,
    socket: &mut TLSWebSocket,
    msg: &wsocket::Message,
  ) -> IOResult<()> {
    match msg {
      wsocket::Message::Ping(txt) => {
        let _ = socket.send(wsocket::Message::Pong(txt.to_owned())).await;
      }
      wsocket::Message::Binary(msg) => {
        self.handle_trade(&msg[..]).await;
      }
      wsocket::Message::Text(msg) => {
        let msg = msg.to_owned().into_bytes();
        self.handle_trade(&msg[..]).await;
      }
      wsocket::Message::Close(close_opt) => {
        if let Some(close) = close_opt {
          ::slog::warn!(
            self.logger,
            "Closing connection for a reason.";
            "code" => format!("{}", close.code),
            "reason" => format!("{}", close.reason),
          );
        } else {
          ::slog::warn!(self.logger, "Closing connection...");
        }
        return Err(IOErr::new(
          IOErrKind::ConnectionAborted,
          "Unexpected Closed",
        ));
      }
      wsocket::Message::Pong(_) => {
        ::slog::info!(
          self.logger,
          "Got Pong frame somehow... why?? Anyway, Ingoring."
        );
      }
    }
    return Ok(());
  }

  fn handle_add_symbol(&self, symbol: &Symbol) -> Option<String> {
    if symbol.status != "TRADING"
      || self.get_symbol_index(&symbol.symbol).is_some()
    {
      return None;
    }
    return Some(symbol.symbol.clone());
  }

  async fn handle_event(
    &mut self,
    sockets: &mut StreamMap<usize, &mut TLSWebSocket>,
  ) -> ThreadSafeResult<()> {
    let (mut add_buf, mut del_buf) = (HashSet::new(), HashSet::new());
    let me = self.clone();
    let (handler, symbol_event) =
      me.symbol_event.queue_subscribe("trade_observer")?;
    let mut clear_sym_map_flag = false;
    let mut initial_symbols_stream = self.init().await?;
    loop {
      let event_delay = sleep(EVENT_DELAY);
      select! {
        Some(symbol) = initial_symbols_stream.next() => {
          add_buf.insert(symbol.symbol);
        }
        Some((event, _)) = symbol_event.next() => {
          match event {
            SymbolEvent::Add(symbol) => {
              if let Some(symb) = self.handle_add_symbol(&symbol) {
                add_buf.insert(symb);
              }
            },
            SymbolEvent::Remove(symbol) => {
              del_buf.insert(symbol.symbol);
            }
          }
        },
        _ = event_delay => {
          if clear_sym_map_flag {
            let symbols = self.symbols.to_owned().into_iter().flatten();
            if let Err(e) = self.unsubscribe(socket, symbols).await {
              ::slog::warn!(
                self.logger,
                "Got an error while unsubscribing the symbol (init): {}",
                e
              );
            } else {
              self.symbols.clear();
            }
            clear_sym_map_flag = false;
          }
          if let Err(e) = self.subscribe(socket, add_buf.drain()).await {
            ::slog::warn!(
              self.logger,
              "Got an error while subscribing the symbol: {}",
              e
            );
          }
          if let Err(e) = self.unsubscribe(socket, del_buf.drain()).await {
            ::slog::warn!(
              self.logger,
              "Got an error while unsubscribing the symbol: {}",
              e
            );
          }
        },
        Some((index, Ok(msg))) = sockets.next() => {
          let _ =  self.handle_websocket_message(socket, &msg).await?;
        }
        else => {break;}
      }
    }
    return Ok(());
  }

  async fn init(&self) -> ThreadSafeResult<ListSymbolStream<'static>> {
    let recorder = self
      .recorder
      .clone()
      .ok_or(InitError::new(Some("binance.observer")))?;
    return recorder.list(doc! {"status": "TRADING"}).await;
  }
}

#[async_trait]
impl TradeObserverTrait for TradeObserver {
  async fn start(&self) -> GenericResult<()> {
    let mut me = self.clone();
    let mut sockets = vec![];
    for _ in 0..NUM_SOCKET {
      sockets.push(me.connect());
    }
    let sockets = join_all(sockets).await;
    let mut socket_map = StreamMap::new();
    for (index, socket) in sockets.into_iter().enumerate() {
      socket_map.insert(index, &mut socket?);
    }
    if let Err(e) = me.handle_event(&mut socket_map).await {
      ::slog::error!(
        me.logger,
        "Failed to open the handler of the trade event: {}",
        e,
      );
    };
    return Ok(());
  }
  async fn subscribe(
    &self,
  ) -> ::std::io::Result<BoxStream<'_, CommonBookTicker>> {
    let (_, st) = self.bookticker_pubsub.subscribe()?;
    let st = st.map(|(item, _)| {
      let ret: CommonBookTicker = item.into();
      return ret;
    });
    return Ok(st.boxed());
  }
}

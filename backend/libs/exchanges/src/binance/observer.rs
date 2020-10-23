use ::std::collections::HashMap;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::join;
use ::futures::sink::SinkExt;
use ::futures::stream::StreamExt;
use ::nats::asynk::{Connection as Broker, Subscription as NatsSub};
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde_json::{from_slice as from_json, to_vec as to_json};
use ::slog::Logger;
use ::tokio::net::TcpStream;
use ::tokio::select;
use ::tokio::time::{delay_for, interval};
use ::tokio_native_tls::TlsStream;
use ::tokio_tungstenite::{
  connect_async, stream::Stream, tungstenite as wsocket, WebSocketStream,
};

use ::config::DEFAULT_RECONNECT_INTERVAL;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::{
  SYMBOL_ADD_EVENT, SYMBOL_REMOVE_EVENT, TRADE_OBSERVER_SUB_NAME, WS_ENDPOINT,
};
use super::entities::{
  StreamEvent, Symbol, Trade, TradeSubRequest, TradeSubRequestInner,
};

use crate::errors::{MaximumAttemptExceeded, WebsocketError};
use crate::traits::TradeObserver as TradeObserverTrait;

type TLSWebSocket = WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>;

const NUM_SESSION: usize = 10;
const EVENT_DELAY: Duration = Duration::from_secs(1);

#[derive(Clone)]
pub struct TradeObserver {
  broker: Broker,
  logger: Logger,
  symbols: Vec<Vec<String>>,
}

impl TradeObserver {
  pub fn new(broker: Broker, logger: Logger) -> Self {
    let me = Self {
      broker,
      logger,
      symbols: vec![],
    };
    return me;
  }

  fn add_symbol(&mut self, symbol: &String) -> usize {
    if self.symbols.len() < NUM_SESSION {
      self.symbols.push(vec![symbol.clone()]);
      return self.symbols.len() - 1;
    }
    let (index, most_bored) = self
      .symbols
      .iter_mut()
      .enumerate()
      .min_by_key(|(_, x)| x.len())
      .unwrap();
    most_bored.push(symbol.clone());
    return index;
  }

  fn get_symbol_index(&self, symbol: &String) -> Option<(usize, usize)> {
    for (index, symbols) in self.symbols.iter().enumerate() {
      match symbols.iter().position(|s| s == symbol) {
        None => {
          continue;
        }
        Some(pos) => return Some((index, pos)),
      }
    }
    return None;
  }

  async fn init_socket(&self) -> SendableErrorResult<TLSWebSocket> {
    let (websocket, resp) = ret_on_err!(connect_async(WS_ENDPOINT).await);
    let status = resp.status();
    if !status.is_informational() {
      return Err(Box::new(WebsocketError { status }));
    }
    return Ok(websocket);
  }

  async fn connect(
    &mut self,
    initial_symbols: Option<Vec<String>>,
  ) -> SendableErrorResult<TLSWebSocket> {
    let mut interval =
      interval(Duration::from_secs(DEFAULT_RECONNECT_INTERVAL as u64));
    for _ in 0..20 {
      let mut socket = match self.init_socket().await {
        Err(e) => {
          ::slog::error!(
            self.logger,
            "Failed to subscribe trade stream: {}",
            e
          );
          interval.tick().await;
          continue;
        }
        Ok(v) => v,
      };
      let initial_symbols = match initial_symbols {
        None => self.symbols.clone().into_iter().flatten().collect(),
        Some(initial_symbols) => initial_symbols,
      };
      self.symbols.clear();
      let _ = self.subscribe(&mut socket, &initial_symbols).await;
      return Ok(socket);
    }
    return Err(Box::new(MaximumAttemptExceeded {}));
  }

  async fn subscribe(
    &mut self,
    socket: &mut TLSWebSocket,
    symbols: &Vec<String>,
  ) -> SendableErrorResult<()> {
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();
    for symbol in symbols {
      let index = self.add_symbol(symbol);
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
        .map(|item| format!("{}@trade", item.to_lowercase()))
        .collect();
      let id = id as u64;
      let req = TradeSubRequestInner { id, params };
      let req = TradeSubRequest::Subscribe(req);
      ::slog::debug!(self.logger, "Subscribe: {:?}", &req);
      let req = ret_on_err!(to_json(&req));
      let req = ret_on_err!(String::from_utf8(req));
      let _ = socket.send(wsocket::Message::Text(req)).await;
      let _ = socket.flush().await;
      let _ = timer.tick().await;
    }
    return Ok(());
  }

  async fn unsubscribe(
    &mut self,
    socket: &mut TLSWebSocket,
    symbols: &Vec<String>,
  ) -> SendableErrorResult<()> {
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
        .map(|item| format!("{}@trade", item.to_lowercase()))
        .collect();
      let req = TradeSubRequestInner {
        id: id as u64,
        params,
      };
      let req = TradeSubRequest::Unsubscribe(req);
      ::slog::debug!(self.logger, "Unsubscribe: {:?}", &req);
      let req = ret_on_err!(to_json(&req));
      let req = ret_on_err!(String::from_utf8(req));
      let _ = socket.send(wsocket::Message::Text(req)).await;
      let _ = socket.flush().await;
      let _ = timer.tick().await;
    }
    return Ok(());
  }

  async fn handle_trade(&self, data: &[u8]) {
    let event: StreamEvent = match from_json(data) {
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
      Ok(v) => v,
    };
    match event {
      StreamEvent::Trade(trade) => {
        let trade: SendableErrorResult<Trade> = trade.into();
        let trade: Trade = match trade {
          Err(e) => {
            ::slog::warn!(
              self.logger,
              "Failed to cast trade data: {}. Ignoring",
              e
            );
            return;
          }
          Ok(v) => v,
        };
        ::slog::debug!(self.logger, "Trade: {:?}", trade);
        let msg = match to_msgpack(&trade) {
          Err(e) => {
            ::slog::warn!(
              self.logger,
              "Failed to encode the trade data: {}",
              e
            );
            return;
          }
          Ok(v) => v,
        };
        let _ = self.broker.publish(TRADE_OBSERVER_SUB_NAME, &msg[..]).await;
      } // _ => {}
    }
  }

  async fn handle_websocket_message(
    &mut self,
    socket: &mut TLSWebSocket,
    msg: &wsocket::Message,
  ) {
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
        ::slog::info!(self.logger, "Reconnecting...");
        *socket = match self.connect(None).await {
          Err(e) => {
            ::slog::error!(self.logger, "Failed to connect: {}", e);
            return;
          }
          Ok(s) => s,
        };
      }
      wsocket::Message::Pong(_) => {
        ::slog::info!(
          self.logger,
          "Got Pong frame somehow... why?? Anyway, Ingoring."
        );
      }
    }
  }

  async fn handle_event(
    &mut self,
    socket: &mut TLSWebSocket,
  ) -> SendableErrorResult<()> {
    let (mut add_buf, mut del_buf) = (vec![], vec![]);
    let (symbol_add_event, symbol_remove_evnet) = join!(
      self
        .broker
        .queue_subscribe(SYMBOL_ADD_EVENT, "trade_observer"),
      self.broker.subscribe(SYMBOL_REMOVE_EVENT)
    );
    let (mut symbol_add_event, mut symbol_remove_evnet) = (
      ret_on_err!(symbol_add_event),
      ret_on_err!(symbol_remove_evnet),
    );
    loop {
      let event_delay = delay_for(EVENT_DELAY);
      select! {
        Some(msg) = symbol_add_event.next() => {
          let symbol: Symbol = match from_msgpack(&msg.data[..]) {
            Err(e) => {
              ::slog::warn!(
                self.logger,
                "Failed to read symbol add event payload: {}",
                e
              );
              continue;
            },
            Ok(o) => o
          };
          if symbol.status != "TRADING" {
            continue;
          }
          add_buf.push(symbol.symbol);
        },
        Some(msg) = symbol_remove_evnet.next() => {
          let symbol: Symbol = match from_msgpack(&msg.data[..]) {
            Err(e) => {
              ::slog::warn!(
                self.logger,
                "Failed to read symbol removal event payload: {}",
                e
              );
              continue;
            },
            Ok(o) => o
          };
          del_buf.push(symbol.symbol);
        },
        _ = event_delay => {
          if add_buf.is_empty() {
            continue;
          }
          if let Err(e) = self.subscribe(socket, &add_buf).await {
            ::slog::warn!(
              self.logger,
              "Got an error while subscribing the symbol: {}",
              e
            );
          }
          add_buf.clear();
          if del_buf.is_empty() {
            continue;
          }
          if let Err(e) = self.unsubscribe(socket, &del_buf).await {
            ::slog::warn!(
              self.logger,
              "Got an error while unsubscribing the symbol: {}",
              e
            );
          }
          del_buf.clear();
        },
        Some(Ok(msg)) = socket.next() => {
          self.handle_websocket_message(socket, &msg).await;
        }
        else => {break;}
      }
    }
    return Ok(());
  }
}

#[async_trait]
impl TradeObserverTrait for TradeObserver {
  async fn start(
    &self,
    initial_symbols: Option<Vec<String>>,
  ) -> SendableErrorResult<()> {
    let mut me = self.clone();
    let mut socket = me.connect(initial_symbols).await?;
    if let Err(e) = me.handle_event(&mut socket).await {
      ::slog::error!(
        me.logger,
        "Failed to open the handler of the trade event: {}",
        e,
      );
    };
    return Ok(());
  }
  async fn subscribe(&self) -> ::std::io::Result<NatsSub> {
    return self.broker.subscribe(TRADE_OBSERVER_SUB_NAME).await;
  }
}

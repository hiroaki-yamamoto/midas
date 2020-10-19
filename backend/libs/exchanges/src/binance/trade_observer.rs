use ::async_trait::async_trait;
use ::futures::sink::SinkExt;
use ::futures::stream::StreamExt;
use ::nats::asynk::{Connection as Broker, Subscription as NatsSub};
use ::rand::random;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde_json::{from_slice as from_json, to_vec as to_json};
use ::slog::Logger;
use ::std::time::Duration;
use ::tokio::net::TcpStream;
use ::tokio::select;
use ::tokio::time::interval;
use ::tokio_native_tls::TlsStream;
use ::tokio_tungstenite::{
  connect_async, stream::Stream, tungstenite as wsocket, WebSocketStream,
};

use ::config::DEFAULT_RECONNECT_INTERVAL;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::{
  SYMBOL_UPDATE_EVENT, TRADE_OBSERVER_SUB_NAME, WS_ENDPOINT,
};
use super::entities::{
  StreamEvent, SymbolUpdateEvent, Trade, TradeSubRequest, TradeSubRequestInner,
};

use crate::errors::{MaximumAttemptExceeded, WebsocketError};
use crate::traits::TradeObserver as TradeObserverTrait;

type TLSWebSocket = WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>;

#[derive(Clone)]
pub struct TradeObserver {
  id: u32,
  broker: Broker,
  logger: Logger,
  symbols: Vec<String>,
}

impl TradeObserver {
  pub fn new(
    broker: Broker,
    logger: Logger,
    initial_symbols: Vec<String>,
  ) -> Self {
    return Self {
      id: random(),
      broker,
      logger,
      symbols: initial_symbols,
    };
  }

  async fn init_socket(&self) -> SendableErrorResult<TLSWebSocket> {
    let (websocket, resp) = ret_on_err!(connect_async(WS_ENDPOINT).await);
    let status = resp.status();
    if !status.is_informational() {
      return Err(Box::new(WebsocketError { status }));
    }
    return Ok(websocket);
  }

  async fn init_subscription(
    &self,
    socket: &mut TLSWebSocket,
  ) -> SendableErrorResult<()> {
    if self.symbols.is_empty() {
      return Ok(());
    }
    let mut inner = TradeSubRequestInner {
      id: self.id,
      params: vec![],
    };
    for symbol in &self.symbols {
      inner
        .params
        .push(format!("{}@trade", symbol.to_lowercase()));
    }
    let req = TradeSubRequest::Subscribe(inner);
    let payload = ret_on_err!(to_json(&req));
    let payload = ret_on_err!(String::from_utf8(payload));
    let req = wsocket::Message::Text(payload);
    let _ = socket.send(req).await;
    let _ = socket.flush().await;
    return Ok(());
  }

  async fn connect(&self) -> SendableErrorResult<TLSWebSocket> {
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
      let _ = self.init_subscription(&mut socket).await;
      return Ok(socket);
    }
    return Err(Box::new(MaximumAttemptExceeded {}));
  }

  async fn update_symbols(
    &mut self,
    socket: &mut TLSWebSocket,
    update_event: SymbolUpdateEvent,
  ) -> SendableErrorResult<()> {
    let mut sub_req_inner = TradeSubRequestInner {
      id: self.id,
      params: vec![],
    };
    let mut unsub_req_inner = sub_req_inner.clone();
    for to_sub in &update_event.to_add {
      sub_req_inner
        .params
        .push(format!("{}@trade", to_sub.to_lowercase()));
    }
    self.symbols.extend(update_event.to_add);
    for to_unsub in &update_event.to_remove {
      let sub_name = format!("{}@trade", to_unsub.to_lowercase());
      unsub_req_inner.params.push(sub_name.clone());
      let sym_position =
        self.symbols.iter().position(move |name| name == &sub_name);
      if let Some(pos) = sym_position {
        self.symbols.remove(pos);
      }
    }
    let sub_req = TradeSubRequest::Subscribe(sub_req_inner);
    let unsub_req = TradeSubRequest::Unsubscribe(unsub_req_inner);
    let sub_payload = ret_on_err!(to_json(&sub_req));
    let unsub_payload = ret_on_err!(to_json(&unsub_req));
    let _ = socket.send(wsocket::Message::Binary(unsub_payload)).await;
    let _ = socket.send(wsocket::Message::Binary(sub_payload)).await;
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
    &self,
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
        *socket = match self.connect().await {
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
    let mut symbol_update =
      ret_on_err!(self.broker.subscribe(SYMBOL_UPDATE_EVENT).await);
    loop {
      select! {
        Some(msg) = symbol_update.next() => {
          let event: SymbolUpdateEvent = match from_msgpack(&msg.data[..]) {
            Err(e) => {
              ::slog::warn!(self.logger, "Failed to read update event payload: {}", e);
              continue;
            },
            Ok(o) => o
          };
          if let Err(e) = self.update_symbols(socket, event).await {
            ::slog::warn!(self.logger, "Got an error while updating symbol: {}", e);
          }
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
  async fn start(&self) -> SendableErrorResult<()> {
    let mut me = self.clone();
    let mut socket = me.connect().await?;
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

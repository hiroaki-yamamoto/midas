use ::async_trait::async_trait;
use ::futures::sink::SinkExt;
use ::futures::stream::StreamExt;
use ::nats::asynk::{Connection as Broker, Subscription as NatsSub};
use ::rand::random;
use ::rmp_serde::from_slice as from_msgpack;
use ::serde_json::to_vec as to_json;
use ::slog::Logger;
use ::tokio::net::TcpStream;
use ::tokio::select;
use ::tokio_tungstenite::{
  connect_async, tungstenite as wsocket, WebSocketStream,
};
use ::tonic::transport::Channel;
use ::tonic::Request;

use ::rpc::entities::Exchanges;
use ::rpc::symbol::symbol_client::SymbolClient;
use ::rpc::symbol::QueryRequest;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::{
  SYMBOL_UPDATE_EVENT, TRADE_OBSERVER_SUB_NAME, WS_ENDPOINT,
};
use super::entities::{
  SymbolUpdateEvent, TradeSubRequest, TradeSubRequestInner,
};

use crate::errors::WebsocketError;
use crate::traits::TradeObserver as TradeObserverTrait;

#[derive(Clone)]
pub struct TradeObserver {
  id: u32,
  broker: Broker,
  logger: Logger,
  symbols: Vec<String>,
}

impl TradeObserver {
  pub async fn new(
    broker: Broker,
    logger: Logger,
    symbol_client: &mut SymbolClient<Channel>,
  ) -> SendableErrorResult<Self> {
    let symbols = ret_on_err!(
      symbol_client
        .query(Request::new(QueryRequest {
          exchange: Exchanges::Binance as i32,
          status: String::from("TRADING"),
          symbols: vec![],
        }))
        .await
    )
    .into_inner()
    .symbols
    .into_iter()
    .map(|item| item.symbol)
    .collect();

    return Ok(Self {
      id: random(),
      broker,
      logger,
      symbols,
    });
  }

  async fn init_socket(
    &self,
  ) -> SendableErrorResult<WebSocketStream<TcpStream>> {
    let (websocket, resp) = ret_on_err!(connect_async(WS_ENDPOINT).await);
    let status = resp.status();
    if !status.is_informational() {
      return Err(Box::new(WebsocketError { status }));
    }
    return Ok(websocket);
  }

  async fn init_subscription(
    &self,
    socket: &mut WebSocketStream<TcpStream>,
  ) -> SendableErrorResult<()> {
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

  async fn update_symbols(
    &mut self,
    socket: &mut WebSocketStream<TcpStream>,
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

  async fn handle_websocket_message(
    &self,
    socket: &mut WebSocketStream<TcpStream>,
    msg: &wsocket::Message,
  ) {
    // match msg {
    //   wsocket::Message::Ping(txt) => {
    //     socket.send(wsocket::Message::Pong(txt.to_owned()));
    //   }
    //   wsocket::Message::Binary(msg) => {}
    // }
  }

  async fn handle_event(
    &mut self,
    socket: &mut WebSocketStream<TcpStream>,
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
  async fn start(&self) -> SendableErrorResult<NatsSub> {
    let mut me = self.clone();
    let mut socket = me.init_socket().await?;
    me.init_subscription(&mut socket).await?;
    ::tokio::spawn(async move {
      if let Err(e) = me.handle_event(&mut socket).await {
        ::slog::error!(
          me.logger,
          "Failed to open the handler of the trade event: {}",
          e,
        );
      };
    });
    let sub = ret_on_err!(self.broker.subscribe(TRADE_OBSERVER_SUB_NAME).await);
    return Ok(sub);
  }
  async fn stop(&self) {}
}

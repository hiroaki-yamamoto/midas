use ::async_trait::async_trait;
use ::nats::asynk::{Connection as Broker, Subscription as NatsSub};
use ::tokio::net::TcpStream;
use ::tokio_tungstenite::{connect_async, WebSocketStream};

use ::types::{ret_on_err, SendableErrorResult};

use super::constants::{TRADE_OBSERVER_SUB_NAME, WS_ENDPOINT};

use crate::errors::WebsocketError;
use crate::traits::TradeObserver as TradeObserverTrait;

pub struct TradeObserver {
  websocket: WebSocketStream<TcpStream>,
  broker: Broker,
}

impl TradeObserver {
  pub async fn new(broker: Broker) -> SendableErrorResult<Self> {
    let (websocket, resp) = ret_on_err!(connect_async(WS_ENDPOINT).await);
    let status = resp.status();
    if !status.is_informational() {
      return Err(Box::new(WebsocketError { status }));
    }
    return Ok(Self { broker, websocket });
  }
}

#[async_trait]
impl TradeObserverTrait for TradeObserver {
  async fn start(&self) -> SendableErrorResult<NatsSub> {
    let sub = ret_on_err!(self.broker.subscribe(TRADE_OBSERVER_SUB_NAME).await);
    return Ok(sub);
  }
  async fn stop(&self) {}
}

use ::std::marker::PhantomData;
use ::std::pin::Pin;
use ::std::task::{Context, Poll};
use ::std::time::Duration;

use ::futures::stream::{Stream, StreamExt};
use ::log::{as_error, error};
use ::rand::thread_rng;
use ::rand::Rng;
use ::serde::{de::DeserializeOwned, ser::Serialize};
use ::tokio::runtime::Runtime;
use ::tokio::time::interval;
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use ::tokio_tungstenite::tungstenite::protocol::CloseFrame;
use ::tokio_tungstenite::tungstenite::{Message, Result as WSResult};

use ::errors::{
  MaximumAttemptExceeded, WebSocketInitResult, WebsocketHandleResult,
};
use ::types::TLSWebSocket;

/// WebSocket Client.
/// R = Read Entity
/// W = Write Entity
pub struct WebSocket<R, W>
where
  R: DeserializeOwned,
  W: Serialize,
{
  socket: TLSWebSocket,
  endpoints: Vec<String>,
  runtime: Runtime,
  _r: PhantomData<R>,
  _w: PhantomData<W>,
}

impl<R, W> WebSocket<R, W>
where
  R: DeserializeOwned,
  W: Serialize,
{
  pub async fn connect(
    endpoints: &[String],
  ) -> WebSocketInitResult<TLSWebSocket> {
    let mut interval = interval(Duration::from_secs(1));
    let url_index = {
      let mut rng = thread_rng();
      rng.gen_range(0..endpoints.len())
    };
    for jitter in 0..endpoints.len() {
      let url = endpoints[(url_index + jitter) % endpoints.len()].to_string();
      let (socket, resp) = connect_async(&url).await?;
      if resp.status().is_client_error() || resp.status().is_server_error() {
        let message = resp.body();
        let message = message
          .as_ref()
          .map(|b| String::from_utf8_lossy(b.as_slice()));
        error!(
          code = resp.status().as_u16(),
          message = message;
          "Failed to establish websocket connection. Re-Connecting..."
        );
      } else {
        return Ok(socket);
      }
      interval.tick().await;
    }
    return Err(MaximumAttemptExceeded.into());
  }

  pub async fn new(endpoints: &[String]) -> WebSocketInitResult<Self> {
    let socket = Self::connect(endpoints).await?;
    return Ok(Self {
      socket,
      endpoints: endpoints.into_iter().map(|s| s.to_string()).collect(),
      runtime: Runtime::new()?,
      _r: PhantomData,
      _w: PhantomData,
    });
  }

  pub fn handle_server_payload(
    &mut self,
    payload: WSResult<Option<Message>>,
  ) -> WebsocketHandleResult<Poll<R>> {
    match payload {
      Err(e) => {
        error!(
          error = as_error!(e);
          "Error while receiving payload from server. Re-Connecting..."
        );
        self.runtime.block_on(
          self.socket.close(
            CloseFrame {
              code: CloseCode::Abnormal,
              reason: "Received an anomaly disconnection from server."
                .to_string()
                .into(),
            }
            .into(),
          ),
        );
        self.socket = self
          .runtime
          .block_on(Self::connect(self.endpoints.as_slice()))?;
        return Ok(Poll::Pending);
      }
      Ok(None) => {
        error!("Received close payload from server. Re-Connecting...");
        self.runtime.block_on(
          self.socket.close(
            CloseFrame {
              code: CloseCode::Abnormal,
              reason: "Received close payload from server.".to_string().into(),
            }
            .into(),
          ),
        );
        self.socket = self
          .runtime
          .block_on(Self::connect(self.endpoints.as_slice()))?;
        return Ok(Poll::Pending);
      }
      Ok(Some(msg)) => {}
    }
  }
}

impl<R, W> Stream for WebSocket<R, W>
where
  R: DeserializeOwned,
  W: Serialize,
{
  type Item = R;
  fn poll_next(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    match self.socket.poll_next_unpin(cx) {
      Poll::Ready(payload) => match self.handle_server_payload(payload) {
        Err(e) => {
          error!(
            error = as_error!(e);
            "Un-recoverable Error while handling server payload."
          );
          return Poll::Ready(None);
        }
        Ok(_) => {}
      },
      Poll::Pending => {
        return Poll::Pending;
      }
    }
  }
}

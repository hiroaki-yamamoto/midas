use ::std::marker::PhantomData;
use ::std::pin::Pin;
use ::std::sync::Arc;
use ::std::task::{Context, Poll};
use ::std::time::Duration;

use ::futures::sink::Sink;
use ::futures::sink::SinkExt;
use ::futures::stream::{Stream, StreamExt};
use ::log::{as_display, as_error, error, info, warn};
use ::rand::thread_rng;
use ::rand::Rng;
use ::serde::{de::DeserializeOwned, ser::Serialize};
use ::serde_json::{from_str as json_parse, to_string as jsonify};
use ::tokio::runtime::Runtime;
use ::tokio::sync::RwLock;
use ::tokio::time::interval;
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use ::tokio_tungstenite::tungstenite::protocol::CloseFrame;
use ::tokio_tungstenite::tungstenite::{Message, Result as WSResult};

use ::errors::{
  MaximumAttemptExceeded, WebSocketInitResult, WebsocketHandleResult,
  WebsocketMessageResult, WebsocketSinkError,
};
use ::types::TLSWebSocket;

/// WebSocket Client.
/// R = Read Entity
/// W = Write Entity
pub struct WebSocket<R, W>
where
  R: DeserializeOwned + Unpin,
  W: Serialize + Unpin,
{
  socket: Arc<RwLock<TLSWebSocket>>,
  endpoints: Vec<String>,
  runtime: Arc<Runtime>,
  _r: PhantomData<R>,
  _w: PhantomData<W>,
}

impl<R, W> WebSocket<R, W>
where
  R: DeserializeOwned + Unpin,
  W: Serialize + Unpin,
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
      if !resp.status().is_informational() {
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
      socket: Arc::new(RwLock::new(socket)),
      endpoints: endpoints.into_iter().map(|s| s.to_string()).collect(),
      runtime: Arc::new(Runtime::new()?),
      _r: PhantomData,
      _w: PhantomData,
    });
  }

  async fn close(&mut self, code: CloseCode, reason: &str) -> WSResult<()> {
    return self
      .socket
      .write()
      .await
      .close(
        CloseFrame {
          code,
          reason: reason.to_string().into(),
        }
        .into(),
      )
      .await;
  }

  async fn reconnect(&mut self) -> WebSocketInitResult<()> {
    self.socket =
      Arc::new(RwLock::new(Self::connect(self.endpoints.as_slice()).await?));
    return Ok(());
  }

  async fn send_msg(&mut self, msg: Message) -> WSResult<()> {
    let mut socket = self.socket.write().await;
    let send_result = socket.send(msg).await;
    let flush_result = socket.flush().await;
    return send_result.and(flush_result);
  }

  async fn reconnect_on_error(
    &mut self,
    payload: Option<WSResult<Message>>,
  ) -> WebsocketHandleResult<Option<Message>> {
    match payload {
      Some(Err(e)) => {
        error!(
          error = as_error!(e);
          "Error while receiving payload from server. Re-Connecting..."
        );
        let _ = self.close(
          CloseCode::Abnormal,
          "Error while receiving payload from server.",
        );
        self.reconnect().await?;
        return Ok(None);
      }
      None => {
        error!("Received close payload from stream. Re-Connecting...");
        let _ = self
          .close(CloseCode::Abnormal, "Received close payload from stream.");
        self.reconnect().await?;
        return Ok(None);
      }
      Some(Ok(msg)) => return Ok(Some(msg)),
    }
  }

  async fn handle_message(
    &mut self,
    msg: Message,
  ) -> WebsocketMessageResult<Option<R>> {
    match msg {
      Message::Text(text) => {
        let payload: R = json_parse(text.as_str())?;
        return Ok(payload.into());
      }
      Message::Binary(blob) => {
        let payload = String::from_utf8(blob)?;
        let payload: R = json_parse(&payload)?;
        return Ok(payload.into());
      }
      Message::Ping(payload) => {
        let _ = self.send_msg(Message::Pong(payload)).await?;
        return Ok(None);
      }
      Message::Pong(msg) => {
        info!(message = String::from_utf8_lossy(&msg); "Received Pong Message");
        return Ok(None);
      }
      Message::Close(_) => {
        error!("Disconnected. Re-Connecting...");
        let _ = self.reconnect().await?;
        return Ok(None);
      }
      Message::Frame(frame) => {
        warn!(frame = as_display!(frame); "Received Unexpected Frame Message");
        return Ok(None);
      }
    }
  }
}

impl<R, W> Stream for WebSocket<R, W>
where
  R: DeserializeOwned + Unpin,
  W: Serialize + Unpin,
{
  type Item = R;
  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let runtime = self.runtime.clone();
    let payload = {
      let mut socket = runtime.block_on(self.socket.write());
      socket.poll_next_unpin(cx)
    };
    match payload {
      Poll::Ready(payload) => {
        match runtime.block_on(self.reconnect_on_error(payload)) {
          Err(e) => {
            error!(
              error = as_error!(e);
              "Un-recoverable Error while handling server payload."
            );
            return Poll::Ready(None);
          }
          Ok(None) => {
            return Poll::Pending;
          }
          Ok(Some(msg)) => match runtime.block_on(self.handle_message(msg)) {
            Err(e) => {
              error!(error = as_error!(e); "Failed to decoding the payload.");
              return Poll::Pending;
            }
            Ok(None) => {
              return Poll::Pending;
            }
            Ok(Some(payload)) => {
              return Poll::Ready(Some(payload));
            }
          },
        }
      }
      Poll::Pending => {
        return Poll::Pending;
      }
    }
  }
}

impl<R, W> Sink<W> for WebSocket<R, W>
where
  R: DeserializeOwned + Unpin,
  W: Serialize + Unpin,
{
  type Error = WebsocketSinkError;

  fn poll_ready(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    let mut socket = self.runtime.block_on(self.socket.write());
    return socket
      .poll_ready_unpin(cx)
      .map(|res| res.map_err(|err| err.into()));
  }

  fn poll_close(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    let mut socket = self.runtime.block_on(self.socket.write());
    return socket
      .poll_close_unpin(cx)
      .map(|res| res.map_err(|err| err.into()));
  }

  fn poll_flush(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    let mut socket = self.runtime.block_on(self.socket.write());
    return socket
      .poll_flush_unpin(cx)
      .map(|res| res.map_err(|err| err.into()));
  }

  fn start_send(mut self: Pin<&mut Self>, item: W) -> Result<(), Self::Error> {
    let payload = jsonify(&item)?;
    let msg = Message::Text(payload);
    let runtime = self.runtime.clone();
    return runtime
      .block_on(self.send_msg(msg))
      .map_err(|err| err.into());
  }
}

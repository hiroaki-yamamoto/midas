use ::std::marker::PhantomData;
use ::std::pin::Pin;
use ::std::task::{Context, Poll};
use ::std::time::Duration;

use ::futures::future::FutureExt;
use ::futures::sink::Sink;
use ::futures::sink::SinkExt;
use ::futures::stream::{Stream, StreamExt};
use ::log::{as_display, as_error, error, info, warn};
use ::rand::thread_rng;
use ::rand::Rng;
use ::serde::{de::DeserializeOwned, ser::Serialize};
use ::serde_json::{from_str as json_parse, to_string as jsonify};
use ::tokio::runtime::Runtime;
use ::tokio::sync::Mutex;
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
  socket: Mutex<TLSWebSocket>,
  endpoints: Vec<String>,
  runtime: Runtime,
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
      socket: socket.into(),
      endpoints: endpoints.into_iter().map(|s| s.to_string()).collect(),
      runtime: Runtime::new()?,
      _r: PhantomData,
      _w: PhantomData,
    });
  }

  async fn close(&self, code: CloseCode, reason: &str) -> WSResult<()> {
    let mut socket = self.socket.lock().await;
    return socket
      .close(
        CloseFrame {
          code,
          reason: reason.to_string().into(),
        }
        .into(),
      )
      .await;
  }

  async fn reconnect(&self) -> WebSocketInitResult<()> {
    let mut socket = self.socket.lock().await;
    *socket = Self::connect(self.endpoints.as_slice()).await?;
    return Ok(());
  }

  async fn send_msg(&self, msg: Message) -> WSResult<()> {
    let mut socket = self.socket.lock().await;
    let send_result = socket.send(msg).await;
    let flush_result = socket.flush().await;
    return send_result.and(flush_result);
  }

  async fn reconnect_on_error(
    &self,
    payload: Option<WSResult<Message>>,
  ) -> WebsocketHandleResult<Option<Message>> {
    match payload {
      Some(Err(e)) => {
        error!(
          error = as_error!(e);
          "Error while receiving payload from server. Re-Connecting..."
        );
        let _ = self
          .close(
            CloseCode::Abnormal,
            "Error while receiving payload from server.",
          )
          .await;
        self.reconnect().await?;
        return Ok(None);
      }
      None => {
        error!("Received close payload from stream. Re-Connecting...");
        let _ = self
          .close(CloseCode::Abnormal, "Received close payload from stream.")
          .await;
        self.reconnect().await?;
        return Ok(None);
      }
      Some(Ok(msg)) => return Ok(Some(msg)),
    }
  }

  async fn handle_message(
    self,
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
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    match {
      let mut socket = match self.socket.lock().boxed().poll_unpin(cx) {
        Poll::Ready(socket) => socket,
        Poll::Pending => return Poll::Pending,
      };
      socket.poll_next_unpin(cx)
    } {
      Poll::Ready(payload) => {
        if let Poll::Ready(r) = self
          .reconnect_on_error(payload)
          .boxed_local()
          .poll_unpin(cx)
        {
          match r {
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
            Ok(Some(msg)) => {
              if let Poll::Ready(msg) =
                self.handle_message(msg).boxed_local().poll_unpin(cx)
              {
                match msg {
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
                }
              } else {
                return Poll::Pending;
              }
            }
          }
        } else {
          return Poll::Pending;
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
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    let mut socket = match self.socket.lock().boxed().poll_unpin(cx) {
      Poll::Ready(socket) => socket,
      Poll::Pending => return Poll::Pending,
    };
    return socket
      .poll_ready_unpin(cx)
      .map(|res| res.map_err(|err| err.into()));
  }

  fn poll_close(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    let mut socket = match self.socket.lock().boxed().poll_unpin(cx) {
      Poll::Ready(socket) => socket,
      Poll::Pending => return Poll::Pending,
    };
    return socket
      .poll_close_unpin(cx)
      .map(|res| res.map_err(|err| err.into()));
  }

  fn poll_flush(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    let mut socket = match self.socket.lock().boxed().poll_unpin(cx) {
      Poll::Ready(socket) => socket,
      Poll::Pending => return Poll::Pending,
    };
    return socket
      .poll_flush_unpin(cx)
      .map(|res| res.map_err(|err| err.into()));
  }

  fn start_send(self: Pin<&mut Self>, item: W) -> Result<(), Self::Error> {
    let me = self.get_mut();
    let payload = jsonify(&item)?;
    let msg = Message::Text(payload);
    return me
      .send_msg(msg)
      .boxed()
      .poll_unpin(cx)
      .map_err(|err| err.into());
  }
}

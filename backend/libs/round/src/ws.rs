use ::std::pin::Pin;
use ::std::task::{Context, Poll};
use ::std::time::Duration;

use ::futures::executor::block_on;
use ::futures::sink::Sink;
use ::futures::sink::SinkExt;
use ::futures::stream::{Stream, StreamExt};
use ::log::{as_display, as_error, error, info, warn};
use ::rand::thread_rng;
use ::rand::Rng;
use ::serde::{de::DeserializeOwned, ser::Serialize};
use ::serde_json::{from_str as json_parse, to_string as jsonify};
use ::tokio::sync::mpsc;
use ::tokio::time::interval;
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use ::tokio_tungstenite::tungstenite::protocol::CloseFrame;
use ::tokio_tungstenite::tungstenite::{Message, Result as WSResult};
use serde_json::error;

use ::errors::{
  MaximumAttemptExceeded, WebSocketInitResult, WebsocketHandleResult,
  WebsocketMessageResult, WebsocketSinkError,
};
use ::types::TLSWebSocket;

enum Command {
  Terminate,
  Reconnect(CloseCode, String),
  Close(CloseCode, String),
  Send(Message),
}

/// WebSocket Client.
pub struct WebSocket {
  command: mpsc::UnboundedSender<Command>,
}

impl WebSocket {
  pub async fn new(endpoints: &[String]) -> WebSocketInitResult<Self> {
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<Command>();
    let handle = ::tokio::spawn(Self::event_loop(cmd_rx, endpoints));
    return Ok(Self { command: cmd_tx });
  }

  async fn event_loop(
    mut cmd: mpsc::UnboundedReceiver<Command>,
    endpoints: &[String],
  ) -> WebsocketHandleResult<()> {
    let mut socket = Self::connect(endpoints).await?;
    loop {
      match cmd.recv().await {
        Some(Command::Terminate) => {
          break;
        }
        None => {
          break;
        }
        Some(Command::Reconnect(code, reason)) => {
          if let Err(e) = socket
            .close(
              CloseFrame {
                code,
                reason: reason.into(),
              }
              .into(),
            )
            .await
          {
            error!(error = as_error!(e); "Reconnect: Failed to close socket.");
          }
          socket = match Self::connect(endpoints).await {
            Err(e) => {
              error!(error = as_error!(e); "Reconnect: Failed to connect.");
            }
            Ok(socket) => socket,
          };
        }
        Some(Command::Close(code, reason)) => {
          if let Err(e) = socket
            .close(
              CloseFrame {
                code,
                reason: reason.into(),
              }
              .into(),
            )
            .await
          {
            error!(error = as_error!(e); "Reconnect: Failed to close socket.");
          }
        }
        Some(Command::Send(msg)) => {
          if let Err(e) = socket.send(msg).await {
            error!(error = as_error!(e); "Send: Failed to send message.");
          }
          if let Err(e) = socket.flush().await {
            error!(error = as_error!(e); "Send: Failed to flush socket.");
          }
        }
      }
    }
    return Ok(());
  }

  async fn connect(endpoints: &[String]) -> WebSocketInitResult<TLSWebSocket> {
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

  async fn close(&mut self, code: CloseCode, reason: &str) -> WSResult<()> {
    return self
      .socket
      .close(
        CloseFrame {
          code,
          reason: reason.to_string().into(),
        }
        .into(),
      )
      .await;
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
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let me = self.get_mut();
    match me.socket.poll_next_unpin(cx) {
      Poll::Ready(payload) => match block_on(me.reconnect_on_error(payload)) {
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
        Ok(Some(msg)) => match block_on(me.handle_message(msg)) {
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
      },
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
    return self
      .socket
      .poll_ready_unpin(cx)
      .map(|res| res.map_err(|err| err.into()));
  }

  fn poll_close(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    return self
      .socket
      .poll_close_unpin(cx)
      .map(|res| res.map_err(|err| err.into()));
  }

  fn poll_flush(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    return self
      .socket
      .poll_flush_unpin(cx)
      .map(|res| res.map_err(|err| err.into()));
  }

  fn start_send(self: Pin<&mut Self>, item: W) -> Result<(), Self::Error> {
    let payload = jsonify(&item)?;
    let msg = Message::Text(payload);
    let me = self.get_mut();

    return block_on(me.send_msg(msg)).map_err(|err| err.into());
  }
}

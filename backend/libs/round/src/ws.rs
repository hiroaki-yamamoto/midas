use ::std::marker::PhantomData;
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
use ::tokio::select;
use ::tokio::sync::mpsc;
use ::tokio::task::JoinHandle;
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
pub struct WebSocket<R, W> {
  ev_handle: JoinHandle<WebsocketHandleResult<()>>,
  command: mpsc::UnboundedSender<Command>,
  payload: mpsc::UnboundedReceiver<Message>,
  _r: PhantomData<R>,
  _w: PhantomData<W>,
}

impl<R, W> WebSocket<R, W>
where
  R: DeserializeOwned + Unpin + Send + 'static,
  W: Serialize + Unpin + Send + 'static,
{
  pub async fn new(endpoints: &[String]) -> WebSocketInitResult<Self> {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<Command>();
    let (payload_tx, payload_rx) = mpsc::unbounded_channel::<Message>();
    let handle = ::tokio::spawn(Self::event_loop(
      cmd_rx,
      payload_tx,
      endpoints.to_owned(),
    ));
    return Ok(Self {
      ev_handle: handle,
      command: cmd_tx,
      payload: payload_rx,
      _r: PhantomData,
      _w: PhantomData,
    });
  }

  async fn event_loop(
    mut cmd: mpsc::UnboundedReceiver<Command>,
    payload_tx: mpsc::UnboundedSender<Message>,
    endpoints: Vec<String>,
  ) -> WebsocketHandleResult<()> {
    let mut socket = Self::connect(endpoints.as_slice()).await?;
    loop {
      select! {
        cmd_payload = cmd.recv() => {
          match cmd_payload {
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
              socket = match Self::connect(endpoints.as_slice()).await {
                Err(e) => {
                  error!(error = as_error!(e); "Reconnect: Failed to connect.");
                  continue;
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
        },
        Some(payload) = socket.next() => {
          match payload {
            Err(e) => {
              error!(error = as_error!(e); "Receive: Failed to receive payload.");
              continue;
            }
            Ok(msg) => {
              if let Err(e) = payload_tx.send(msg) {
                error!(error = as_error!(e); "Receive: Failed to signal payload.");
              }
            }
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

  fn close(&mut self, code: CloseCode, reason: &str) {
    let _ = self.command.send(Command::Close(code, reason.to_string()));
  }

  fn reconnect_on_error(
    &self,
    payload: Option<Message>,
  ) -> WebsocketHandleResult<Option<Message>> {
    match payload {
      None => {
        error!("Received close payload from stream. Re-Connecting...");
        self.command.send(Command::Reconnect(
          CloseCode::Abnormal,
          "Received close payload from stream.".to_string(),
        ));
        return Ok(None);
      }
      Some(msg) => return Ok(Some(msg)),
    }
  }

  fn handle_message(&self, msg: Message) -> WebsocketMessageResult<Option<R>> {
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
        let _ = self.command.send(Command::Send(Message::Pong(payload)));
        return Ok(None);
      }
      Message::Pong(msg) => {
        info!(message = String::from_utf8_lossy(&msg); "Received Pong Message");
        return Ok(None);
      }
      Message::Close(_) => {
        error!("Disconnected. Re-Connecting...");
        let _ = self.command.send(Command::Reconnect(
          CloseCode::Abnormal,
          "Disconnected.".to_string(),
        ));
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
  R: DeserializeOwned + Send + Unpin + 'static,
  W: Serialize + Send + Unpin + 'static,
{
  type Item = R;
  fn poll_next(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let me = self.get_mut();
    match me.payload.poll_recv(cx) {
      Poll::Ready(payload) => match me.reconnect_on_error(payload) {
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
        Ok(Some(msg)) => match me.handle_message(msg) {
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
  R: DeserializeOwned + Unpin + Send + 'static,
  W: Serialize + Unpin + Send + 'static,
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
    return me
      .command
      .send(Command::Send(msg))
      .map_err(|err| WebsocketSinkError::CommandSendError(err.to_string()));
  }
}

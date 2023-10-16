use ::std::marker::PhantomData;
use ::std::pin::Pin;
use ::std::sync::Arc;
use ::std::task::{Context, Poll};
use ::std::time::Duration;

use ::futures::sink::{Sink, SinkExt};
use ::futures::stream::{Stream, StreamExt};
use ::log::{as_display, as_error, error, info, warn};
use ::rand::thread_rng;
use ::rand::Rng;
use ::serde::{de::DeserializeOwned, ser::Serialize};
use ::serde_json::{from_str as json_parse, to_string as jsonify};
use ::tokio::select;
use ::tokio::sync::mpsc;
use ::tokio::sync::RwLock;
use ::tokio::task::JoinHandle;
use ::tokio::time::interval;
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use ::tokio_tungstenite::tungstenite::protocol::CloseFrame;
use ::tokio_tungstenite::tungstenite::Message;
use futures::FutureExt;

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
  Flush,
}

/// WebSocket Client.
pub struct WebSocket<R, W>
where
  R: DeserializeOwned + Unpin + Send + 'static,
  W: Serialize + Unpin + Send + 'static,
{
  _ev_handle: JoinHandle<WebsocketHandleResult<()>>,
  command: mpsc::UnboundedSender<Command>,
  payload: mpsc::UnboundedReceiver<Message>,
  is_running: Arc<RwLock<bool>>,
  _r: PhantomData<R>,
  _w: PhantomData<W>,
}

impl<R, W> Drop for WebSocket<R, W>
where
  R: DeserializeOwned + Unpin + Send + 'static,
  W: Serialize + Unpin + Send + 'static,
{
  fn drop(&mut self) {
    let _ = self.command.send(Command::Terminate);
  }
}

impl<R, W> WebSocket<R, W>
where
  R: DeserializeOwned + Unpin + Send + 'static,
  W: Serialize + Unpin + Send + 'static,
{
  pub async fn new(endpoints: &[String]) -> WebSocketInitResult<Self> {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<Command>();
    let (payload_tx, payload_rx) = mpsc::unbounded_channel::<Message>();
    let is_running = Arc::new(RwLock::new(false));
    let handle = ::tokio::spawn(Self::event_loop(
      cmd_rx,
      payload_tx,
      is_running.clone(),
      endpoints.to_owned(),
    ));
    return Ok(Self {
      _ev_handle: handle,
      command: cmd_tx,
      is_running,
      payload: payload_rx,
      _r: PhantomData,
      _w: PhantomData,
    });
  }

  async fn event_loop(
    mut cmd: mpsc::UnboundedReceiver<Command>,
    payload_tx: mpsc::UnboundedSender<Message>,
    is_running: Arc<RwLock<bool>>,
    endpoints: Vec<String>,
  ) -> WebsocketHandleResult<()> {
    let mut socket = Self::connect(endpoints.as_slice()).await?;
    *is_running.write().await = true;
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
            },
            Some(Command::Flush) => {
              if let Err(e) = socket.flush().await {
                error!(error = as_error!(e); "Flush: Failed to flush socket.");
              }
            },
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

  pub fn close(&mut self, code: CloseCode, reason: &str) {
    let _ = self.command.send(Command::Close(code, reason.to_string()));
  }

  fn reconnect_on_error(
    &self,
    payload: Option<Message>,
  ) -> WebsocketHandleResult<Option<Message>> {
    match payload {
      None => {
        error!("Received close payload from stream. Re-Connecting...");
        let _ = self.command.send(Command::Reconnect(
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
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    let is_running = match self.is_running.read().boxed().poll_unpin(cx) {
      Poll::Ready(is_running) => is_running,
      Poll::Pending => return Poll::Pending,
    };
    return if *is_running {
      Poll::Ready(Ok(()))
    } else {
      Poll::Pending
    };
  }

  fn poll_close(
    self: Pin<&mut Self>,
    _: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    let _ = self
      .command
      .send(Command::Close(CloseCode::Normal, "Bye.".to_string()))
      .map_err(|err| {
        return WebsocketSinkError::CommandSendError(format!(
          "poll_close failure: {:?}",
          err
        ));
      })?;
    return Poll::Ready(Ok(()));
  }

  fn poll_flush(
    self: Pin<&mut Self>,
    _: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    let _ = self.command.send(Command::Flush).map_err(|err| {
      WebsocketSinkError::CommandSendError(format!(
        "poll_flush failure: {:?}",
        err
      ))
    })?;
    return Poll::Ready(Ok(()));
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

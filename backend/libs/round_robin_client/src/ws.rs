use ::std::marker::PhantomData;
use ::std::pin::Pin;
use ::std::task::{Context, Poll};
use ::std::time::Duration;

use ::futures::executor::block_on;
use ::futures::sink::Sink;
use ::futures::sink::SinkExt;
use ::futures::stream::{Stream, StreamExt};
use ::log::{as_display, error, info, warn};
use ::rand::thread_rng;
use ::rand::Rng;
use ::serde::{de::DeserializeOwned, ser::Serialize};
use ::serde_json::{from_str as json_parse, to_string as jsonify};
use ::tokio::time::interval;
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::{
  protocol::frame::coding::CloseCode, protocol::CloseFrame, Error as WSError,
  Message, Result as WSResult,
};
use futures::FutureExt;

use ::errors::{
  MaximumAttemptExceeded, WebSocketInitResult, WebsocketMessageResult,
  WebsocketSinkError,
};
use ::types::TLSWebSocket;

use crate::entities::WSMessageDetail as MsgDetail;

/// WebSocket Client.
/// R = Read Entity
/// W = Write Entity
pub struct WebSocket<R, W>
where
  R: DeserializeOwned + Unpin,
  W: Serialize + Unpin,
{
  socket: TLSWebSocket,
  _r: PhantomData<R>,
  _w: PhantomData<W>,
}

impl<R, W> WebSocket<R, W>
where
  R: DeserializeOwned + Unpin,
  W: Serialize + Unpin,
{
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

  pub async fn new(endpoints: &[String]) -> WebSocketInitResult<Self> {
    let socket = Self::connect(endpoints).await?;
    return Ok(Self {
      socket,
      _r: PhantomData,
      _w: PhantomData,
    });
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

  async fn send_msg(&mut self, msg: Message) -> WSResult<()> {
    let send_result = self.socket.send(msg).await;
    let flush_result = self.socket.flush().await;
    return send_result.and(flush_result);
  }

  async fn handle_message(
    &mut self,
    msg: Message,
  ) -> WebsocketMessageResult<MsgDetail<R>> {
    match msg {
      Message::Text(text) => {
        info!(payload = text; "Received Text Message");
        let payload: R = json_parse(text.as_str())?;
        return Ok(MsgDetail::EntityReceived(payload));
      }
      Message::Binary(blob) => {
        info!(payload = String::from_utf8_lossy(&blob); "Received Binary Message");
        let payload = String::from_utf8(blob)?;
        let payload: R = json_parse(&payload)?;
        return Ok(MsgDetail::EntityReceived(payload));
      }
      Message::Ping(payload) => {
        info!(payload = String::from_utf8_lossy(&payload); "Received Ping Message");
        let _ = self.send_msg(Message::Pong(payload)).await?;
        return Ok(MsgDetail::Continue);
      }
      Message::Pong(payload) => {
        info!(payload = String::from_utf8_lossy(&payload); "Received Pong Message");
        return Ok(MsgDetail::Continue);
      }
      Message::Close(_) => {
        error!("Disconnected.");
        let _ = self.socket.close(None).await;
        return Ok(MsgDetail::Disconnected);
      }
      Message::Frame(frame) => {
        warn!(frame = as_display!(frame); "Received Unexpected Frame Message");
        return Ok(MsgDetail::Continue);
      }
    }
  }

  async fn next_item(&mut self) -> WebsocketMessageResult<MsgDetail<R>> {
    let payload = self.socket.next().await;
    return match payload {
      None => {
        return Err(WSError::AlreadyClosed.into());
      }
      Some(payload) => {
        let payload = payload?;
        self.handle_message(payload).await
      }
    };
  }
}

impl<R, W> Drop for WebSocket<R, W>
where
  R: DeserializeOwned + Unpin,
  W: Serialize + Unpin,
{
  fn drop(&mut self) {
    let _ = block_on(self.close(CloseCode::Normal, "Client Closed."));
  }
}

impl<R, W> Stream for WebSocket<R, W>
where
  R: DeserializeOwned + Unpin,
  W: Serialize + Unpin,
{
  type Item = MsgDetail<R>;
  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let payload = self.next_item().boxed_local().poll_unpin(cx);
    if let Poll::Ready(payload) = payload {
      return match payload {
        Ok(payload) => Poll::Ready(Some(payload)),
        Err(err) => {
          error!(error = as_display!(err); "Error while polling websocket stream.");
          Poll::Ready(None)
        }
      };
    }
    return Poll::Pending;
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

  fn start_send(mut self: Pin<&mut Self>, item: W) -> Result<(), Self::Error> {
    let payload = jsonify(&item)?;
    let msg = Message::Text(payload);

    return block_on(self.send_msg(msg)).map_err(|err| err.into());
  }
}

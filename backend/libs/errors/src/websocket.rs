use ::err_derive::Error;
use ::serde::Serialize;
use ::tokio_tungstenite::tungstenite::Error as SocketError;

use super::MaximumAttemptExceeded;

#[derive(Debug, Clone, Serialize, Default, Error)]
#[error(display = "Websocket Error (status: {:?}, msg: {:?})", status, msg)]
pub struct WebsocketError {
  pub status: Option<u16>,
  pub msg: Option<String>,
}

#[derive(Debug, Error)]
pub enum WebsocketInitError {
  #[error(display = "Socket Error: {:?}", _0)]
  SocketError(#[source] SocketError),
  #[error(display = "Maximum Attempt Exceeded: {:?}", _0)]
  MaximumAttemptExceeded(#[source] MaximumAttemptExceeded),
}

pub type WebSocketInitResult<T> = Result<T, WebsocketInitError>;

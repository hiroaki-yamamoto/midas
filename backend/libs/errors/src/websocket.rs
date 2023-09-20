use ::err_derive::Error;
use ::serde::Serialize;
use ::std::io::Error as IoError;
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
  #[error(display = "IO Error: {:?}", _0)]
  IoError(#[source] IoError),
}

#[derive(Debug, Error)]
pub enum WebsocketHandleError {
  #[error(display = "Socket Error: {:?}", _0)]
  SocketError(#[source] SocketError),
  #[error(display = "Socket Init Error: {:?}", _0)]
  SocketInitError(#[source] WebsocketInitError),
}

pub type WebSocketInitResult<T> = Result<T, WebsocketInitError>;
pub type WebsocketHandleResult<T> = Result<T, WebsocketHandleError>;

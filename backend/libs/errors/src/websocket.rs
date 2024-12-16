use ::err_derive::Error;
use ::serde::Serialize;
use ::serde_json::Error as JsonError;
use ::std::io::Error as IoError;
use ::std::str::Utf8Error;
use ::std::string::FromUtf8Error;
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

#[derive(Debug, Error)]
pub enum WebsocketMessageError {
  #[error(display = "WebSocket Error: {:?}", _0)]
  WebSocketError(#[source] SocketError),
  #[error(display = "Socket Init Error: {:?}", _0)]
  SocketInitError(#[source] WebsocketInitError),
  #[error(display = "JSON decode Error: {:?}", _0)]
  JsonError(#[source] JsonError),
  #[error(display = "UTF8 Decode error: {:?}", _0)]
  UTF8DecodeError(#[source] FromUtf8Error),
  #[error(display = "UTF8 Error: {:?}", _0)]
  UTF8Error(#[source] Utf8Error),
}

#[derive(Debug, Error)]
pub enum WebsocketSinkError {
  #[error(display = "WebSocket Error: {:?}", _0)]
  WebSocketError(#[source] SocketError),
  #[error(display = "JSON Encode Error: {:?}", _0)]
  JsonError(#[source] JsonError),
}

pub type WebSocketInitResult<T> = Result<T, WebsocketInitError>;
pub type WebsocketHandleResult<T> = Result<T, WebsocketHandleError>;
pub type WebsocketMessageResult<T> = Result<T, WebsocketMessageError>;
pub type WebsocketSinkResult<T> = Result<T, WebsocketSinkError>;

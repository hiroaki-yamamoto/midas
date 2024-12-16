use ::serde::Serialize;
use ::serde_json::Error as JsonError;
use ::std::io::Error as IoError;
use ::std::str::Utf8Error;
use ::std::string::FromUtf8Error;
use ::thiserror::Error;
use ::tokio_tungstenite::tungstenite::Error as SocketError;

use super::MaximumAttemptExceeded;

#[derive(Debug, Clone, Serialize, Default, Error)]
#[error("Websocket Error (status: {:?}, msg: {:?})", status, msg)]
pub struct WebsocketError {
  pub status: Option<u16>,
  pub msg: Option<String>,
}

#[derive(Debug, Error)]
pub enum WebsocketInitError {
  #[error("Socket Error: {:?}", _0)]
  SocketError(#[from] SocketError),
  #[error("Maximum Attempt Exceeded: {:?}", _0)]
  MaximumAttemptExceeded(#[from] MaximumAttemptExceeded),
  #[error("IO Error: {:?}", _0)]
  IoError(#[from] IoError),
}

#[derive(Debug, Error)]
pub enum WebsocketHandleError {
  #[error("Socket Error: {:?}", _0)]
  SocketError(#[from] SocketError),
  #[error("Socket Init Error: {:?}", _0)]
  SocketInitError(#[from] WebsocketInitError),
}

#[derive(Debug, Error)]
pub enum WebsocketMessageError {
  #[error("WebSocket Error: {:?}", _0)]
  WebSocketError(#[from] SocketError),
  #[error("Socket Init Error: {:?}", _0)]
  SocketInitError(#[from] WebsocketInitError),
  #[error("JSON decode Error: {:?}", _0)]
  JsonError(#[from] JsonError),
  #[error("UTF8 Decode error: {:?}", _0)]
  UTF8DecodeError(#[from] FromUtf8Error),
  #[error("UTF8 Error: {:?}", _0)]
  UTF8Error(#[from] Utf8Error),
}

#[derive(Debug, Error)]
pub enum WebsocketSinkError {
  #[error("WebSocket Error: {:?}", _0)]
  WebSocketError(#[from] SocketError),
  #[error("JSON Encode Error: {:?}", _0)]
  JsonError(#[from] JsonError),
}

pub type WebSocketInitResult<T> = Result<T, WebsocketInitError>;
pub type WebsocketHandleResult<T> = Result<T, WebsocketHandleError>;
pub type WebsocketMessageResult<T> = Result<T, WebsocketMessageError>;
pub type WebsocketSinkResult<T> = Result<T, WebsocketSinkError>;

use ::async_nats::jetstream::context::CreateStreamError as NatsCreateStreamError;
use ::err_derive::Error;
use ::tokio::sync::mpsc::error::SendError;
use ::tokio::sync::oneshot::error::RecvError;

use crate::kvs::KVSError;
use crate::pubsub::{ConsumerError, RequestError};
use crate::unknown::UnknownExchangeError;
use crate::websocket::{WebsocketInitError, WebsocketSinkError};

#[derive(Debug, Error)]
pub enum ObserverError {
  #[error(display = "(Un)Subscribe Event Signaling Send Error: {}", _0)]
  SubscribeSendError(#[source] SendError<Vec<String>>),
  #[error(display = "(Un)subscribe Event Signaling Receive Error: {}", _0)]
  SubscribeRecvError(#[source] RecvError),
  #[error(display = "Websocket Initialization Error: {}", _0)]
  WebsocketInitErr(#[source] WebsocketInitError),
  #[error(display = "Websocket Sink Error: {}", _0)]
  WebsocketSinkErr(#[source] WebsocketSinkError),
  #[error(display = "Nats Stream Creation Error: {}", _0)]
  NatsCreateStreamError(#[source] NatsCreateStreamError),
  #[error(display = "NATS consumer error: {}", _0)]
  ConsumerError(#[source] ConsumerError),
  #[error(display = "NATS request error: {}", _0)]
  PublishError(#[source] RequestError),
  #[error(display = "KVS Error: {}", _0)]
  KVSError(#[source] KVSError),
  #[error(display = "Unknown Exchange: {}", _0)]
  UnknownExchangeError(#[source] UnknownExchangeError),
  #[error(display = "Unhandled Error: {}", _0)]
  Other(String),
}

pub type ObserverResult<T> = Result<T, ObserverError>;

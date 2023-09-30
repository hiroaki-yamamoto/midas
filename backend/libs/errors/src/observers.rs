use ::async_nats::jetstream::context::CreateStreamError as NatsCreateStreamError;
use ::err_derive::Error;

use crate::pubsub::{ConsumerError, RequestError};
use crate::websocket::{WebsocketInitError, WebsocketSinkError};

#[derive(Debug, Error)]
pub enum ObserverError {
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
}

pub type ObserverResult<T> = Result<T, ObserverError>;

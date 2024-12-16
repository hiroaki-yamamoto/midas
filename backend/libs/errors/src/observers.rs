use ::async_nats::jetstream::context::CreateStreamError as NatsCreateStreamError;
use ::mongodb::error::Error as DBErr;
use ::serde::Serialize;
use ::thiserror::Error;
use ::tokio::sync::mpsc::error::SendError;
use ::tokio::sync::oneshot::error::RecvError;
use ::tokio::task::JoinError;

use crate::dlock::DLockError;
use crate::kvs::KVSError;
use crate::pubsub::{ConsumerError, PublishError};
use crate::unknown::UnknownExchangeError;
use crate::websocket::{WebsocketInitError, WebsocketSinkError};

#[derive(Debug, Serialize, Error)]
#[error("Socket Not Found. id: {}", id)]
pub struct SocketNotFound {
  id: String,
}

impl SocketNotFound {
  pub fn new(id: String) -> Self {
    Self { id }
  }
}

#[derive(Debug, Error)]
pub enum ObserverError {
  #[error("DB Error: {}", _0)]
  DB(#[from] DBErr),
  #[error("DLock Error: {}", _0)]
  DLockError(#[from] DLockError),
  #[error("(Un)Subscribe Event Signaling Send Error: {}", _0)]
  SubscribeSendError(#[from] SendError<Vec<String>>),
  #[error("(Un)subscribe Event Signaling Receive Error: {}", _0)]
  SubscribeRecvError(#[from] RecvError),
  #[error("Websocket Initialization Error: {}", _0)]
  WebsocketInitErr(#[from] WebsocketInitError),
  #[error("Websocket Sink Error: {}", _0)]
  WebsocketSinkErr(#[from] WebsocketSinkError),
  #[error("Nats Stream Creation Error: {}", _0)]
  NatsCreateStreamError(#[from] NatsCreateStreamError),
  #[error("NATS consumer error: {}", _0)]
  ConsumerError(#[from] ConsumerError),
  #[error("NATS publish error: {}", _0)]
  PublishError(#[from] PublishError),
  #[error("KVS Error: {}", _0)]
  KVSError(#[from] KVSError),
  #[error("Unknown Exchange: {}", _0)]
  UnknownExchangeError(#[from] UnknownExchangeError),
  #[error("Parallel Join Error: {}", _0)]
  JoinError(#[from] JoinError),
  #[error("Socket Not Found: {}", _0)]
  SocketNotFound(#[from] SocketNotFound),
  #[error("Unhandled Error: {}", _0)]
  Other(String),
}

pub type ObserverResult<T> = Result<T, ObserverError>;

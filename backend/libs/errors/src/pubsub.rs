use ::async_nats::jetstream::consumer::StreamError;
use ::async_nats::jetstream::context::CreateStreamError;
use ::async_nats::jetstream::context::PublishError as JSPublishError;
use ::async_nats::jetstream::stream::ConsumerError as JSConsumerError;
use ::err_derive::Error;
use ::rmp_serde::decode::Error as MsgPackDecErr;
use ::rmp_serde::encode::Error as MsgPackEncErr;

#[derive(Debug, Error)]
pub enum PublishError {
  #[error(display = "Publish error: {}", _0)]
  PublishError(#[source] JSPublishError),
  #[error(display = "Msgpack encode error: {}", _0)]
  EncodeError(#[source] MsgPackEncErr),
}

#[derive(Debug, Error)]
pub enum RespondError {
  #[error(display = "Nats Publish error: {}", _0)]
  PublishError(#[source] JSPublishError),
  #[error(display = "Msgpack encode error: {}", _0)]
  EncodeError(#[source] MsgPackEncErr),
  #[error(display = "No Reply subject")]
  NoReplySubject,
  #[error(display = "No Header")]
  NoHeaders,
}

#[derive(Debug, Error)]
pub enum ConsumerError {
  #[error(display = "Stream Creation Error: {}", _0)]
  CreateStreamError(#[source] CreateStreamError),
  #[error(display = "Consumer error: {}", _0)]
  ConsumerError(#[source] JSConsumerError),
  #[error(display = "Stream Error: {}", _0)]
  StreamError(#[source] StreamError),
  #[error(display = "Msgpack decode error: {}", _0)]
  DecodeError(#[source] MsgPackDecErr),
}

pub type CreateStreamResult<T> = Result<T, CreateStreamError>;
pub type ConsumerResult<T> = Result<T, ConsumerError>;
pub type PublishResult<T> = Result<T, PublishError>;
pub type RespondResult<T> = Result<T, RespondError>;

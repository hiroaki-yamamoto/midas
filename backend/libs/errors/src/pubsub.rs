use ::async_nats::jetstream::consumer::StreamError;
use ::async_nats::jetstream::context::CreateStreamError;
use ::async_nats::jetstream::context::PublishError as JSPublishError;
use ::async_nats::jetstream::stream::ConsumerError as JSConsumerError;
use ::rmp_serde::decode::Error as MsgPackDecErr;
use ::rmp_serde::encode::Error as MsgPackEncErr;
use ::thiserror::Error;

#[derive(Debug, Error)]
pub enum PublishError {
  #[error("Publish error: {}", _0)]
  PublishError(#[from] JSPublishError),
  #[error("Msgpack encode error: {}", _0)]
  EncodeError(#[from] MsgPackEncErr),
}

#[derive(Debug, Error)]
pub enum RespondError {
  #[error("Nats Publish error: {}", _0)]
  PublishError(#[from] JSPublishError),
  #[error("Msgpack encode error: {}", _0)]
  EncodeError(#[from] MsgPackEncErr),
  #[error("No Reply subject")]
  NoReplySubject,
  #[error("No Header")]
  NoHeaders,
}

#[derive(Debug, Error)]
pub enum ConsumerError {
  #[error("Stream Creation Error: {}", _0)]
  CreateStreamError(#[from] CreateStreamError),
  #[error("Consumer error: {}", _0)]
  ConsumerError(#[from] JSConsumerError),
  #[error("Stream Error: {}", _0)]
  StreamError(#[from] StreamError),
  #[error("Msgpack decode error: {}", _0)]
  DecodeError(#[from] MsgPackDecErr),
}

pub type CreateStreamResult<T> = Result<T, CreateStreamError>;
pub type ConsumerResult<T> = Result<T, ConsumerError>;
pub type PublishResult<T> = Result<T, PublishError>;
pub type RespondResult<T> = Result<T, RespondError>;

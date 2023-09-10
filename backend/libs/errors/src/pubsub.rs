use ::async_nats::jetstream::consumer::StreamError;
use ::async_nats::jetstream::context::CreateStreamError;
use ::async_nats::jetstream::context::PublishError as JSPublishError;
use ::async_nats::jetstream::stream::ConsumerError as JSConsumerError;
use ::async_nats::RequestError as NatsRequestError;
use ::err_derive::Error;
use ::rmp_serde::decode::Error as MsgPackDecErr;
use ::rmp_serde::encode::Error as MsgPackEncErr;

#[derive(Debug, Error)]
pub enum PublishError {
  #[error(display = "Publish error: {}", _0)]
  PublishError(#[source] JSPublishError),
  #[error(display = "Msgpack encode error: {}", _0)]
  EncodeError(#[source] MsgPackEncErr),
  #[error(display = "Respond Error: No Reply subject")]
  NoReplySubject,
}

#[derive(Debug, Error)]
pub enum ConsumerError {
  #[error(display = "Consumer error: {}", _0)]
  ConsumerError(#[source] JSConsumerError),
  #[error(display = "Stream Error: {}", _0)]
  StreamError(#[source] StreamError),
  #[error(display = "Msgpack decode error: {}", _0)]
  DecodeError(#[source] MsgPackDecErr),
}

#[derive(Debug, Error)]
pub enum RequestError {
  #[error(display = "Request error: {}", _0)]
  RequestError(#[source] NatsRequestError),
  #[error(display = "Msgpack decode error: {}", _0)]
  DecodeError(#[source] MsgPackDecErr),
  #[error(display = "Msgpack encode error: {}", _0)]
  EncodeError(#[source] MsgPackEncErr),
}

pub type CreateStreamResult<T> = Result<T, CreateStreamError>;
pub type ConsumerResult<T> = Result<T, ConsumerError>;
pub type PublishResult<T> = Result<T, PublishError>;
pub type RequestResult<T> = Result<T, RequestError>;

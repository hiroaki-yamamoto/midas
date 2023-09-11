use ::async_trait::async_trait;
use ::bytes::Bytes;
use ::errors::{RespondError, RespondResult};
use ::rmp_serde::to_vec as to_msgpack;
use ::serde::Serialize;

use crate::natsJS::context::PublishAckFuture;
use crate::natsJS::message::Message;

#[async_trait]
pub trait Respond<T>
where
  T: Serialize + Send + Sync,
{
  async fn respond(&self, msg: &T) -> RespondResult<PublishAckFuture>;
}

#[async_trait]
impl<T> Respond<T> for Message
where
  T: Serialize + Send + Sync,
{
  async fn respond(&self, msg: &T) -> RespondResult<PublishAckFuture> {
    let serialized = to_msgpack(msg).map(|v| Bytes::from(v))?;
    let headers = self.headers.clone().ok_or(RespondError::NoHeaders)?;
    let reply_subject = headers
      .get("midas-respond-subject")
      .ok_or(RespondError::NoReplySubject)?;
    return Ok(
      self
        .context
        .publish(reply_subject.into(), serialized)
        .await?,
    );
  }
}

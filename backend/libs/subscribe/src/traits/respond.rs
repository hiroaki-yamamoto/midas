use ::async_trait::async_trait;
use ::bytes::Bytes;
use ::errors::{PublishError, PublishResult};
use ::rmp_serde::to_vec as to_msgpack;
use ::serde::Serialize;

use crate::natsJS::context::PublishAckFuture;
use crate::natsJS::message::Message;

#[async_trait]
pub trait Respond<T>
where
  T: Serialize + Send + Sync,
{
  async fn respond(&self, msg: &T) -> PublishResult<PublishAckFuture>;
}

#[async_trait]
impl<T> Respond<T> for Message
where
  T: Serialize + Send + Sync,
{
  async fn respond(&self, msg: &T) -> PublishResult<PublishAckFuture> {
    let serialized = to_msgpack(msg).map(|v| Bytes::from(v))?;
    let reply_subject =
      self.reply.as_ref().ok_or(PublishError::NoReplySubject)?;
    return Ok(
      self
        .context
        .publish(reply_subject.clone(), serialized)
        .await?,
    );
  }
}

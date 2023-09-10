use ::async_trait::async_trait;
use ::bytes::Bytes;
use ::std::borrow::Borrow;

use ::futures::stream::{BoxStream, StreamExt};
use ::log::warn;
use ::rmp_serde::{
  encode::Error as EncodeErr, from_slice as from_msgpack, to_vec as to_msgpack,
};
use ::serde::de::DeserializeOwned;
use ::serde::ser::Serialize;

use ::errors::{ConsumerResult, PublishResult, RequestResult};

use crate::natsJS::consumer::{
  pull::Config, AckPolicy, Consumer, DeliverPolicy,
};
use crate::natsJS::stream::Stream as NatsJS;

use crate::nats::Client;
use crate::natsJS::context::{Context, PublishAckFuture};
use crate::natsJS::message::Message;

#[async_trait]
pub trait PubSub<T>
where
  T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
{
  fn get_client(&self) -> &Client;
  fn get_subject(&self) -> &str;
  fn get_stream(&self) -> &NatsJS;
  fn get_ctx(&self) -> &Context;

  async fn add_consumer<C>(
    &self,
    consumer_name: C,
  ) -> ConsumerResult<Consumer<Config>>
  where
    C: AsRef<str> + Send + Sync,
  {
    let stream = self.get_stream();
    let mut cfg = Config {
      name: Some(consumer_name.as_ref().into()),
      max_deliver: 1024,
      memory_storage: true,
      ..Default::default()
    };
    // let mut cfg = push::Config {
    //   deliver_subject: format!(
    //     "{}.{}.deliver",
    //     self.get_subject(),
    //     consumer_name.as_ref()
    //   ),
    //   deliver_group: Some(consumer_name.as_ref().into()),
    //   name: Some(consumer_name.as_ref().into()),
    //   flow_control: true,
    //   idle_heartbeat: Duration::from_secs(1),
    //   max_deliver: 1024,
    //   memory_storage: true,
    //   ..Default::default()
    // };
    cfg.deliver_policy = DeliverPolicy::All;
    cfg.ack_policy = AckPolicy::Explicit;
    return Ok(
      stream
        .get_or_create_consumer(self.get_subject(), cfg)
        .await?,
    );
  }

  fn serialize<S>(entity: &S) -> Result<Bytes, EncodeErr>
  where
    S: Serialize,
  {
    return to_msgpack(entity).map(|v| Bytes::from(v));
  }

  async fn publish(
    &self,
    entity: impl Borrow<T> + Send + Sync,
  ) -> PublishResult<PublishAckFuture> {
    let msg = Self::serialize(entity.borrow())?;
    let res = self.get_ctx().publish(self.get_subject().into(), msg).await;
    return Ok(res?);
  }

  async fn request(
    &self,
    entity: impl Borrow<T> + Send + Sync,
  ) -> RequestResult<T> {
    let msg = Self::serialize(entity.borrow())?;
    let res = self
      .get_client()
      .request(self.get_subject().into(), msg)
      .await?;
    let res: T = from_msgpack(&res.payload)?;
    return Ok(res);
  }

  async fn pull_subscribe<C>(
    &self,
    consumer_name: C,
  ) -> ConsumerResult<BoxStream<(T, Message)>>
  where
    C: AsRef<str> + Send + Sync,
  {
    let consumer = self.add_consumer(consumer_name).await?;
    let msg = consumer
      .messages()
      .await?
      .filter_map(|msg_result| async {
        if let Err(e) = msg_result {
          warn!("Msg Stream Failure: {:?}", e);
          return None;
        }
        return msg_result.ok();
      })
      .filter_map(|msg| async {
        let _ = msg.ack().await.ok()?;
        return Some(msg);
      })
      .map(|msg| {
        return (from_msgpack::<T>(&msg.payload), msg);
      })
      .filter_map(|(res, msg)| async {
        if let Err(ref e) = res {
          warn!("Msg deserialization failure: {:?}", e);
          return None;
        }
        return res.ok().map(|obj| (obj, msg));
      })
      .boxed();
    return Ok(msg);
  }
}

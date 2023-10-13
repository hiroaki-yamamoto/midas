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

use ::errors::{ConsumerResult, CreateStreamResult, PublishResult};

use crate::natsJS::consumer::{
  pull::Config as PullSubscribeConfig, AckPolicy, Consumer, DeliverPolicy,
};
use crate::natsJS::stream::{Config as StreamConfig, Stream as NatsJS};

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
  fn get_ctx(&self) -> &Context;

  async fn get_or_create_stream(
    &self,
    stream_name: Option<&str>,
  ) -> CreateStreamResult<NatsJS> {
    let mut option: StreamConfig =
      stream_name.unwrap_or(self.get_subject()).into();
    option.max_consumers = -1;
    log::debug!(stream_name = option.name; "Reached pre-stream creation.");
    return self.get_ctx().get_or_create_stream(option).await;
  }

  async fn add_consumer(
    &self,
    durable_name: &str,
    stream_name: Option<&str>,
  ) -> ConsumerResult<Consumer<PullSubscribeConfig>> {
    let stream = self.get_or_create_stream(stream_name).await?;
    let mut cfg = PullSubscribeConfig {
      durable_name: Some(durable_name.into()),
      max_deliver: 1024,
      memory_storage: true,
      ..Default::default()
    };
    cfg.deliver_policy = DeliverPolicy::All;
    cfg.ack_policy = AckPolicy::Explicit;
    log::debug!(durable_name = cfg.durable_name;"Reached pre-consumer creation.");
    return Ok(stream.get_or_create_consumer(durable_name, cfg).await?);
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

  async fn raw_pull_subscribe<R>(
    &self,
    durable_name: &str,
    stream_name: Option<&str>,
  ) -> ConsumerResult<BoxStream<(R, Message)>>
  where
    R: DeserializeOwned + Send + 'life0,
  {
    let consumer = self.add_consumer(durable_name, stream_name).await?;
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
        return (from_msgpack::<R>(&msg.payload), msg);
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

  async fn pull_subscribe(
    &self,
    durable_name: &str,
  ) -> ConsumerResult<BoxStream<(T, Message)>> {
    let stream = self.raw_pull_subscribe(durable_name, None).await?;
    return Ok(stream);
  }
}

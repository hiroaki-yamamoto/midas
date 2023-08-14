use ::async_trait::async_trait;
use ::bytes::Bytes;

use ::futures::stream::{BoxStream, StreamExt};
use ::log::warn;
use ::rmp_serde::{
  encode::Error as EncodeErr, from_slice as from_msgpack, to_vec as to_msgpack,
};
use ::serde::de::DeserializeOwned;
use ::serde::ser::Serialize;

use ::errors::{ConsumerResult, PublishResult};

use crate::natsJS::consumer::{
  pull::Config, AckPolicy, Consumer, DeliverPolicy,
};
use crate::natsJS::stream::Stream as NatsJS;

use crate::natsJS::context::{Context, PublishAckFuture};
use crate::natsJS::message::Message;

#[async_trait]
pub trait PubSub<T>
where
  T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
{
  fn get_subject(&self) -> &str;
  fn get_stream(&self) -> &NatsJS;
  fn get_ctx(&self) -> &Context;

  async fn add_consumer(&self) -> ConsumerResult<Consumer<Config>> {
    let name = self.get_subject();
    let stream = self.get_stream();
    let mut cfg = Config {
      name: Some(format!("{}Consumer", name)),
      ..Default::default()
    };
    cfg.deliver_policy = DeliverPolicy::All;
    cfg.ack_policy = AckPolicy::Explicit;
    return Ok(stream.get_or_create_consumer(name, cfg).await?);
  }

  fn serialize<S>(entity: &S) -> Result<Bytes, EncodeErr>
  where
    S: Serialize,
  {
    return to_msgpack(entity).map(|v| Bytes::from(v));
  }

  async fn publish(&self, entity: &T) -> PublishResult<PublishAckFuture> {
    let msg = Self::serialize(entity)?;
    let res = self.get_ctx().publish(self.get_subject().into(), msg).await;
    return Ok(res?);
  }

  async fn queue_subscribe(&self) -> ConsumerResult<BoxStream<(T, Message)>> {
    let consumer = self.add_consumer().await?;
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
      .map(|msg| {
        let _ = msg.ack();
        return (from_msgpack::<T>(&msg.message.payload), msg);
      })
      .filter_map(|(res, msg)| async {
        if let Err(ref e) = res {
          warn!("Msg deserialization failure: {:?}", e);
          return None;
        }
        return res.ok().map(|obj| (obj, msg));
      })
      .boxed();
    // let (sender, recv) = unbounded_channel();
    // thread::spawn(move || {
    //   while let Some(msg) = sub.next() {
    //     let _ = msg.ack();
    //     let obj = from_msgpack::<T>(&msg.data).map(|obj| (obj, msg));
    //     if let Err(ref e) = obj {
    //       println!("Msg deserialization failure: {:?}", e);
    //     }
    //     let _ = sender.send(obj.unwrap());
    //   }
    // });
    return Ok(msg);
  }
}

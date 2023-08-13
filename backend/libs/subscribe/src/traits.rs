use ::std::io::Result as IOResult;
use ::std::io::{Error as IOError, ErrorKind as IOErrKind};
use ::std::thread;

use ::async_trait::async_trait;
use ::log::warn;
use ::nats::jetstream::{
  AckPolicy, ConsumerConfig, ConsumerInfo, DeliverPolicy, JetStream as NatsJS,
  PublishAck, PullSubscribeOptions,
};
use ::nats::Message;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde::de::DeserializeOwned;
use ::serde::ser::Serialize;
use ::tokio::sync::mpsc::unbounded_channel;
use ::tokio::time::sleep;
use ::tokio_stream::wrappers::UnboundedReceiverStream;

#[async_trait]
pub trait PubSub<T>
where
  T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
{
  fn get_subject(&self) -> &str;
  fn get_natsjs(&self) -> &NatsJS;

  async fn add_consumer(
    &self,
    group_name: String,
  ) -> ::std::io::Result<ConsumerInfo> {
    let js = self.get_natsjs();
    let name = self.get_subject();
    let mut cfg: ConsumerConfig = format!("{}Consumer", name).as_str().into();
    cfg.deliver_policy = DeliverPolicy::All;
    cfg.ack_policy = AckPolicy::Explicit;
    cfg.deliver_group = Some(group_name);
    const MAX_RETRY: usize = 30;
    for count in 1..=MAX_RETRY {
      match js.add_consumer(name, &cfg) {
        Ok(info) => {
          return Ok(info);
        }
        Err(e) => {
          warn!(
            "Failed to acquire consumer. Retrying after
              1 sec...({}/{}): {}",
            count, e, MAX_RETRY
          );
          sleep(::std::time::Duration::from_secs(1)).await;
        }
      }
    }
    return Err(::std::io::Error::new(
      ::std::io::ErrorKind::Other,
      "Failed to acquire consumer",
    ));
  }

  fn serialize<S>(entity: &S) -> IOResult<Vec<u8>>
  where
    S: Serialize,
  {
    return to_msgpack(entity).map_err(|e| IOError::new(IOErrKind::Other, e));
  }

  fn publish(&self, entity: &T) -> IOResult<PublishAck> {
    let msg = Self::serialize(entity)?;
    let res = self.get_natsjs().publish(self.get_subject(), msg);
    return res;
  }

  async fn queue_subscribe(
    &self,
    queue_name: &str,
  ) -> IOResult<UnboundedReceiverStream<(T, Message)>> {
    let con = self.get_natsjs();
    let consumer = self.add_consumer(queue_name.to_string()).await?;
    let options = PullSubscribeOptions::new()
      .bind_stream(self.get_subject().into())
      .consumer_config(consumer.config);
    let sub = con.pull_subscribe_with_options(self.get_subject(), &options)?;
    let (sender, recv) = unbounded_channel();
    thread::spawn(move || {
      while let Some(msg) = sub.next() {
        let _ = msg.ack();
        let obj = from_msgpack::<T>(&msg.data).map(|obj| (obj, msg));
        if let Err(ref e) = obj {
          println!("Msg deserialization failure: {:?}", e);
        }
        let _ = sender.send(obj.unwrap());
      }
    });
    return Ok(UnboundedReceiverStream::new(recv));
  }
}

use ::std::io::Result as IOResult;
use ::std::io::{Error as IOError, ErrorKind as IOErrKind};
use ::std::thread;

use ::nats::jetstream::SubscribeOptions;
use ::nats::jetstream::{JetStream as NatsJS, PublishAck};
use ::nats::Message;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde::de::DeserializeOwned;
use ::serde::ser::Serialize;
use ::tokio::sync::mpsc::unbounded_channel;
use ::tokio_stream::wrappers::UnboundedReceiverStream;

pub trait PubSub<T>
where
  T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
{
  fn get_subject(&self) -> &str;
  fn get_natsjs(&self) -> &NatsJS;

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

  fn queue_subscribe(
    &self,
    queue_name: &str,
  ) -> IOResult<UnboundedReceiverStream<(T, Message)>> {
    let con = self.get_natsjs();
    let options = SubscribeOptions::bind_stream(self.get_subject().into());
    let sub = con.queue_subscribe_with_options(
      self.get_subject(),
      queue_name,
      &options,
    )?;
    let (sender, recv) = unbounded_channel();
    thread::spawn(move || {
      while let Some(msg) = sub.next() {
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

use ::std::io::Result as IOResult;
use ::std::io::{Error as IOError, ErrorKind as IOErrKind};
use ::std::thread;

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

  fn subscribe(&self) -> IOResult<UnboundedReceiverStream<(T, Message)>> {
    let con = self.get_natsjs();
    // con.add_consumer("midas", self.get_subject())?;
    let sub = con.pull_subscribe(self.get_subject())?;
    let (sender, recv) = unbounded_channel();
    thread::spawn(move || loop {
      let msgs = match sub.fetch(1) {
        Ok(msgs) => msgs.filter_map(|msg| {
          let obj = from_msgpack::<T>(&msg.data).map(|obj| (obj, msg));
          if let Err(ref e) = obj {
            println!("Msg deserialization failure: {:?}", e);
          }
          return obj.ok();
        }),
        Err(e) => {
          println!("Fetch messages failure: {:?}", e);
          continue;
        }
      };
      for msg in msgs {
        let _ = sender.send(msg);
      }
    });
    return Ok(UnboundedReceiverStream::new(recv));
  }
}

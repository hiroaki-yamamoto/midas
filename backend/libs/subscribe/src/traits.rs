use ::std::io::Result as IOResult;
use ::std::io::{Error as IOError, ErrorKind as IOErrKind};

use ::nats::jetstream::{JetStream as NatsJS, PublishAck};
use ::rmp_serde::to_vec as to_msgpack;
use ::serde::de::DeserializeOwned;
use ::serde::ser::Serialize;

use super::streams::{JSSubs, Sub};

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

  fn subscribe(&self) -> IOResult<Sub<T>> {
    let con = self.get_natsjs();
    let sub = con.pull_subscribe(self.get_subject())?;
    let sub = Sub::new(JSSubs::Pull(sub));
    return Ok(sub);
  }

  fn queue_subscribe(&self, queue_name: &str) -> IOResult<Sub<T>> {
    let con = self.get_natsjs();
    let sub = con.queue_subscribe(self.get_subject(), queue_name)?;
    let sub = Sub::new(JSSubs::Push(sub));
    return Ok(sub);
  }
}

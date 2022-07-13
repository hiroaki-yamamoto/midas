use ::std::io::Result as IOResult;
use ::std::io::{Error as IOError, ErrorKind as IOErrKind};
use ::std::time::Duration;

use ::nats::{Connection as NatsCon, Message};
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde::de::DeserializeOwned;
use ::serde::ser::Serialize;

use super::streams::Sub;

pub trait PubSub<T>
where
  T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
{
  fn get_subject(&self) -> &str;
  fn get_broker(&self) -> &NatsCon;

  fn serialize<S>(entity: &S) -> IOResult<Vec<u8>>
  where
    S: Serialize,
  {
    return to_msgpack(entity).map_err(|e| IOError::new(IOErrKind::Other, e));
  }

  fn publish(&self, entity: &T) -> IOResult<()> {
    let msg = Self::serialize(entity)?;
    let res = self.get_broker().publish(self.get_subject(), msg);
    return res;
  }

  fn subscribe(&self) -> IOResult<Sub<T>> {
    let con = self.get_broker();
    let sub = con.subscribe(self.get_subject())?;
    let sub = Sub::new(sub);
    return Ok(sub);
  }

  fn queue_subscribe(&self, queue_name: &str) -> IOResult<Sub<T>> {
    let con = self.get_broker();
    let sub = con.queue_subscribe(self.get_subject(), queue_name)?;
    let sub = Sub::new(sub);
    return Ok(sub);
  }

  fn request<S>(&self, entity: &T) -> IOResult<(S, Message)>
  where
    S: DeserializeOwned + Send + Sync,
  {
    let msg = Self::serialize(entity)?;
    let res = self.get_broker().request_timeout(
      self.get_subject(),
      msg,
      Duration::from_secs(5),
    )?;
    let des = from_msgpack::<S>(&res.data)
      .map_err(|e| IOError::new(IOErrKind::Other, e))?;
    return Ok((des, res));
  }

  fn respond<S>(&self, msg: &Message, resp: &S) -> IOResult<()>
  where
    S: Serialize,
  {
    let resp = Self::serialize(resp)?;
    return msg.respond(resp);
  }
}

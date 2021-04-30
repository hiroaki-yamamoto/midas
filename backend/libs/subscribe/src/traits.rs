use ::std::io::Result as IOResult;
use ::std::io::{Error as IOError, ErrorKind as IOErrKind};

use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::nats::subscription::Handler;
use ::nats::{Connection as NatsCon, Message};
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde::de::DeserializeOwned;
use ::serde::ser::Serialize;

use super::streams::to_stream;

pub trait PubSub<T>
where
  T: DeserializeOwned + Serialize + Send + Sync + 'static,
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
    return self.get_broker().publish(self.get_subject(), msg);
  }

  fn subscribe(&self) -> IOResult<(Handler, BoxStream<(T, Message)>)> {
    let sub = self.get_broker().subscribe(self.get_subject())?;
    let (handler, st) = to_stream::<T>(sub);
    return Ok((handler, st.boxed()));
  }

  fn queue_subscribe(
    &self,
    queue_name: &str,
  ) -> IOResult<(Handler, BoxStream<(T, Message)>)> {
    let sub = self
      .get_broker()
      .queue_subscribe(self.get_subject(), queue_name)?;
    let (handler, st) = to_stream::<T>(sub);
    return Ok((handler, st.boxed()));
  }

  fn request<S>(&self, entity: &T) -> IOResult<(S, Message)>
  where
    S: DeserializeOwned + Send + Sync,
  {
    let msg = Self::serialize(entity)?;
    let res = self.get_broker().request(self.get_subject(), msg)?;
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

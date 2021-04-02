use ::std::io::Result as IOResult;
use ::std::io::{Error as IOError, ErrorKind};

use ::nats::subscription::{Handler, Subscription};
use ::nats::Message;
use ::rmp_serde::from_slice as from_msgpack;
use ::serde::de::DeserializeOwned;

pub fn handle<F, T>(sub: Subscription, func: F) -> Handler
where
  F: Fn(T, Message) -> IOResult<()> + Send + 'static,
  T: DeserializeOwned,
{
  return sub.with_handler(move |msg| {
    let obj = from_msgpack::<T>(&msg.data)
      .map_err(|e| IOError::new(ErrorKind::Other, e))?;
    return func(obj, msg);
  });
}

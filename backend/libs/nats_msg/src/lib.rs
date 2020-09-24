use ::std::result::Result;

use ::rmp_serde::decode::Error as MsgpackErr;
use ::rmp_serde::from_slice as from_msgpack;
use ::serde::Deserialize;
use ::tokio::stream::{Stream, StreamExt};

use ::nats::asynk::Subscription;

pub fn from_nats<'t, T>(
  sub: Subscription,
) -> impl Stream<Item = Result<T, MsgpackErr>>
where
  T: Deserialize<'t>,
{
  return sub.map(|msg| {
    let msg = msg.data.clone();
    let msg = &msg[..];
    let msg = from_msgpack::<T>(msg.clone());
    return msg;
  });
}

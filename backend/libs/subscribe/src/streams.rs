use ::futures::{stream::iter, Stream, StreamExt};
use ::nats::{Message, Subscription};
use ::rmp_serde::from_slice as from_msgpack;
use ::serde::de::DeserializeOwned;

pub fn to_stream_raw(
  sub: Subscription,
) -> impl Stream<Item = Message> + Send + Sync {
  return iter(sub);
}

pub fn to_stream<T>(
  sub: Subscription,
) -> impl Stream<Item = (T, Message)> + Send + Sync
where
  T: DeserializeOwned + Send + Sync,
{
  let stream = to_stream_raw(sub);
  let stream = stream
    .map(|msg| from_msgpack::<T>(&msg.data).map(|d| (d, msg)))
    .filter_map(|res| async { res.ok() });
  return stream;
}

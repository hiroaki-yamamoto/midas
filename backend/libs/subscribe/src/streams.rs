use ::std::io::{Error as IOError, ErrorKind};

use ::futures::{Stream, StreamExt};
use ::nats::{Message, Subscription as NatsSub};
use ::rmp_serde::from_slice as from_msgpack;
use ::serde::de::DeserializeOwned;

use ::tokio::sync::mpsc::unbounded_channel;
use ::tokio_stream::wrappers::UnboundedReceiverStream;

pub fn to_stream_raw(
  sub: NatsSub,
) -> impl Stream<Item = Message> + Send + Sync {
  let (sender, receiver) = unbounded_channel();
  let _ = sub.with_handler(move |msg| {
    return sender
      .send(msg)
      .map_err(|e| IOError::new(ErrorKind::Other, e));
  });
  let stream = UnboundedReceiverStream::new(receiver);
  return stream;
}

pub fn to_stream<T>(
  sub: NatsSub,
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

use ::futures::task::Poll;
use ::futures::{Stream, StreamExt};
use ::nats::{Message, Subscription as NatsSub};
use ::rmp_serde::from_slice as from_msgpack;
use ::serde::de::DeserializeOwned;
use ::std::time::Duration;

const POLL_TIMEOUT: Duration = Duration::from_micros(1);

pub struct Sub {
  sub: NatsSub,
}

impl Sub {
  pub fn new(sub: NatsSub) -> Self {
    return Self { sub };
  }
}

impl Stream for Sub {
  type Item = Message;
  fn poll_next(
    self: std::pin::Pin<&mut Self>,
    _: &mut std::task::Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    return match self.sub.next_timeout(POLL_TIMEOUT) {
      Ok(v) => Poll::Ready(Some(v)),
      Err(_) => Poll::Pending,
    };
  }
}

pub fn to_stream_raw(
  sub: NatsSub,
) -> impl Stream<Item = Message> + Send + Sync {
  return Sub::new(sub);
}

pub fn to_stream<T>(
  sub: NatsSub,
) -> impl Stream<Item = (T, Message)> + Send + Sync
where
  T: DeserializeOwned + Send + Sync,
{
  return to_stream_raw(sub)
    .map(|msg| from_msgpack::<T>(&msg.data).map(|d| (d, msg)))
    .filter_map(|res| async { res.ok() });
}

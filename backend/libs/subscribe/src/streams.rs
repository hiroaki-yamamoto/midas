use ::futures::task::Poll;
use ::futures::Stream;
use ::nats::{Connection as NatsCon, Message, Subscription as NatsSub};
use ::rmp_serde::from_slice as from_msgpack;
use ::serde::de::DeserializeOwned;
use ::std::marker::PhantomData;
use ::std::time::Duration;

const POLL_TIMEOUT: Duration = Duration::from_micros(1);

#[derive(Debug)]
pub struct Sub<'a, T>
where
  T: DeserializeOwned,
{
  con: &'a NatsCon,
  sub: NatsSub,
  p: PhantomData<T>,
}

impl<'a, T> Sub<'a, T>
where
  T: DeserializeOwned,
{
  pub fn new(con: &'a NatsCon, sub: NatsSub) -> Self {
    return Self {
      con,
      sub,
      p: PhantomData,
    };
  }
}

impl<'a, T> Stream for Sub<'a, T>
where
  T: DeserializeOwned,
{
  type Item = (T, Message);
  fn poll_next(
    self: std::pin::Pin<&mut Self>,
    _: &mut std::task::Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    if let Err(e) = self.con.flush() {
      println!("Nats Flushing Failure: {:?}", e);
      return Poll::Ready(None);
    }
    return self
      .sub
      .next_timeout(POLL_TIMEOUT)
      .ok()
      .map(|msg| from_msgpack::<T>(&msg.data).map(|obj| (obj, msg)).ok())
      .flatten()
      .map(|tup| Poll::Ready(Some(tup)))
      .unwrap_or(Poll::Pending);
  }
}

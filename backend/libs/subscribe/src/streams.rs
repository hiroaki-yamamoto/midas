use ::std::sync::{Arc, Mutex};
use ::std::task::Waker;
use ::std::thread;

use ::futures::task::Poll;
use ::futures::Stream;
use ::nats::jetstream::PushSubscription as NatsPushSub;
use ::nats::Message;
use ::rmp_serde::from_slice as from_msgpack;
use ::serde::de::DeserializeOwned;

pub struct Sub<T>
where
  T: DeserializeOwned + Clone + Send + Sync,
{
  state: Arc<Mutex<State<T>>>,
}

struct State<T>
where
  T: DeserializeOwned + Clone + Send + Sync,
{
  cur: Option<(T, Message)>,
  waker: Option<Waker>,
}

impl<T> Sub<T>
where
  T: DeserializeOwned + Clone + Send + Sync + 'static,
{
  pub fn new(sub: NatsPushSub) -> Self {
    let state = Arc::new(Mutex::new(State {
      waker: None,
      cur: None,
    }));
    let threaded_ctx = state.clone();
    thread::spawn(move || loop {
      let msg = sub
        .next()
        .map(|msg| {
          let obj = from_msgpack::<T>(&msg.data).map(|obj| (obj, msg));
          if let Err(ref e) = obj {
            println!("Msg deserialization failure: {:?}", e);
          }
          return obj.ok();
        })
        .flatten();
      if let Some(msg) = msg {
        let mut ctx = threaded_ctx.lock().unwrap();
        ctx.cur = Some(msg);
        if let Some(waker) = ctx.waker.take() {
          waker.wake();
        }
      }
    });
    return Self { state };
  }
}

impl<T> Stream for Sub<T>
where
  T: DeserializeOwned + Clone + Send + Sync,
{
  type Item = (T, Message);
  fn poll_next(
    self: std::pin::Pin<&mut Self>,
    ctx: &mut std::task::Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let mut state = self.state.lock().unwrap();
    state.waker = Some(ctx.waker().clone());
    if state.cur.is_none() {
      return Poll::Pending;
    }
    let poll = Poll::Ready(state.cur.clone());
    state.cur = None;
    return poll;
  }
}

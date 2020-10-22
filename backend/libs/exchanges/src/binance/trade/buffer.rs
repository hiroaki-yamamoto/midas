use ::std::error::Error;
use ::std::pin::Pin;

use ::async_stream::stream;
use ::futures::future::join_all;
use ::futures::{select, Stream, StreamExt};
use ::nats::asynk::Connection as NatsCon;
use ::rmp_serde::to_vec as to_msgpack;
use ::std::time::Duration;

use ::types::GenericResult;

use super::super::entities::Symbol;

type SymbolSubscription = Pin<Box<dyn Stream<Item = Symbol>>>;

const SUB_NAME: &str = "binance.trades.init";
const NUM_QUERY: usize = 10;
const flush_interval: Duration = Duration::from_secs(1);

struct ObserverBuffer<'t> {
  broker: &'t NatsCon,
  buf: Vec<Symbol>,
}

impl<'t> ObserverBuffer<'t> {
  pub fn new(broker: &'t NatsCon) -> Self {
    return Self {
      buf: vec![],
      broker,
    };
  }

  pub async fn publish(&self) -> GenericResult<()> {
    let mut pub_fut = vec![];
    for symbol in &self.buf {
      pub_fut.push(self.broker.publish(SUB_NAME, to_msgpack(&symbol)?));
    }
    let first_err = join_all(pub_fut)
      .await
      .into_iter()
      .filter_map(|f| f.err())
      .nth(0)
      .map(|err| Box::new(err));
    if let Some(err) = first_err {
      return Err(err);
    }
    return Ok(());
  }

  // pub async fn subscribe(&mut self) -> GenericResult<SymbolSubscription> {
  //   let sub = self
  //     .broker
  //     .queue_subscribe(SUB_NAME, "observer_buffer")
  //     .await?;
  //   let stream = stream! {
  //     loop {
  //       select! {
  //         Some(item) = sub.next() => {
  //           self.buf.push(item);
  //         },
  //       }
  //     };
  //   };
  //   return stream;
  // }
}

impl<'t> Extend<Symbol> for ObserverBuffer<'t> {
  fn extend<T: IntoIterator<Item = Symbol>>(&mut self, iter: T) {
    self.buf.extend(iter);
  }
}

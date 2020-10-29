use ::std::pin::Pin;

use super::super::history_recorder::HistoryRecorder;
use ::futures::stream::{Stream, StreamExt};
use ::futures::task::{Context, Poll};

use ::types::GenericResult;

use super::super::entities::Kline;

pub struct Executor {
  spread: f64,
  maker_fee: f64,
  taker_fee: f64,
  cur_trade: Option<Kline>,
  kline_stream: Pin<Box<dyn Stream<Item = Kline>>>,
}

impl Executor {
  pub async fn new(
    history_recorder: HistoryRecorder,
    spread: f64,
    maker_fee: f64,
    taker_fee: f64,
  ) -> GenericResult<Self> {
    return Ok(Self {
      spread,
      maker_fee,
      taker_fee,
      cur_trade: None,
      kline_stream: history_recorder.list(None).await?.boxed(),
    });
  }
}

impl Stream for Executor {
  type Item = Kline;
  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let ret = self.kline_stream.poll_next_unpin(cx);
    if let Poll::Ready(d) = &ret {
      self.cur_trade = d.to_owned();
    }
    return ret;
  }
}

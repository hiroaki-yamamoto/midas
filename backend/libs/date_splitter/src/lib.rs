use ::std::time::{SystemTime, SystemTimeError};
use std::task::Poll;
use std::time::Duration;

use ::futures::Stream;

#[derive(Debug, Clone)]
pub struct DateSplitter {
  start: SystemTime,
  end: SystemTime,
  cur: SystemTime,
  interval: Duration,
}

impl DateSplitter {
  pub fn new(
    start: SystemTime,
    end: SystemTime,
    interval: Duration,
  ) -> Result<Self, SystemTimeError> {
    return Ok(Self {
      start: start,
      cur: start,
      end: end,
      interval,
    });
  }
  pub fn len(&self) -> Result<usize, SystemTimeError> {
    let diff = self.end.duration_since(self.start)?.as_nanos();
    let interval = self.interval.as_nanos();
    let ret = diff / interval;
    let rem = diff % interval;
    if rem == 0 {
      return Ok(ret as usize);
    }
    return Ok((ret + 1) as usize);
  }
  fn predict_next(&self) -> SystemTime {
    if self.cur + self.interval <= self.end {
      return self.cur + self.interval;
    }
    return self.end;
  }
}

impl Stream for DateSplitter {
  type Item = (SystemTime, SystemTime);

  fn poll_next(
    mut self: std::pin::Pin<&mut Self>,
    _: &mut std::task::Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let ret = self.cur.clone();
    self.cur = self.predict_next();

    if ret >= self.end {
      return Poll::Ready(None);
    } else {
      return Poll::Ready(Some((ret, self.cur)));
    }
  }
}

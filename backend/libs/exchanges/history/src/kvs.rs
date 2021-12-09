use std::fmt::Display;

use ::redis::Commands;
use ::redis::RedisResult;

pub struct CurrentSyncProgressStore<T>
where
  T: Commands,
{
  con: T,
}

impl<T> CurrentSyncProgressStore<T>
where
  T: Commands,
{
  pub fn new(con: T) -> Self {
    return Self { con };
  }

  pub fn channel_name<E, S>(&self, exchange: E, symbol: S) -> String
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    return format!("{}.{}.kline_sync.current", exchange, symbol);
  }

  pub fn incr<E, S>(
    &mut self,
    exchange: E,
    symbol: S,
    delta: i32,
  ) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    return self.con.incr(self.channel_name(exchange, symbol), delta);
  }
}

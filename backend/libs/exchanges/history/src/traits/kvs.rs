use ::std::fmt::Display;

use ::redis::Commands;
use ::redis::RedisResult;

pub trait Store<T>
where
  T: Commands,
{
  fn commands(&mut self) -> &mut T;
  fn channel_name<E, S>(&self, exchange: E, symbol: S) -> String
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display;

  fn reset<E, S>(&mut self, exchange: E, symbol: S) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(exchange, symbol);
    return self.commands().del(channel_name);
  }

  fn set<E, S>(&mut self, exchange: E, symbol: S, value: i32) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(exchange, symbol);
    return self.commands().set(channel_name, value);
  }

  fn incr<E, S>(
    &mut self,
    exchange: E,
    symbol: S,
    delta: i32,
  ) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(exchange, symbol);
    return self.commands().incr(channel_name, delta);
  }
}

use ::std::fmt::Display;

use ::redis::{Commands, FromRedisValue, RedisResult, ToRedisArgs};

pub trait Store<T, V>
where
  T: Commands,
  V: FromRedisValue + ToRedisArgs,
{
  fn commands(&mut self) -> &mut T;
  fn channel_name<E, S>(&self, exchange: E, symbol: S) -> String
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display;

  fn del<E, S>(&mut self, exchange: E, symbol: S) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(exchange, symbol);
    return self.commands().del(channel_name);
  }

  fn get<E, S>(&mut self, exchange: E, symbol: S) -> RedisResult<V>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(exchange, symbol);
    return self.commands().get(channel_name);
  }

  fn set<E, S>(&mut self, exchange: E, symbol: S, value: V) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(exchange, symbol);
    return self.commands().set(channel_name, value);
  }
}

pub trait IncrementalStore<T>: Store<T, i64>
where
  T: Commands,
{
  fn incr<E, S>(
    &mut self,
    exchange: E,
    symbol: S,
    delta: i64,
  ) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(exchange, symbol);
    return self.commands().incr(channel_name, delta);
  }

  fn reset<E, S>(&mut self, exchange: E, symbol: S) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(exchange, symbol);
    return self.commands().set(channel_name, 0);
  }
}

use ::std::fmt::Display;

use ::redis::{Commands, FromRedisValue, RedisResult, ToRedisArgs};

use super::options::{WriteOption, WriteOptionTrait};

pub trait Store<T, V>
where
  T: Commands,
  V: FromRedisValue + ToRedisArgs,
{
  fn commands(&mut self) -> &mut T;
  fn channel_name<S>(&self, key: S) -> String
  where
    S: AsRef<str> + Display;

  fn del<S>(&mut self, key: S) -> RedisResult<()>
  where
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(key);
    return self.commands().del(channel_name);
  }
  fn get<S>(&mut self, key: S) -> RedisResult<V>
  where
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(key);
    return self.commands().get(channel_name);
  }

  fn set<E, S>(
    &mut self,
    key: S,
    value: V,
    opt: Option<WriteOption>,
  ) -> RedisResult<()>
  where
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(key);
    let cmds = self.commands();
    let result = cmds
      .set(&channel_name, value)
      .and_then(|_: ()| opt.execute(cmds, &channel_name));
    return result;
  }
}

pub trait SymbolKeyStore<T, V>
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
    let channel_name = SymbolKeyStore::channel_name(self, exchange, symbol);
    return self.commands().del(channel_name);
  }

  fn get<E, S>(&mut self, exchange: E, symbol: S) -> RedisResult<V>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = SymbolKeyStore::channel_name(self, exchange, symbol);
    return self.commands().get(channel_name);
  }

  fn set<E, S>(
    &mut self,
    exchange: E,
    symbol: S,
    value: V,
    opt: Option<WriteOption>,
  ) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = SymbolKeyStore::channel_name(self, exchange, symbol);
    let cmds = self.commands();
    let result = cmds
      .set(&channel_name, value)
      .and_then(|_: ()| opt.execute(cmds, &channel_name));
    return result;
  }
}

pub trait IncrementalStore<T>: SymbolKeyStore<T, i64>
where
  T: Commands,
{
  fn incr<E, S>(
    &mut self,
    exchange: E,
    symbol: S,
    delta: i64,
    opt: Option<WriteOption>,
  ) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name =
      SymbolKeyStore::<T, i64>::channel_name(self, exchange, symbol);
    let cmds = self.commands();
    return cmds
      .incr(&channel_name, delta)
      .and_then(|_: ()| opt.execute(cmds, &channel_name));
  }

  fn reset<E, S>(&mut self, exchange: E, symbol: S) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name =
      SymbolKeyStore::<T, i64>::channel_name(self, exchange, symbol);
    return self.commands().set(channel_name, 0);
  }
}

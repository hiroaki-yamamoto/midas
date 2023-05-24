use ::std::fmt::Display;
use ::std::time::Duration;

use ::redis::{Commands, FromRedisValue, RedisResult, ToRedisArgs};

#[derive(Default, Clone)]
pub struct WriteOption {
  pub dur: Option<Duration>,
}

impl WriteOption {
  pub fn duration(&self, dur: Option<Duration>) -> Self {
    let mut me = self.clone();
    me.dur = dur;
    return me;
  }
}

fn execute_set_option<T, S>(
  cmds: &mut T,
  key: S,
  opt: Option<WriteOption>,
) -> RedisResult<()>
where
  T: Commands,
  S: AsRef<str> + Display,
{
  let key = key.as_ref();
  let mut res: RedisResult<()> = Ok(());
  if let Some(opt) = opt {
    if let Some(dur) = opt.dur {
      res = res.and(cmds.pexpire(key, dur.as_millis() as usize));
    }
  }
  return res;
}

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
    let channel_name = self.channel_name(exchange, symbol);
    let cmds = self.commands();
    let result = cmds
      .set(&channel_name, value)
      .and_then(|_: ()| execute_set_option(cmds, channel_name, opt));
    return result;
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
    opt: Option<WriteOption>,
  ) -> RedisResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(exchange, symbol);
    let cmds = self.commands();
    return cmds
      .incr(&channel_name, delta)
      .and_then(|_: ()| execute_set_option(cmds, &channel_name, opt));
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

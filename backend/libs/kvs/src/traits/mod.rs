pub mod last_checked;
pub mod normal;

use ::std::fmt::Display;
use ::std::num::NonZeroUsize;
use ::std::sync::MutexGuard;

use ::redis::{Commands, FromRedisValue, SetOptions, ToRedisArgs};

use super::options::{WriteOption, WriteOptionTrait};
use ::errors::KVSResult;

pub trait SymbolKeyStore<T, V>
where
  T: Commands,
  V: FromRedisValue + ToRedisArgs,
{
  fn lock_commands(&self) -> MutexGuard<T>;

  fn channel_name(
    &self,
    exchange: impl AsRef<str> + Display,
    symbol: impl AsRef<str> + Display,
  ) -> String;

  fn del<E, S>(&mut self, exchange: E, symbol: S) -> KVSResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = SymbolKeyStore::channel_name(self, exchange, symbol);
    return Ok(self.lock_commands().del(channel_name)?);
  }

  fn get<E, S>(&mut self, exchange: E, symbol: S) -> KVSResult<V>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name = SymbolKeyStore::channel_name(self, exchange, symbol);
    return Ok(self.lock_commands().get(channel_name)?);
  }

  fn set<R>(
    &mut self,
    exchange: impl AsRef<str> + Display,
    symbol: impl AsRef<str> + Display,
    value: V,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = SymbolKeyStore::channel_name(self, exchange, symbol);
    let mut cmds = self.lock_commands();
    let result = if let Some(opt) = opt {
      let opt: SetOptions = opt.into();
      cmds.set_options(&channel_name, value, opt)
    } else {
      cmds.set(&channel_name, value)
    };
    return Ok(result?);
  }
}

pub trait IncrementalStore<T>: SymbolKeyStore<T, i64>
where
  T: Commands,
{
  fn incr(
    &mut self,
    exchange: impl AsRef<str> + Display,
    symbol: impl AsRef<str> + Display,
    delta: i64,
    opt: Option<WriteOption>,
  ) -> KVSResult<()> {
    let channel_name =
      SymbolKeyStore::<T, i64>::channel_name(self, exchange, symbol);
    let mut cmds = self.lock_commands();
    return Ok(cmds.incr(&channel_name, delta).and_then(|_: ()| {
      return opt.execute(&mut cmds, &channel_name);
    })?);
  }

  fn reset(
    &mut self,
    exchange: impl AsRef<str> + Display,
    symbol: impl AsRef<str> + Display,
  ) -> KVSResult<()> {
    let channel_name =
      SymbolKeyStore::<T, i64>::channel_name(self, exchange, symbol);
    return Ok(self.lock_commands().set(channel_name, 0)?);
  }
}

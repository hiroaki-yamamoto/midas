mod last_checked;
mod normal;

use ::std::fmt::Display;
use ::std::num::NonZeroUsize;
use ::std::sync::MutexGuard;
use ::std::time::{Duration, SystemTime};

use ::redis::{Commands, FromRedisValue, SetOptions, ToRedisArgs};

use ::errors::KVSResult;

pub use self::normal::{
  NormalStoreBase, NormalStoreExist, NormalStoreExpiration, NormalStoreGet,
  NormalStoreListOp, NormalStoreLock, NormalStoreRemove, NormalStoreSet, Store,
};
use super::options::{WriteOption, WriteOptionTrait};

pub trait SoftExpirationStore<T, V>: Store<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  fn set<R>(
    &mut self,
    key: impl AsRef<str> + Display,
    value: V,
    opt: impl Into<Option<WriteOption>> + Clone,
    last_checked: &mut impl Store<T, u64>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let ret = Store::set(self, &key, value, opt.clone())?;
    self.flag_last_checked(key, last_checked, opt.into())?;
    return Ok(ret);
  }

  fn expire(
    &mut self,
    key: &str,
    dur: Duration,
    last_checked: &mut impl Store<T, u64>,
  ) -> KVSResult<bool> {
    let ret = Store::expire(self, key, dur)?;
    self.flag_last_checked(
      key,
      last_checked,
      WriteOption::default().duration(dur.into()).into(),
    )?;
    return Ok(ret);
  }

  fn del(
    &mut self,
    key: &(impl AsRef<str> + Display + Send + Sync),
    last_checked: &mut impl Store<T, u64>,
  ) -> KVSResult<()> {
    let ret = Store::del(self, key);
    let _ = last_checked.del(key)?;
    return ret;
  }

  fn lpush<R>(
    &mut self,
    key: impl AsRef<str> + Display,
    value: V,
    opt: Option<WriteOption>,
    last_checked: &mut impl Store<T, u64>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let ret = Store::lpush(self, &key, value, opt.clone())?;
    self.flag_last_checked(key, last_checked, opt.into())?;
    return Ok(ret);
  }

  fn lpop<R>(
    &mut self,
    key: impl AsRef<str> + Display,
    count: Option<NonZeroUsize>,
    last_checked: &mut impl Store<T, u64>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let ret = Store::lpop(self, &key, count)?;
    self.flag_last_checked(key, last_checked, None)?;
    return Ok(ret);
  }
}

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

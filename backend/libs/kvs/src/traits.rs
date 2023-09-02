use ::std::fmt::Display;
use ::std::num::NonZeroUsize;
use ::std::sync::MutexGuard;
use ::std::time::{Duration, SystemTime};

use ::errors::KVSResult;
use ::redis::{Commands, FromRedisValue, SetOptions, ToRedisArgs};

use super::options::{WriteOption, WriteOptionTrait};

pub trait Store<T, V>
where
  T: Commands,
  V: FromRedisValue + ToRedisArgs,
{
  fn lock_commands(&self) -> MutexGuard<T>;
  fn channel_name(&self, key: impl AsRef<str> + Display) -> String;

  fn del(&mut self, key: impl AsRef<str> + Display) -> KVSResult<()> {
    let channel_name = self.channel_name(key);
    return Ok(self.lock_commands().del(channel_name)?);
  }

  fn get(&mut self, key: impl AsRef<str> + Display) -> KVSResult<V> {
    let channel_name = self.channel_name(key);
    return Ok(self.lock_commands().get(channel_name)?);
  }

  fn set<R>(
    &mut self,
    key: impl AsRef<str> + Display,
    value: V,
    opt: impl Into<Option<WriteOption>>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    let mut cmds = self.lock_commands();
    let result = if let Some(opt) = opt.into() {
      let opt: SetOptions = opt.into();
      cmds.set_options(&channel_name, value, opt)
    } else {
      cmds.set(&channel_name, value)
    };
    return Ok(result?);
  }

  fn expire(
    &mut self,
    key: impl AsRef<str> + Display,
    dur: Duration,
  ) -> KVSResult<bool>
  where
    V: FromRedisValue,
  {
    let dur_mils = dur.as_millis() as usize;
    let channel_name = self.channel_name(key);
    if self
      .lock_commands()
      .pexpire::<_, u16>(channel_name, dur_mils)?
      == 1
    {
      return Ok(true);
    } else {
      return Ok(false);
    };
  }

  fn lpush<R>(
    &mut self,
    key: impl AsRef<str> + Display,
    value: V,
    opt: impl Into<Option<WriteOption>>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    let mut cmds = self.lock_commands();
    let opt: Option<WriteOption> = opt.into();
    let result = if opt.non_existent_only() {
      cmds.lpush_exists(&channel_name, value)?
    } else {
      cmds.lpush(&channel_name, value)?
    };

    opt.execute(&mut cmds, &channel_name)?;
    return Ok(result);
  }

  fn lpop<R>(
    &mut self,
    key: impl AsRef<str> + Display,
    count: Option<NonZeroUsize>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    return Ok(self.lock_commands().lpop(channel_name, count)?);
  }

  fn lrange<R>(
    &mut self,
    key: impl AsRef<str> + Display,
    start: isize,
    stop: isize,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    return Ok(self.lock_commands().lrange(channel_name, start, stop)?);
  }
}

pub trait SoftExpirationStore<T, V>: Store<T, V>
where
  T: Commands,
  V: FromRedisValue + ToRedisArgs,
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

  fn flag_last_checked<R>(
    &mut self,
    key: impl AsRef<str> + Display,
    last_checked: &mut impl Store<T, u64>,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_secs();
    return Ok(last_checked.set(key, now, opt)?);
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

use ::std::fmt::Display;
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
  fn channel_name<S>(&self, key: S) -> String
  where
    S: AsRef<str> + Display;

  fn del<S>(&mut self, key: S) -> KVSResult<()>
  where
    S: AsRef<str> + Display + Send + Sync,
  {
    let channel_name = self.channel_name(key);
    return Ok(self.lock_commands().del(channel_name)?);
  }
  fn get<S>(&mut self, key: S) -> KVSResult<V>
  where
    S: AsRef<str> + Display,
  {
    let channel_name = self.channel_name(key);
    return Ok(self.lock_commands().get(channel_name)?);
  }

  fn set<R, S>(
    &mut self,
    key: S,
    value: V,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    S: AsRef<str> + Display,
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    let mut cmds = self.lock_commands();
    let result = if let Some(opt) = opt {
      let opt: SetOptions = opt.into();
      cmds.set_options(&channel_name, value, opt)
    } else {
      cmds.set(&channel_name, value)
    };
    return Ok(result?);
  }

  fn expire(&mut self, key: &str, dur: Duration) -> KVSResult<bool>
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
}

pub trait SoftExpirationStore<T, V>: Store<T, V>
where
  T: Commands,
  V: FromRedisValue + ToRedisArgs,
{
  fn set<LC, R, S>(
    &mut self,
    key: S,
    value: V,
    opt: Option<WriteOption>,
    last_checked: &mut LC,
  ) -> KVSResult<R>
  where
    LC: Store<T, u64>,
    S: AsRef<str> + Display,
    R: FromRedisValue,
  {
    let ret = Store::set(self, &key, value, opt.clone())?;
    self.flag_last_checked(key, last_checked, opt.into())?;
    return Ok(ret);
  }

  fn flag_last_checked<LC, S, R>(
    &mut self,
    key: S,
    last_checked: &mut LC,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    S: AsRef<str> + Display,
    LC: Store<T, u64>,
    R: FromRedisValue,
  {
    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_secs();
    return Ok(last_checked.set(key, now, opt)?);
  }

  fn expire<LC>(
    &mut self,
    key: &str,
    dur: Duration,
    last_checked: &mut LC,
  ) -> KVSResult<bool>
  where
    V: FromRedisValue,
    LC: Store<T, u64>,
  {
    let ret = Store::expire(self, key, dur)?;
    self.flag_last_checked(
      key,
      last_checked,
      WriteOption::default().duration(dur.into()).into(),
    )?;
    return Ok(ret);
  }
}

pub trait SymbolKeyStore<T, V>
where
  T: Commands,
  V: FromRedisValue + ToRedisArgs,
{
  fn lock_commands(&self) -> MutexGuard<T>;

  fn channel_name<E, S>(&self, exchange: E, symbol: S) -> String
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display;

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

  fn set<E, R, S>(
    &mut self,
    exchange: E,
    symbol: S,
    value: V,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    E: AsRef<str> + Display,
    R: FromRedisValue,
    S: AsRef<str> + Display,
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
  fn incr<E, S>(
    &mut self,
    exchange: E,
    symbol: S,
    delta: i64,
    opt: Option<WriteOption>,
  ) -> KVSResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name =
      SymbolKeyStore::<T, i64>::channel_name(self, exchange, symbol);
    let mut cmds = self.lock_commands();
    return Ok(cmds.incr(&channel_name, delta).and_then(|_: ()| {
      return opt.execute(&mut cmds, &channel_name);
    })?);
  }

  fn reset<E, S>(&mut self, exchange: E, symbol: S) -> KVSResult<()>
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    let channel_name =
      SymbolKeyStore::<T, i64>::channel_name(self, exchange, symbol);
    return Ok(self.lock_commands().set(channel_name, 0)?);
  }
}

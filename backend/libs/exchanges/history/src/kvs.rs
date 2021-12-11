use std::fmt::Display;

use ::redis::Commands;

use super::traits::Store;

pub struct CurrentSyncProgressStore<'a, T>
where
  T: Commands,
{
  con: &'a mut T,
}

impl<'a, T> CurrentSyncProgressStore<'a, T>
where
  T: Commands,
{
  pub fn new(con: &'a mut T) -> Self {
    return Self { con };
  }
}

impl<'a, T> Store<T> for CurrentSyncProgressStore<'a, T>
where
  T: Commands,
{
  fn channel_name<E, S>(&self, exchange: E, symbol: S) -> String
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    return format!("{}.{}.kline_sync.current", exchange, symbol);
  }
  fn commands(&mut self) -> &mut T {
    return self.con;
  }
}

pub struct NumObjectsToFetchStore<'a, T>
where
  T: Commands,
{
  con: &'a mut T,
}

impl<'a, T> NumObjectsToFetchStore<'a, T>
where
  T: Commands,
{
  pub fn new(con: &'a mut T) -> Self {
    return Self { con };
  }
}

impl<'a, T> Store<T> for NumObjectsToFetchStore<'a, T>
where
  T: Commands,
{
  fn channel_name<E, S>(&self, exchange: E, symbol: S) -> String
  where
    E: AsRef<str> + Display,
    S: AsRef<str> + Display,
  {
    return format!("{}.{}.kline_sync.num", exchange, symbol);
  }
  fn commands(&mut self) -> &mut T {
    return self.con;
  }
}

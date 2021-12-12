use std::fmt::Display;

use ::redis::Commands;

use super::traits::Store;

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
    return Self { con: con };
  }
}

impl<T> Store<T> for CurrentSyncProgressStore<T>
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
    return &mut self.con;
  }
}

pub struct NumObjectsToFetchStore<T>
where
  T: Commands,
{
  con: T,
}

impl<T> NumObjectsToFetchStore<T>
where
  T: Commands,
{
  pub fn new(con: T) -> Self {
    return Self { con: con };
  }
}

impl<T> Store<T> for NumObjectsToFetchStore<T>
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
    return &mut self.con;
  }
}

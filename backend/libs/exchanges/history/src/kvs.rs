use std::fmt::Display;

pub use ::redis;
use ::redis::Commands;

use super::traits::Store;

#[derive(Debug)]
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

#[derive(Debug)]
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

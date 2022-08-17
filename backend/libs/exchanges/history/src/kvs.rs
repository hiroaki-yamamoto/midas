use std::fmt::Display;

pub use ::redis;
use ::redis::Commands;

use super::traits::{IncrementalStore, Store};

macro_rules! impl_kvs {
  ($name: ident, $vtype: ty, $ch_name: expr) => {
    #[derive(Debug)]
    pub struct $name<T>
    where
      T: Commands,
    {
      con: T,
    }

    impl<T> $name<T>
    where
      T: Commands,
    {
      pub fn new(con: T) -> Self {
        return Self { con: con };
      }
    }

    impl<T> Store<T, $vtype> for $name<T>
    where
      T: Commands,
    {
      fn channel_name<E, S>(&self, exchange: E, symbol: S) -> String
      where
        E: AsRef<str> + Display,
        S: AsRef<str> + Display,
      {
        return format!($ch_name, exchange, symbol);
      }
      fn commands(&mut self) -> &mut T {
        return &mut self.con;
      }
    }
  };
}

impl_kvs!(CurrentSyncProgressStore, i64, "{}.{}.kline_sync.current");
impl_kvs!(NumObjectsToFetchStore, i64, "{}.{}.kline_sync.num");

impl<T> IncrementalStore<T> for CurrentSyncProgressStore<T> where T: Commands {}
impl<T> IncrementalStore<T> for NumObjectsToFetchStore<T> where T: Commands {}

mod connection;
mod options;
mod traits;

pub use ::redis;

pub use crate::options::WriteOption;
pub use crate::traits::{
  IncrementalStore, SoftExpirationStore, Store, SymbolKeyStore,
};

pub use crate::connection::Connection;

pub use ::errors::{KVSError, KVSResult};

#[macro_export]
macro_rules! kvs {
    ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
        #[derive(Clone)]
        $acc struct $name<T>
        where
          T: ::kvs::redis::Commands,
        {
          con: ::kvs::Connection<T>,
        }

        impl<T> $name<T>
        where
          T: ::kvs::redis::Commands,
        {
          pub fn new(con: ::kvs::Connection<T>) -> Self {
            return Self { con: con };
          }
        }

        impl<T> ::kvs::Store<T, $vtype> for $name<T>
        where
          T: ::kvs::redis::Commands,
        {
          fn channel_name(
            &self,
            key: impl AsRef<str> + ::std::fmt::Display
          ) -> String where
          {
            return format!($ch_name, key);
          }
          fn lock_commands(&self) -> ::std::sync::MutexGuard<T> {
            return self.con.lock().unwrap();
          }
        }
    };
}

#[macro_export]
macro_rules! symbol_kvs {
  ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
    #[derive(Clone)]
    $acc struct $name<T>
    where
      T: ::kvs::redis::Commands,
    {
      con: ::kvs::Connection<T>,
    }

    impl<T> $name<T>
    where
      T: ::kvs::redis::Commands,
    {
      pub fn new(con: ::kvs::Connection<T>) -> Self {
        return Self { con: con };
      }
    }

    impl<T> ::kvs::SymbolKeyStore<T, $vtype> for $name<T>
    where
      T: ::kvs::redis::Commands,
    {
      fn lock_commands(&self) -> ::std::sync::MutexGuard<T> {
        return self.con.lock().unwrap();
      }
      fn channel_name(
        &self,
        exchange: impl AsRef<str> + ::std::fmt::Display,
        symbol: impl AsRef<str> + ::std::fmt::Display
      ) -> String
      {
        return format!($ch_name, exchange, symbol);
      }
    }
  };
}

#[macro_export]
macro_rules! incr_kvs {
  ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
    ::kvs::symbol_kvs!($acc, $name, $vtype, $ch_name);
    impl<T> ::kvs::IncrementalStore<T> for $name<T> where
      T: ::kvs::redis::Commands
    {
    }
  };
}

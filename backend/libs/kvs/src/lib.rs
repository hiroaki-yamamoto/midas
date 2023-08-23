mod options;
mod traits;

pub use ::redis;

pub use crate::options::WriteOption;
pub use crate::traits::{
  IncrementalStore, SoftExpirationStore, Store, SymbolKeyStore,
};

pub use ::errors::{KVSError, KVSResult};

#[macro_export]
macro_rules! kvs {
    ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
        #[derive(Debug)]
        $acc struct $name<T>
        where
          T: ::kvs::redis::Commands,
        {
          con: ::std::sync::Arc<::std::sync::Mutex<T>>,
        }

        impl<T> $name<T>
        where
          T: ::kvs::redis::Commands,
        {
          pub fn new(con: ::std::sync::Arc<::std::sync::Mutex<T>>) -> Self {
            return Self { con: con };
          }
        }

        impl<T> ::kvs::Store<T, $vtype> for $name<T>
        where
          T: ::kvs::redis::Commands,
        {
          fn channel_name< S>(&self, key: S) -> String where
            S: AsRef<str> + ::std::fmt::Display,
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
    #[derive(Debug)]
    $acc struct $name<T>
    where
      T: ::kvs::redis::Commands,
    {
      con: ::std::sync::Arc<::std::sync::Mutex<T>>,
    }

    impl<T> $name<T>
    where
      T: ::kvs::redis::Commands,
    {
      pub fn new(con: ::std::sync::Arc<::std::sync::Mutex<T>>) -> Self {
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
      fn channel_name<E, S>(&self, exchange: E, symbol: S) -> String
      where
        E: AsRef<str> + ::std::fmt::Display,
        S: AsRef<str> + ::std::fmt::Display,
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

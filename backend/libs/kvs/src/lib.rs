mod nats;
mod options;
mod traits;

pub use crate::nats::NatsKVS;
pub use crate::options::WriteOption;
pub use crate::traits::{
  ExchangeKeyStore, IncrementalStore, Store, SymbolKeyStore,
};
pub use ::redis;

#[macro_export]
macro_rules! kvs {
    ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
        #[derive(Debug)]
        $acc struct $name<T>
        where
          T: ::kvs::redis::Commands,
        {
          con: T,
        }

        impl<T> $name<T>
        where
          T: ::kvs::redis::Commands,
        {
          pub fn new(con: T) -> Self {
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
          fn commands(&mut self) -> &mut T {
            return &mut self.con;
          }
        }
    };
}

#[macro_export]
macro_rules! exchange_kvs {
  ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
    ::kvs::kvs!($acc, $name, $vtype, $ch_name);
    impl<T> ::kvs::ExchangeKeyStore<T, $vtype> for $name<T> where
      T: ::kvs::redis::Commands
    {
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
      con: T,
    }

    impl<T> $name<T>
    where
      T: ::kvs::redis::Commands,
    {
      pub fn new(con: T) -> Self {
        return Self { con: con };
      }
    }

    impl<T> ::kvs::SymbolKeyStore<T, $vtype> for $name<T>
    where
      T: ::kvs::redis::Commands,
    {
      fn commands(&mut self) -> &mut T {
        return &mut self.con;
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

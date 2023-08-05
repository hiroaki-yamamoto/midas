mod options;
mod traits;

pub use crate::options::WriteOption;
pub use crate::traits::{IncrementalStore, Store, SymbolKeyStore};

#[macro_export]
macro_rules! kvs {
    ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
        #[derive(Debug)]
        $acc struct $name<T>
        where
          T: ::redis::Commands,
        {
          con: T,
        }

        impl<T> $name<T>
        where
          T: ::redis::Commands,
        {
          pub fn new(con: T) -> Self {
            return Self { con: con };
          }
        }

        impl<T> ::kvs::Store<T, $vtype> for $name<T>
        where
          T: ::redis::Commands,
        {
          fn channel_name< S>(&self, key: S) -> String where
            S: AsRef<str> + Display,
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
macro_rules! symbol_kvs {
  ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
    #[derive(Debug)]
    $acc struct $name<T>
    where
      T: ::redis::Commands,
    {
      con: T,
    }

    impl<T> $name<T>
    where
      T: ::redis::Commands,
    {
      pub fn new(con: T) -> Self {
        return Self { con: con };
      }
    }

    impl<T> ::kvs::SymbolKeyStore<T, $vtype> for $name<T>
    where
      T: ::redis::Commands,
    {
      fn commands(&mut self) -> &mut T {
        return &mut self.con;
      }
      fn channel_name<E, S>(&self, exchange: E, symbol: S) -> String
      where
        E: AsRef<str> + Display,
        S: AsRef<str> + Display,
      {
        return format!($ch_name, exchange, symbol);
      }
    }
  };
}

#[macro_export]
macro_rules! kvs_incr {
  ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
    ::kvs::symbol_kvs!($acc, $name, $vtype, $ch_name);
    impl<T> ::kvs::IncrementalStore<T> for $name<T> where T: Commands {}
  };
}

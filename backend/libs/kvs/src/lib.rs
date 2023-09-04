mod connection;
mod options;
pub mod traits;

pub use ::redis;

pub use crate::options::WriteOption;
pub use crate::traits::symbol::Incr as IncrementalStore;

pub use crate::connection::Connection;

pub use ::errors::{KVSError, KVSResult};

#[macro_export]
macro_rules! kvs {
  ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
    macro_rules! impl_trait_1 {
      ($src: ty, $dst: ty) => {
        impl<T> $src<T> for $dst<T> {}
      };
    }
    #[derive(Clone)]
    $acc struct $name<T>
    where
      T: ::kvs::redis::Commands + Send + Sync,
    {
      con: ::kvs::Connection<T>,
    }

    impl<T> $name<T>
    where
      T: ::kvs::redis::Commands + Send + Sync,
    {
      pub fn new(con: ::kvs::Connection<T>) -> Self {
        return Self { con: con };
      }
    }

    impl<T> ::kvs::traits::normal::ChannelName for $name<T>
    where
      T: ::kvs::redis::Commands + Send + Sync,
    {
      fn channel_name(
        &self,
        key: impl AsRef<str> + ::std::fmt::Display
      ) -> String where
      {
        return format!($ch_name, key);
      }
    }

    impl<T> ::kvs::traits::normal::Base<T> for $name<T>
    where
      T: ::kvs::redis::Commands + Send + Sync,
    {
      fn commands(&self) -> ::tokio::sync::MutexGuard<T> {
        return self.con.clone();
      }
    }
    impl_trait_1!(::kvs::traits::normal::Lock, $name);
  };
}

#[macro_export]
macro_rules! symbol_kvs {
  ($acc: vis, $name: ident, $vtype: ty, $ch_name: expr) => {
    #[derive(Clone)]
    $acc struct $name<T>
    where
      T: ::kvs::redis::Commands + Send,
    {
      con: ::kvs::Connection<T>,
    }

    impl<T> $name<T>
    where
      T: ::kvs::redis::Commands + Send,
    {
      pub fn new(con: ::kvs::Connection<T>) -> Self {
        return Self { con: con };
      }
    }

    impl<T> ::kvs::traits::SymbolKeyStore<T, $vtype> for $name<T>
    where
      T: ::kvs::redis::Commands + Send,
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
      T: ::kvs::redis::Commands + Send
    {
    }
  };
}

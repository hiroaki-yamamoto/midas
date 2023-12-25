use ::std::sync::Arc;

use ::kvs::redis::aio::MultiplexedConnection;
use ::kvs::traits::symbol::Get;

pub type ProgressKVS =
  Arc<dyn Get<Commands = MultiplexedConnection, Value = i64> + Send + Sync>;

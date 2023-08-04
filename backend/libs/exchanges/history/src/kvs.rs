use std::fmt::Display;

use ::kvs::kvs_incr;
pub use ::redis;
use ::redis::Commands;

kvs_incr!(
  pub,
  CurrentSyncProgressStore,
  i64,
  "{}.{}.kline_sync.current"
);
kvs_incr!(pub, NumObjectsToFetchStore, i64, "{}.{}.kline_sync.num");

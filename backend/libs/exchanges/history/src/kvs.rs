use ::kvs_macros::symbol_kvs;

symbol_kvs!(
  pub,
  CurrentSyncProgressStore,
  i64,
  "{}:{}:kline_sync:current"
);
symbol_kvs!(pub, NumObjectsToFetchStore, i64, "{}:{}:kline_sync:num");
